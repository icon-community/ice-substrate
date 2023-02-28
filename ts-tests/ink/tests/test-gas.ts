import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import { ContractPromise } from "@polkadot/api-contract";
import BigNumber from "bignumber.js";
import { getMetadata, getWasm, SnowApi } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";

chai.use(chaiAsPromised);

const { expect } = chai;

const GAS_LIMIT = "100000000000"; // 10^11
const MAX_GAS_LIMIT = "1299000000000"; // 10^12
const DEPLOY_STORAGE_LIMIT = "10000000000000000000"; // 10^19

const UPLOAD_TIMEOUT = 30_000;
const FUND_TRANSFER_TIMEOUT = 30_000;
const QUERY_TIMEOUT = 30_000;

const END_USER_FUNDS = new BigNumber(1_000 * Math.pow(10, 18)); // 1k ICZ

const ACCUMULATOR_CODE_HASH = "0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9";

const ACCUMULATOR_INC_GAS = { refTime: 4006871040n, proofSize: 131072n };
const ADDER_INC_GAS = { refTime: 5397669461n, proofSize: 153804n };
const ADDER_DEPOSIT_GAS = { refTime: 4006871040n, proofSize: 131072n };

describeWithContext("\n\n👉 Estimate gas for deploying and calling write methods on contract", (context) => {
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

	step("🌟 Estimated gas for deploying accumulator contract should be accurate", async function (done) {
		try {
			console.log("\n\nUploading accumulator contract...\n");
			this.timeout(UPLOAD_TIMEOUT);

			// todo: estimate the gas required for deployment and then deploy with that gas limit
			// Params: origin, balance, gasLimit, storageDeposit, ctxWasm, data, salt
			// const res = context.api?.call.contractsApi.instantiate(context.alice!.address, "0", BigInt(Math.pow(10,18)), null, adderContract.wasm!, [0], [0]);

			const {
				address: ctxAddress,
				blockHash: ctxBlockHash,
				blockNum: ctxBlockNum,
			} = await context.deployContract(
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
				context.endUserWallets[0]!,
			);

			accumulatorContract.address = ctxAddress;
			accumulatorContract.blockHash = ctxBlockHash;
			accumulatorContract.blockNum = ctxBlockNum;
			accumulatorContract.codeHash = await SnowApi.getCodeHash(ctxAddress!);

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Estimated gas for deploying adder contract should accurate", async function (done) {
		try {
			console.log("\n\nUploading adder contract...\n");
			this.timeout(UPLOAD_TIMEOUT);

			// todo: estimate the gas required for deployment and then deploy with that gas limit

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

			adderContract.address = ctxAddress;
			adderContract.blockHash = ctxBlockHash;
			adderContract.blockNum = ctxBlockNum;

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Estimate gas limit for simple transaction", async function (done) {
		try {
			console.log("\n\nEstimating inc method call on accumulator contract...\n");
			this.timeout(QUERY_TIMEOUT);

			const ctxObj = new ContractPromise(
				context.api!,
				accumulatorContract.metadata!,
				accumulatorContract.address!,
			);

			const response = await context.dryRunTransaction(
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				context.alice!.address,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
			);
			expect(response?.result.gasLimit.proofSize.toBigInt()).eql(ACCUMULATOR_INC_GAS.proofSize);
			expect(response?.result.gasLimit.refTime.toBigInt()).eql(ACCUMULATOR_INC_GAS.refTime);

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Estimate gas limit for multicall transaction", async function (done) {
		try {
			console.log("\n\nEstimating inc method on adder contract...\n");
			this.timeout(QUERY_TIMEOUT);

			const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

			const response = await context.dryRunTransaction(
				ctxObj,
				CONTRACTS.multiCallCtx.adder.writeMethods.inc,
				context.endUserWallets[0]!.address,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
			);
			expect(response?.result.gasLimit.proofSize.toBigInt()).eql(ADDER_INC_GAS.proofSize);
			expect(response?.result.gasLimit.refTime.toBigInt()).eql(ADDER_INC_GAS.refTime);

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Estimate gas limit for payable transaction", async function (done) {
		try {
			console.log("\n\nEstimating receiveFunds method on adder contract...\n");
			this.timeout(QUERY_TIMEOUT);

			const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

			const response = await context.dryRunTransaction(
				ctxObj,
				CONTRACTS.multiCallCtx.adder.writeMethods.receiveFunds,
				context.endUserWallets[0]!.address,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
					value: Math.pow(10, 18).toString(),
				},
				[],
			);
			expect(response?.result.gasLimit.proofSize.toBigInt()).eql(ADDER_DEPOSIT_GAS.proofSize);
			expect(response?.result.gasLimit.refTime.toBigInt()).eql(ADDER_DEPOSIT_GAS.refTime);

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Tx exceeding block gas limit should fail", async function (done) {
		console.log("\n\nCalling expensive method on adder contract...\n");
		this.timeout(QUERY_TIMEOUT);

		const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

		expect(
			context.writeContract(
				context.alice!,
				ctxObj,
				CONTRACTS.multiCallCtx.adder.writeMethods.expensiveFunc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: MAX_GAS_LIMIT,
						refTime: MAX_GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[],
			),
		)
			.to.be.rejectedWith(/OutOfGas/, "Should fail due to exceeding gas limit")
			.notify(done);
	});
});
