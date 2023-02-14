// Instantiating contract before setting its code should fail
// uploading contract after having code hash on chain should succeed
// Removing code hash should refund the storage deposit to contract owner

import { step } from "mocha-steps";
import chai from "chai";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import chaiAsPromised from "chai-as-promised";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";
import { ContractPromise } from "@polkadot/api-contract";
import BigNumber from "bignumber.js";

chai.use(chaiAsPromised);

const { expect } = chai;

const GAS_LIMIT = "100000000000"; // 10^11
const DEPLOY_STORAGE_LIMIT = "10000000000000000000"; // 10^19
const MAX_RESIDUE = "20000000000000000"; // 10^16

const UPLOAD_TIMEOUT = 30_000;
const FUND_TRANSFER_TIMEOUT = 30_000;

const END_USER_FUNDS = new BigNumber(1_000 * Math.pow(10, 18)); // 1k ICZ

const TERMINATE_TX_FEE = new BigNumber(0.05).multipliedBy(Math.pow(10, 18));

const ACCUMULATOR_CODE_HASH = "0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9";

describeWithContext("\n\n👉 Tests for code hash", (context) => {
	const accumulatorContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		codeHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.multiCallCtx.accumulator.wasmPath),
		metadata: getMetadata(CONTRACTS.multiCallCtx.accumulator.metadataPath),
	};

	const adderContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.multiCallCtx.adder.wasmPath),
		metadata: getMetadata(CONTRACTS.multiCallCtx.adder.metadataPath),
	};

	before(async function () {
		this.timeout(FUND_TRANSFER_TIMEOUT);
		console.log("\n\nTransferring funds to end user wallets...\n");
		await context.fundEndUserWallets(END_USER_FUNDS);
	});

	step(
		"🌟 Instantiating adder contract before accumulator code hash is available on-chain should fail",
		async function (done) {
			this.timeout(UPLOAD_TIMEOUT);

			expect(
				context.deployContract(
					adderContract.metadata!,
					adderContract.wasm!,
					{
						gasLimit: context.api!.registry.createType("WeightV2", {
							proofSize: GAS_LIMIT,
							refTime: GAS_LIMIT,
						}) as WeightV2,
						storageDepositLimit: DEPLOY_STORAGE_LIMIT,
					},
					[0, 1, ACCUMULATOR_CODE_HASH],
					context.alice!,
				),
			)
				.to.be.rejectedWith(
					/ContractTrapped/,
					"Instantiating adder without accumulator code has on-chain should fail",
				)
				.notify(done);
		},
	);

	step(
		"🌟 Instantiating adder contract after accumulator code hash is available on-chain should succeed",
		async function (done) {
			try {
				this.timeout(UPLOAD_TIMEOUT);

				// deploy accumulator contract
				console.log("\n\nInstantiating accumulator contract...\n");
				await context.deployContract(
					accumulatorContract.metadata!,
					accumulatorContract.wasm!,
					{
						gasLimit: context.api!.registry.createType("WeightV2", {
							proofSize: GAS_LIMIT,
							refTime: GAS_LIMIT,
						}) as WeightV2,
						storageDepositLimit: DEPLOY_STORAGE_LIMIT,
					},
					[0],
					context.alice!,
				);

				// deploy adder contract
				console.log("\n\nInstantiating adder contract...\n");
				const {
					address: ctxAddress,
					blockHash: ctxBlockHash,
					blockNum: ctxBlockNum,
				} = await context.deployContract(
					adderContract.metadata!,
					adderContract.wasm!,
					{
						gasLimit: context.api!.registry.createType("WeightV2", {
							proofSize: GAS_LIMIT,
							refTime: GAS_LIMIT,
						}) as WeightV2,
						storageDepositLimit: DEPLOY_STORAGE_LIMIT,
					},
					[0, 1, ACCUMULATOR_CODE_HASH],
					context.endUserWallets[0]!,
				);

				const { blockNumber: lastBlockNum } = await context.getLastBlock();

				expect(ctxAddress).to.have.lengthOf(49);
				expect(ctxBlockNum).to.equal(lastBlockNum);

				adderContract.address = ctxAddress;
				adderContract.blockHash = ctxBlockHash;
				adderContract.blockNum = ctxBlockNum;

				done();
			} catch (err) {
				done(err);
			}
		},
	);

	step("🌟 Whoever removes the contract code hash should be refunded the contract deposit", async function (done) {
		try {
			this.timeout(UPLOAD_TIMEOUT);

			// get initial balance of deployer
			const initialBal = await context.getBalance(context.endUserWallets[1].address);

			// get balance of the contract
			const ctxBal = await context.getBalance(adderContract.address!, true);

			const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

			// terminate adder contract
			console.log("\n\nTerminating adder contract...\n");
			await context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.adder.writeMethods.tearDown,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[],
			);

			const finalBal = await context.getBalance(context.endUserWallets[1].address);

			// ensure deployer balance is refunded with ctx deposit
			expect(
				finalBal.minus(initialBal).plus(TERMINATE_TX_FEE).minus(ctxBal).toNumber(),
				"Contract funds not properly refunded",
			)
				.to.be.greaterThanOrEqual(0)
				.and.to.be.lessThanOrEqual(parseInt(MAX_RESIDUE));

			done();
		} catch (err) {
			done(err);
		}
	});
});
