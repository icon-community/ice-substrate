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

const GAS_LIMIT = "100000000000"; // 10^11
const DEPLOY_STORAGE_LIMIT = "40000000000000000000"; // 40 ICZ

const UPLOAD_TIMEOUT = 30_000;
const WRITE_TIMEOUT = 30_000;

export async function getCtxState(
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
				proofSize: GAS_LIMIT,
				refTime: GAS_LIMIT,
			}) as WeightV2,
			storageDepositLimit: null,
		},
	};

	// @ts-ignore
	const { output } = await queryMethod(ctxObj, CONTRACTS.simpleCtx.readMethods.get, queryOptions);

	return output?.toString();
}

describeWithContext("\n\n👉 Tests for keccak hash on contract", (context) => {
	const hashCtx: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		codeHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.keccakTestCtx.wasmPath),
		metadata: getMetadata(CONTRACTS.keccakTestCtx.metadataPath),
	};

	step(`🌟 Successfully upload contract`, async function (done) {
		try {
			// simply upload and get contract & block num. Ensure that the block was last produced block
			console.log("\n\nUploading a test contract...\n");
			this.timeout(UPLOAD_TIMEOUT);
			const {
				address: ctxAddress,
				blockHash: ctxBlockHash,
				blockNum: ctxBlockNum,
			} = await context.deployContract(
				hashCtx.metadata!,
				hashCtx.wasm!,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: DEPLOY_STORAGE_LIMIT,
				},
				[1000],
				context.alice!,
			);

			const { blockNumber: lastBlockNum } = await context.getLastBlock();

			expect(ctxAddress).to.have.lengthOf(49);
			expect(ctxBlockNum).to.equal(lastBlockNum);

			hashCtx.address = ctxAddress;
			hashCtx.blockHash = ctxBlockHash;
			hashCtx.blockNum = ctxBlockNum;

			done();
		} catch (err) {
			done(err);
		}
	});

	step("🌟 Successfully carry out hash operation on the contract", async function (done) {
		try {
			// call operate method
			console.log("\n\nCalling operate method on the test contract...\n");
			this.timeout(WRITE_TIMEOUT);

			const ctxObj = new ContractPromise(context.api!, hashCtx.metadata!, hashCtx.address!);

			await context.writeContract(
				context.alice!,
				ctxObj,
				CONTRACTS.keccakTestCtx.writeMethods.operate,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: DEPLOY_STORAGE_LIMIT,
				},
				[],
			);

			// ensure the values returned by get method are accurate
			expect(
				getCtxState(
					context.api!,
					hashCtx.metadata!,
					hashCtx.address!,
					context.endUserWallets[0]!.address,
					// @ts-ignore
					context.queryContract,
				),
			)
				.to.eventually.equal(
					// keccak hash of "3998"
					'{"hash":"0xba20efe605ffaf935740b0609b20e76f4a2eebc2a40e893d19665b3d829318a5","value":3998}',
					"Hashing method did not execute expectedly",
				)
				.notify(done);
		} catch (err) {
			done(err);
		}
	});
});
