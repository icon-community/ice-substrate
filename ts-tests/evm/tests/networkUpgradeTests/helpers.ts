import yargs from "yargs/yargs";
import { ethers } from "ethers";
import { CHAINS } from "../config";

export function parseChainFromArgs(args: typeof process.argv) {
    const chainName = yargs(args.slice(2)).parseSync().chain;
    // @ts-ignore
    if(!Object.keys(CHAINS).includes(chainName)) {
        throw new Error("Error: Supported chains are 'snow', 'arctic', 'snow_staging' and 'local'.")
    }
    // @ts-ignore
    const CHAIN: keyof typeof CHAINS = chainName;
    return CHAIN;
}

export function getEthersProvider(chain: keyof typeof CHAINS) {
	return new ethers.providers.StaticJsonRpcProvider(CHAINS[chain].RPC_ENDPOINT, {
		chainId: CHAINS[chain].CHAIN_ID,
		name: chain,
	});
}
