export const GENESIS_ACCOUNT = "0x8efcaf2c4ebbf88bf07f3bb44a2869c4c675ad7a";

export const GENESIS_ACCOUNT_PRIVATE_KEY = "0x3e2400cd858aa8d07c0c923e307fc1259ee5a8932d05f92f55be5d4589082542";
export const GENESIS_ACCOUNT_BALANCE = "340282366920938463463364607431768211455";

export const FIRST_CONTRACT_ADDRESS = "0xc2bf5f29a4384b1ab0c063e1c666f02121b6084a";

export const NODE_BINARY_NAME = "ice-node";

export const RUNTIME_SPEC_NAME = "frost-testnet";
export const RUNTIME_SPEC_VERSION = 1;
export const RUNTIME_IMPL_VERSION = 1;

export const CHAIN_ID = 554;
export const BLOCK_TIMESTAMP = 6; // 6 seconds per block
export const BLOCK_HASH_COUNT = 256;
export const EXISTENTIAL_DEPOSIT = 10_000_000_000_000_000; // The minimum amount required to keep an account open
export const BLOCK_GAS_LIMIT = 60000000;

export const CHAINS = {
    snow: {
        RPC_ENDPOINT: "https://snow-rpc.icenetwork.io:9933",
        CHAIN_ID: 552,
        UPGRADE_CTX_ADDRESS: "0x3f71f31caf936b5b95fe1f207498fe798bd7723b",
    },
    arctic: {
        RPC_ENDPOINT: "https://arctic-rpc.icenetwork.io:9933",
        CHAIN_ID: 553,
        UPGRADE_CTX_ADDRESS: "0x41897dad572342942d8e86572dd579cf3da16526",
    },
    snow_staging: {
        RPC_ENDPOINT: "https://snow-staging-rpc.web3labs.com:9933",
        CHAIN_ID: 552,
        UPGRADE_CTX_ADDRESS: "0x5c1c61cb2d66bb77ae2df436cb5e3f92db065a3c",
    },
    local: {
        RPC_ENDPOINT: "https://localhost:9933",
        CHAIN_ID: 554,
        UPGRADE_CTX_ADDRESS: "", // Update according to local env
    },
}
