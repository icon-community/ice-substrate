import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import dotenv from "dotenv-flow";
import { CONTRACTS } from "../../constants";
import { getMetadata, getWasm, SnowApi } from "../../services";
import { parseChainFromArgs } from "./helpers";

dotenv.config();

const DEPLOY_GAS_LIMIT = "1000000000000";
const DEPLOY_STORAGE_LIMIT = "40000000000000000000"; // 40 ICZ

const STATE_TEST_CTX_METADATA = getMetadata(CONTRACTS.stateTestCtx.metadataPath);
const STATE_TEST_CTX_WASM = getWasm(CONTRACTS.stateTestCtx.wasmPath);

const WALLET_URI = process.env["INK_CTX_DEPLOYER_URI"];
const chain = parseChainFromArgs(process.argv);

async function deployMigrationCtx() {
	await SnowApi.initialize(chain);

	const wallet = SnowApi.keyring?.addFromUri(WALLET_URI!);

	console.log(`Deploying on ${chain} with wallet ${wallet!.address}`);

	const { address, blockNum } = await SnowApi.deployContract(
		STATE_TEST_CTX_METADATA,
		STATE_TEST_CTX_WASM,
		{
			gasLimit: SnowApi.api!.registry.createType("WeightV2", {
				proofSize: DEPLOY_GAS_LIMIT,
				refTime: DEPLOY_GAS_LIMIT,
			}) as WeightV2,
			storageDepositLimit: DEPLOY_STORAGE_LIMIT,
		},
		[SnowApi.endUserWallets[0].address, "0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9"],
		wallet!,
	);

	console.log(`\n\nDeployment successful\nContract: ${address}\nBlock Number: ${blockNum}`);

	SnowApi.cleanUp();
}

deployMigrationCtx();
