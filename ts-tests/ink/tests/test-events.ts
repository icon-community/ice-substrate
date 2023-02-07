// events emitted by ink contracts can be parsed
// failed transactions throw proper error message on dry run

import { step } from "mocha-steps";
import chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { getMetadata, getWasm } from "../services";
import { describeWithContext } from "./utils";
import { CONTRACTS } from "../constants";
import { ContractInterface } from "../interfaces/core";
import { ContractPromise } from "@polkadot/api-contract";

chai.use(chaiAsPromised);

const { expect } = chai;

const GAS_LIMIT = "100000000000"; // 10^11
const DEPLOY_STORAGE_LIMIT = "10000000000000000000"; // 10^19

const UPLOAD_TIMEOUT = 30_000; // todo
const TX_TIMEOUT = 30_000; // todo

describeWithContext("\n\n👉 Test events emitted by contracts can be parsed", (context) => {
	const flipperContract: ContractInterface = {
		address: undefined,
		blockHash: undefined,
		blockNum: undefined,
		wasm: getWasm(CONTRACTS.simpleCtx.wasmPath),
		metadata: getMetadata(CONTRACTS.simpleCtx.metadataPath),
	};

	// deploy flipper contract
	before(async function () {
		this.timeout(UPLOAD_TIMEOUT);
		console.log("\n\nDeploying flipper contract...\n");

		const {
			address: ctxAddress,
			blockHash: ctxBlockHash,
			blockNum: ctxBlockNum,
		} = await context.deployContract(
			flipperContract.metadata!,
			flipperContract.wasm!,
			{ gasLimit: GAS_LIMIT, storageDepositLimit: DEPLOY_STORAGE_LIMIT },
			[0],
			context.alice!,
		);

		flipperContract.address = ctxAddress;
		flipperContract.blockHash = ctxBlockHash;
		flipperContract.blockNum = ctxBlockNum;
	});

	step("Verify event emitted by a transaction", async function (done) {
		this.timeout(TX_TIMEOUT);
		console.log("\n\nCalling flip on flipper contract...");

		const ctxObj = new ContractPromise(context.api!, flipperContract.metadata!, flipperContract.address!);

		const { events } = await context.writeContract(
			context.alice!,
			ctxObj,
			CONTRACTS.simpleCtx.writeMethods.flip,
			{ gasLimit: GAS_LIMIT, storageDepositLimit: DEPLOY_STORAGE_LIMIT },
			[],
		);

		console.log("\n\nTracking emitted event...");

		const emittedEvent = events?.find(({ event }) => event.method === "ContractEmitted");

		expect(emittedEvent?.event.data[1].toHex()).to.equal("0x0001");

		done();
	});
});
