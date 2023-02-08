import { ethers } from "ethers";

import { SNOW_CHAIN_ID, SNOW_RPC_ENDPOINT } from "../config";

export const ethersProvider = new ethers.providers.StaticJsonRpcProvider(SNOW_RPC_ENDPOINT, {
	chainId: SNOW_CHAIN_ID,
	name: "snow-mainnet",
});
