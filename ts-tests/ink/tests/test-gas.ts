import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { getMetadata, getWasm, SnowApi } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";
import { ContractPromise } from "@polkadot/api-contract";
import BigNumber from "bignumber.js";

chai.use(chaiAsPromised);

const { expect } = chai;

// const GAS_LIMIT = "3276940880";
const GAS_LIMIT = "100000000000"; // 10^11
const MAX_GAS_LIMIT = "1299000000000"; // 10^12
const DEPLOY_STORAGE_LIMIT = "10000000000000000000"; // 10^19

const UPLOAD_TIMEOUT = 30_000; // todo
const FUND_TRANSFER_TIMEOUT = 30_000; // todo
const QUERY_TIMEOUT = 30_000; // todo

const END_USER_FUNDS = new BigNumber(1_000 * Math.pow(10, 18)); // 1k ICZ

const ACCUMULATOR_INC_GAS = "37499961344";
const ACCUMULATOR_CODE_HASH = "0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9";
const ADDER_INC_GAS = "44492876186";
const ADDER_DEPOSIT_GAS = "37499961344";

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
			{ gasLimit: GAS_LIMIT, storageDepositLimit: DEPLOY_STORAGE_LIMIT },
			[0],
			context.endUserWallets[0]!,
		);

		accumulatorContract.address = ctxAddress;
		accumulatorContract.blockHash = ctxBlockHash;
		accumulatorContract.blockNum = ctxBlockNum;
		accumulatorContract.codeHash = await SnowApi.getCodeHash(ctxAddress!);

		done();
	});

	step("🌟 Estimated gas for deploying adder contract should accurate", async function (done) {
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
			{ gasLimit: GAS_LIMIT, storageDepositLimit: DEPLOY_STORAGE_LIMIT },
			[0, 1, ACCUMULATOR_CODE_HASH],
			context.endUserWallets[0]!,
		);

		adderContract.address = ctxAddress;
		adderContract.blockHash = ctxBlockHash;
		adderContract.blockNum = ctxBlockNum;

		done();
	});

	step("🌟 Estimate gas limit for simple transaction", async function (done) {
		console.log("\n\nEstimating inc method call on accumulator contract...\n");
		this.timeout(QUERY_TIMEOUT);

		const ctxObj = new ContractPromise(context.api!, accumulatorContract.metadata!, accumulatorContract.address!);

		const response = await context.dryRunTransaction(
			ctxObj,
			CONTRACTS.multiCallCtx.accumulator.writeMethods.inc,
			context.alice!.address,
			{ gasLimit: GAS_LIMIT, storageDepositLimit: null },
			[1],
		);
		expect(response?.result.gasLimit.toFixed(0)).equal(ACCUMULATOR_INC_GAS);

		done();
	});

	step("🌟 Estimate gas limit for multicall transaction", async function (done) {
		console.log("\n\nEstimating inc method on adder contract...\n");
		this.timeout(QUERY_TIMEOUT);

		const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

		const response = await context.dryRunTransaction(
			ctxObj,
			CONTRACTS.multiCallCtx.adder.writeMethods.inc,
			context.endUserWallets[0]!.address,
			{ gasLimit: GAS_LIMIT, storageDepositLimit: null },
			[1],
		);
		expect(response?.result.gasLimit.toFixed(0)).equal(ADDER_INC_GAS);

		done();
	});

	step("🌟 Estimate gas limit for payable transaction", async function (done) {
		console.log("\n\nEstimating receiveFunds method on adder contract...\n");
		this.timeout(QUERY_TIMEOUT);

		const ctxObj = new ContractPromise(context.api!, adderContract.metadata!, adderContract.address!);

		const response = await context.dryRunTransaction(
			ctxObj,
			CONTRACTS.multiCallCtx.adder.writeMethods.receiveFunds,
			context.endUserWallets[0]!.address,
			{ gasLimit: GAS_LIMIT, storageDepositLimit: null, value: Math.pow(10, 18).toString() },
			[],
		);
		expect(response?.result.gasLimit.toFixed(0)).equal(ADDER_DEPOSIT_GAS);

		done();
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
				{ gasLimit: MAX_GAS_LIMIT, storageDepositLimit: null },
				[],
			),
		)
			.to.be.rejectedWith(/OutOfGas/, "Should fail due to exceeding gas limit")
			.notify(done);
	});
});
