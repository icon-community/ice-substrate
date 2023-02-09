import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";

chai.use(chaiAsPromised);

const { expect } = chai;

const MAX_GAS_LIMIT = "1200000000000";
const DEPLOY_STORAGE_LIMIT = "10000000000000000000";

const UPLOAD_TIMEOUT = 30_000;

describeWithContext("\n\n👉 Tests for contract size", (context) => {
	const largeValidContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.largeCtx.valid.wasmPath),
		metadata: getMetadata(CONTRACTS.largeCtx.valid.metadataPath),
	};

	const largeInvalidContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.largeCtx.invalid.wasmPath),
		metadata: getMetadata(CONTRACTS.largeCtx.invalid.metadataPath),
	};

	step("🌟 Contract size above 128KB should not be deployed", async function (done) {
		console.log("\n\nUploading a large invalid contract...\n");
		this.timeout(UPLOAD_TIMEOUT);

		expect(
			context.deployContract(
				largeInvalidContract.metadata!,
				largeInvalidContract.wasm!,
				{
					gasLimit: context.api!.registry.createType("WeightV2", {
						proofSize: MAX_GAS_LIMIT,
						refTime: MAX_GAS_LIMIT,
					}) as WeightV2,
					storageDepositLimit: DEPLOY_STORAGE_LIMIT,
				},
				[1],
				context.alice!,
			),
		)
			.to.be.rejectedWith(/CodeTooLarge/, "Contract should not be instantiated")
			.notify(done);
	});

	step("🌟 Contract size just below 128KB should be deployed", async function (done) {
		console.log("\n\nUploading a large valid contract...\n");
		this.timeout(UPLOAD_TIMEOUT);

		const { address } = await context.deployContract(
			largeValidContract.metadata!,
			largeValidContract.wasm!,
			{
				gasLimit: context.api!.registry.createType("WeightV2", {
					proofSize: MAX_GAS_LIMIT,
					refTime: MAX_GAS_LIMIT,
				}) as WeightV2,
				storageDepositLimit: DEPLOY_STORAGE_LIMIT,
			},
			[1],
			context.alice!,
		);

		expect(address).to.have.lengthOf(49);
		done();
	});
});
