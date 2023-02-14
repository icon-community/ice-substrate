import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import { ContractPromise } from "@polkadot/api-contract";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { AnyJson } from "@polkadot/types-codec/types";
import { getMetadata, getWasm } from "../../services";
import { describeWithContext } from "../utils";
import { CONTRACTS } from "../../constants";
import { ContractInterface, QueryArgs } from "../../interfaces/core";
import { parseChainFromArgs } from "./helpers";

chai.use(chaiAsPromised);

const { expect } = chai;

const GAS_LIMIT = "100000000000"; // 10^11
const DEPLOY_STORAGE_LIMIT = "40000000000000000000"; // 40 ICZ

const UPLOAD_TIMEOUT = 30_000;
const WRITE_TIMEOUT = 30_000;

const WALLET_URI = process.env["INK_CTX_DEPLOYER_URI"];
const chain = parseChainFromArgs(process.argv);

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

describeWithContext(
	"\n\n👉 Tests for contracts after network upgrade",
	(context) => {
		const migrationCtx: ContractInterface = {
			address: undefined,
			blockHash: undefined,
			codeHash: undefined,
			blockNum: undefined,
			wasm: getWasm(CONTRACTS.simpleCtx.wasmPath),
			metadata: getMetadata(CONTRACTS.simpleCtx.metadataPath),
		};

		let wallet: KeyringPair | undefined;

		step(`🌟 Successfully upload contract to ${chain} network`, async function (done) {
			wallet = context.keyring!.addFromUri(WALLET_URI!);

			// simply upload and get contract & block num. Ensure that the block was last produced block
			console.log("\n\nUploading a test contract...\n");
			this.timeout(UPLOAD_TIMEOUT);
			const {
				address: ctxAddress,
				blockHash: ctxBlockHash,
				blockNum: ctxBlockNum,
			} = await context.deployContract(
				migrationCtx.metadata!,
				migrationCtx.wasm!,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: GAS_LIMIT,
						refTime: GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: DEPLOY_STORAGE_LIMIT,
				},
				[false],
				wallet,
			);

			const { blockNumber: lastBlockNum } = await context.getLastBlock();

			expect(ctxAddress).to.have.lengthOf(49);
			expect(ctxBlockNum).to.equal(lastBlockNum);

			migrationCtx.address = ctxAddress;
			migrationCtx.blockHash = ctxBlockHash;
			migrationCtx.blockNum = ctxBlockNum;

			done();
		});

		step("🌟 Successfully perform write operations on the contract", async function (done) {
			try {
				// call operate method
				console.log("\n\nCalling write method on the test contract...\n");
				this.timeout(WRITE_TIMEOUT);

				const ctxObj = new ContractPromise(context.api!, migrationCtx.metadata!, migrationCtx.address!);

				await context.writeContract(
					wallet!,
					ctxObj,
					CONTRACTS.simpleCtx.writeMethods.flip,
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
						migrationCtx.metadata!,
						migrationCtx.address!,
						wallet!.address,
						// @ts-ignore
						context.queryContract,
					),
				)
					.to.eventually.equal(
						"true",
						"Write method did not execute expectedly",
					)
					.notify(done);
			} catch (err) {
				done(err);
			}
		});
	},
	chain,
);
