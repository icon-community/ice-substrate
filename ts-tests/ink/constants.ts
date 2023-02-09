export const BINARY_PATH = "./assets/ice-node";

export const LOCAL_WSS_URL = "ws://localhost:9944";
export const MAINNET_WSS_URL = "wss://snow-rpc.icenetwork.io";

export const MAINNET_CTX_ADDRESS = "ni3hcgLz19VPerw8PmdZFiApgeU4WnXwJ1sdDBQDRhp3imXbs";
export const MAINNET_CHAIN_PREFIX = 2207;

export const KEYRING_TYPE = "sr25519";
export const CHAIN_PREFIX = 2208;

export const BLOCK_TIME_MS = 12_000;

export const ALICE_URI = "//Alice";

export const CONTRACTS = {
	simpleCtx: {
		name: "flipper",
		metadataPath: "./assets/simpleCtx/flipper.contract",
		wasmPath: "./assets/simpleCtx/flipper.wasm",
		readMethods: {
			get: "get",
		},
		writeMethods: {
			flip: "flip",
		},
	},
	multiCallCtx: {
		adder: {
			name: "adder",
			metadataPath: "./assets/multiCallCtx/adder/adder.contract",
			wasmPath: "./assets/multiCallCtx/adder/adder.wasm",
			readMethods: {},
			writeMethods: {
				inc: "inc",
				expensiveFunc: "expensiveFunc",
				receiveFunds: "receiveFunds",
				tearDown: "tearDown",
			},
		},
		accumulator: {
			name: "accumulator",
			metadataPath: "./assets/multiCallCtx/accumulator/accumulator.contract",
			wasmPath: "./assets/multiCallCtx/accumulator/accumulator.wasm",
			readMethods: {
				get: "get",
			},
			writeMethods: {
				inc: "inc",
			},
		},
	},
	largeCtx: {
		invalid: {
			metadataPath: "./assets/largeCtx/invalid/snow_rewards.contract",
			wasmPath: "./assets/largeCtx/invalid/snow_rewards.wasm",
		},
		valid: {
			metadataPath: "./assets/largeCtx/valid/snow_rewards.contract",
			wasmPath: "./assets/largeCtx/valid/snow_rewards.wasm",
		},
	},
	migrationTestCtx: {
		readMethods: {
			get: "get",
		},
		writeMethods: {
			operate: "operate",
		},
		metadataPath: "./assets/migrationTestCtx/migration_test.contract",
		wasmPath: "./assets/migrationTestCtx/migration_test.wasm",
	},
	stateCheckCtx: {
		readMethods: {
			get: "get",
		},
		metadataPath: "./assets/stateCheckCtx/state_check.contract",
		wasmPath: "./assets/stateCheckCtx/state_check.wasm",
	},
};
