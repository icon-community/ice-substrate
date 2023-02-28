import yargs from "yargs/yargs";
import { CHAINS } from "../../constants";

export function parseChainFromArgs(args: typeof process.argv) {
	const chainName = yargs(args.slice(2)).parseSync().chain;
	// @ts-ignore
	if (!Object.keys(CHAINS).includes(chainName)) {
		throw new Error("Error: Supported chains are 'snow', 'arctic', 'snow_staging' and 'local'.");
	}
	// @ts-ignore
	const CHAIN: keyof typeof CHAINS = chainName;
	return CHAIN;
}
