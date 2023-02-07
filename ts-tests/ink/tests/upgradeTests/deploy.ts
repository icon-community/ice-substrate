import { CONTRACTS } from "../../constants";
import { getMetadata, getWasm, SnowApi } from "../../services";

const DEPLOY_GAS_LIMIT = "1000000000000";
const DEPLOY_STORAGE_LIMIT = "40000000000000000000"; // 40 ICZ

const STATE_CHECK_CTX_METADATA = getMetadata(CONTRACTS.stateCheckCtx.metadataPath);
const STATE_CHECK_CTX_WASM = getWasm(CONTRACTS.stateCheckCtx.wasmPath);

const WALLET_URI = process.env["MAINNET_WALLET_URI"];

async function deployMigrationCtx() {
	await SnowApi.initialize(true);

	const wallet = SnowApi.keyring?.addFromUri(WALLET_URI!);

	console.log(`Deploying on mainnet with wallet ${wallet?.address}`);

	const { address, blockNum } = await SnowApi.deployContract(
		STATE_CHECK_CTX_METADATA,
		STATE_CHECK_CTX_WASM,
		{ gasLimit: DEPLOY_GAS_LIMIT, storageDepositLimit: DEPLOY_STORAGE_LIMIT },
		["SNOW", 100],
		wallet!,
	);

	console.log(`\n\nDeployment successful\nContract: ${address}\nBlock Number: ${blockNum}`);

	SnowApi.cleanUp();
}

deployMigrationCtx();
