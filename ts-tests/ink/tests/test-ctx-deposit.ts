/*
Uploading a flipper contract
Deployment tx fee - 1.75 ICZ
Balance transfer from deployer to contract - 1.00405 ICZ
Above amount is reserved in the contract
Balance transfer from deployer to contract - 1.00005 ICZ
Above amount is reserved in the contract

Uploading flipper contract with two contract values:
Deployment tx fee - 1.7999 ICZ
Balance transfer from deployer to contract - 1.00405 ICZ
Above amount is reserved in the contract
Balance transfer from deployer to contract - 2.00085 ICZ
Above amount is reserved in the contract
Amount reserved on the deployer wallet - 4.878 ICZ

Uploading flipper contract with three contract values:
Deployment tx fee - 1.845 ICZ
Balance transfer from deployer to contract - 1.00405 ICZ
Above amount is reserved in the contract
Balance transfer from deployer to contract - 3.00105 ICZ
Above amount is reserved in the contract
Amount reserved on the deployer wallet - 4.919 ICZ

NOTE: Initial deployment of code hash will reserve some balances on the deployer wallet itself.
*/

import { step } from "mocha-steps";
import chai from "chai";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import BigNumber from "bignumber.js";
import chaiAsPromised from "chai-as-promised";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";

chai.use(chaiAsPromised);
const { expect } = chai;

const END_USER_FUNDS = new BigNumber(1_000 * Math.pow(10, 18)); // 1k ICZ;
const DEPLOYER_RESERVE_BAL = "4825600000000000000";
const CTX_RESERVE_BAL = "2006100000000000000";

const DEPLOY_GAS_LIMIT = "600000000000";
const DEPLOY_STORAGE_LIMIT = "10000000000000000000";
const MIN_DEPLOY_STORAGE_LIMIT = "2000000000000000000";

const UPLOAD_TIMEOUT = 30_000;
const FUND_TRANSFER_TIMEOUT = 30_000;

describeWithContext("\n\n👉 Test reserved balances on deployer wallet and a simple contract", (context) => {
	const simpleContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.simpleCtx.wasmPath),
		metadata: getMetadata(CONTRACTS.simpleCtx.metadataPath),
	};

	before(async function () {
		this.timeout(FUND_TRANSFER_TIMEOUT);
		console.log("\n\nTransferring funds to end user wallets...\n");
		await context.fundEndUserWallets(END_USER_FUNDS);
	});

	step("🌟 Uploading a contract with low storage deposit limit should fail", async function (done) {
		console.log("\n\nUploading a simple contract...\n");
		this.timeout(UPLOAD_TIMEOUT);

		expect(
			context.deployContract(
				simpleContract.metadata!,
				simpleContract.wasm!,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: DEPLOY_GAS_LIMIT,
						refTime: DEPLOY_GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: MIN_DEPLOY_STORAGE_LIMIT,
				},
				[false],
				context.endUserWallets[0]!,
			),
		)
			.to.be.rejectedWith(
				/StorageDepositLimitExhausted/,
				"Contract should not be deployed with less than required Storage Deposit Limit",
			)
			.notify(done);
	});

	step("🌟 Uploading a contract the first time should reserve balance on the deployer", async function (done) {
		console.log("\n\nUploading a simple contract...\n");
		this.timeout(UPLOAD_TIMEOUT);

		const { address: ctxAddress } = await context.deployContract(
			simpleContract.metadata!,
			simpleContract.wasm!,
			{
				gasLimit: context.api!.registry.createType("WeightV2", {
					proofSize: DEPLOY_GAS_LIMIT,
					refTime: DEPLOY_GAS_LIMIT,
				}) as WeightV2,
				storageDepositLimit: DEPLOY_STORAGE_LIMIT,
			},
			[false],
			context.endUserWallets[0]!,
		);

		const deployerReserveBal = await context.getBalance(context.endUserWallets[0].address, true);
		const ctxReserveBal = await context.getBalance(ctxAddress!, true);

		expect(deployerReserveBal.toFixed(0)).to.equal(
			DEPLOYER_RESERVE_BAL,
			"Balance should be reserved on the deployer",
		);
		expect(ctxReserveBal.toFixed(0)).to.equal(CTX_RESERVE_BAL, "Balance should be reserved on the contract");

		done();
	});

	step(
		"🌟 Deploying an already available code hash should not reserve balance on the deployer",
		async function (done) {
			console.log("\n\nUploading a simple contract...\n");
			this.timeout(UPLOAD_TIMEOUT);

			const { address: ctxAddress } = await context.deployContract(
				simpleContract.metadata!,
				simpleContract.wasm!,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: DEPLOY_GAS_LIMIT,
						refTime: DEPLOY_GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: DEPLOY_STORAGE_LIMIT,
				},
				[false],
				context.endUserWallets[1]!,
			);

			const deployerReserveBal = await context.getBalance(context.endUserWallets[1].address, true);
			const ctxReserveBal = await context.getBalance(ctxAddress!, true);

			expect(deployerReserveBal.toFixed(0)).to.equal("0", "Balance should not be reserved on the deployer");
			expect(ctxReserveBal.toFixed(0)).to.equal(CTX_RESERVE_BAL, "Balance should be reserved on the contract");

			done();
		},
	);
});
