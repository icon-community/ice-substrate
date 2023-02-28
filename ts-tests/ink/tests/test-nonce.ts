// Nonce should be updated after every contract call
// Future transaction with nonce less than previous transaction should take precedence over the one with higher nonce
// transaction with gas/depositLimit less than required should be replaced by future transaction with higher gas/depositLimit

import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import { ContractPromise } from "@polkadot/api-contract";
import BigNumber from "bignumber.js";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";

chai.use(chaiAsPromised);

const { expect } = chai;

// const GAS_LIMIT = "3276940880";
const GAS_LIMIT = "100000000000"; // 10^11
const DEPLOY_STORAGE_LIMIT = "10000000000000000000"; // 10^19

const BEFORE_TIMEOUT = 40_000;
const TX_TIMEOUT = 30_000;

const END_USER_FUNDS = new BigNumber(1_000 * Math.pow(10, 18)); // 1k ICZ

const ACCUMULATOR_CODE_HASH = "0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9";

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
		this.timeout(BEFORE_TIMEOUT);
		console.log("\n\nTransferring funds to end user wallets...\n");
		await context.fundEndUserWallets(END_USER_FUNDS);

		console.log("\n\nUploading accumulator contract...\n");
		const { address } = await context.deployContract(
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

		accumulatorContract.address = address;

		console.log("\n\nUploading adder contract...\n");
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
	});

	step("🌟 User nonce should update after it makes a write call on a contract", async function (done) {
		try {
			this.timeout(TX_TIMEOUT);
			console.log("\n\nCalling increment on accumulator contract...\n");

			const ctxObj = new ContractPromise(
				context.api!,
				accumulatorContract.metadata!,
				accumulatorContract.address!,
			);

			await context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
			);

			const userNonce = await context.getNonce(context.endUserWallets[1].address);

			expect(userNonce).to.equal(1, "User nonce didn't increase");
			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 User nonce should update on a multi-call transaction but not the contract nonce", async function (done) {
		try {
			this.timeout(TX_TIMEOUT);

			const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

			console.log("\n\nCalling inc method on adder contract...");
			await context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.adder.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
			);

			const adderNonce = await context.getNonce(adderContract.address!);
			const finalUserNonce = await context.getNonce(context.endUserWallets[1].address);

			expect(adderNonce).to.equal(0, "Adder contract nonce should not increase");
			expect(finalUserNonce).to.equal(2, "User nonce should increase");
			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Transaction with same nonce but higher tip should replace original transaction", async function (done) {
		try {
			this.timeout(TX_TIMEOUT);
			console.log("\n\nCalling inc method on accumulator contract...");

			const ctxObj = new ContractPromise(
				context.api!,
				accumulatorContract.metadata!,
				accumulatorContract.address!,
			);

			context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
			);

			await context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[120],
				"10000000000",
			);

			const { output } = await context.queryContract(ctxObj, CONTRACTS.multiCallCtx.accumulator.readMethods.get, {
				sender: context.endUserWallets[1].address,
				args: [],
				txOptions: {
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
			});

			expect(output?.toHuman()).to.equal("121");

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Transaction with lower nonce should be given priority", async function (done) {
		try {
			this.timeout(TX_TIMEOUT);
			console.log("\n\nCalling inc method on accumulator contract...");

			const ctxObj = new ContractPromise(
				context.api!,
				accumulatorContract.metadata!,
				accumulatorContract.address!,
			);

			const userNonce = await context.getNonce(context.endUserWallets[1].address);

			context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[1],
				undefined,
				userNonce + 2,
			);

			await context.writeContract(
				context.endUserWallets[1]!,
				ctxObj,
				CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
				[120],
				undefined,
				userNonce,
			);

			const { output } = await context.queryContract(ctxObj, CONTRACTS.multiCallCtx.accumulator.readMethods.get, {
				sender: context.endUserWallets[1].address,
				args: [],
				txOptions: {
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: null,
				},
			});

			expect(output?.toHuman()).to.equal("241");

			done();
		} catch (err) {
			done(err);
		}
	});
});
