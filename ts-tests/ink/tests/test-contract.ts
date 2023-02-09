// upload a ctx
// call read methods
// call write methods
// ensure estimated depositLimit and fees are accurate while deploying and calling transactions

import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import { ContractPromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
import { AnyJson } from "@polkadot/types-codec/types";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface, QueryArgs } from "../interfaces/core";

chai.use(chaiAsPromised);

const { expect } = chai;

const DEPLOY_GAS_LIMIT = "1000000000000";
const DEPLOY_STORAGE_LIMIT = "10000000000000000000";
const WRITE_GAS_LIMIT = "1000000000000";

const UPLOAD_TIMEOUT = 30_000;
const QUERY_TIMEOUT = 30_000;
const WRITE_TIMEOUT = 30_000;

export async function getFlipState(
	api: ApiPromise,
	ctxMetadata: string,
	ctxAddress: string,
	callerAddress: string,
	queryMethod: (arg1: ContractPromise, arg2: string, arg3: object) => Promise<unknown>,
): Promise<AnyJson> {
	const ctxObj = new ContractPromise(api, ctxMetadata, ctxAddress);

	const queryOptions: QueryArgs = {
		sender: callerAddress,
		args: [],
		txOptions: {
			gasLimit: api.registry.createType("WeightV2", {
				proofSize: WRITE_GAS_LIMIT,
				refTime: WRITE_GAS_LIMIT,
			}) as WeightV2,
			storageDepositLimit: null,
		},
	};

	// @ts-ignore
	const { output } = await queryMethod(ctxObj, CONTRACTS.simpleCtx.readMethods.get, queryOptions);

	return output?.toHuman();
}

describeWithContext("\n\n👉 Upload and perform read, write on a simple contract", (context) => {
	const simpleContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.simpleCtx.wasmPath),
		metadata: getMetadata(CONTRACTS.simpleCtx.metadataPath),
	};

	step("🌟 Uploading a simple contract should give contract address and blockHash", async function (done) {
		console.log("\n\nUploading a simple contract...\n");
		this.timeout(UPLOAD_TIMEOUT);

		const {
			address: ctxAddress,
			blockHash: ctxBlockHash,
			blockNum: ctxBlockNum,
		} = await context.deployContract(
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
			context.alice!,
		);

		const { blockNumber: lastBlockNum } = await context.getLastBlock();

		expect(ctxAddress).to.have.lengthOf(49);
		expect(ctxBlockNum).to.equal(lastBlockNum);

		simpleContract.address = ctxAddress;
		simpleContract.blockHash = ctxBlockHash;
		simpleContract.blockNum = ctxBlockNum;

		done();
	});

	step("🌟 Querying read method on simple contract should yield valid result", async function (done) {
		console.log("\n\nQuerying method on the deployed simple contract...\n");
		this.timeout(QUERY_TIMEOUT);

		const flipState = await getFlipState(
			context.api!,
			simpleContract.metadata!,
			simpleContract.address!,
			context.endUserWallets[0]?.address,
			// @ts-ignore
			context.queryContract,
		);

		expect(flipState).to.equal(
			false,
			`Invalid value received from ${CONTRACTS.simpleCtx.name}: ${simpleContract.address}`,
		);
		done();
	});

	step("🌟 Calling a write-contract method should update the simple contract's state", async function (done) {
		console.log("\n\nCalling write method on flipper contract...\n");
		this.timeout(WRITE_TIMEOUT);

		const ctxObj = new ContractPromise(context.api!, simpleContract.metadata!, simpleContract.address!);

		await context.writeContract(
			context.alice!,
			ctxObj,
			CONTRACTS.simpleCtx.writeMethods.flip,
			{
				gasLimit: context.api!.registry.createType("WeightV2", {
					proofSize: WRITE_GAS_LIMIT,
					refTime: WRITE_GAS_LIMIT,
				}) as WeightV2,
				storageDepositLimit: DEPLOY_STORAGE_LIMIT,
			},
			[],
		);

		expect(
			getFlipState(
				context.api!,
				simpleContract.metadata!,
				simpleContract.address!,
				context.endUserWallets[0]?.address,
				// @ts-ignore
				context.queryContract,
			),
		)
			.to.eventually.equal(true, "Could not update simple contract state")
			.notify(done);
	});
});
