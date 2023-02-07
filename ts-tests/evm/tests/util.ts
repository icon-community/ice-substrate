import Web3 from "web3";
import { ethers } from "ethers";
import { JsonRpcResponse } from "web3-core-helpers";
import { spawn, ChildProcess } from "child_process";
import { NODE_BINARY_NAME, CHAIN_ID } from "./config";

export const PORT = 19931;
export const RPC_PORT = 9933;
export const WS_PORT = 9944;

export const DISPLAY_LOG = process.env.ICE_LOG || false;
export const ICE_LOG = process.env.ICE_LOG || "info";
export const ICE_BUILD = process.env.ICE_BUILD || "release";

export const BINARY_PATH = `../target/${ICE_BUILD}/${NODE_BINARY_NAME}`;
export const SPAWNING_TIME = 60000;

export async function customRequest(web3: Web3, method: string, params: any[]): Promise<JsonRpcResponse> {
	return new Promise<JsonRpcResponse>((resolve, reject) => {
		(web3.currentProvider as any).send(
			{
				jsonrpc: "2.0",
				id: 1,
				method,
				params,
			},
			(error: Error | null, result?: JsonRpcResponse) => {
				if (error) {
					reject(
						`Failed to send custom request (${method} (${params.join(",")})): ${
							error.message || error.toString()
						}`
					);
				}
				resolve(result);
			}
		);
	});
}

export async function customRequestEther(ethersjs: ethers.providers.JsonRpcProvider, params: string) {
	try {
		const tx = await ethersjs.sendTransaction(params);
		return await tx.wait();
	} catch (e) {
		console.log(e);
	}
}

//wait for transaction to finish
export async function isTransactionFinalized(ethersjs: ethers.providers.JsonRpcProvider, txHash: string) {
	return ethersjs.waitForTransaction(txHash);
}

// Create a block and finalize it.
// It will include all previously executed transactions since the last finalized block.
export async function createAndFinalizeBlock(web3: Web3, finalize: boolean = true) {
	// const response = await customRequest(web3, "engine_createBlock", [true, finalize, null]);
	// if (!response.result) {
	// 	throw new Error(`Unexpected result: ${JSON.stringify(response)}`);
	// }
	await new Promise<void>((resolve) => setTimeout(() => resolve(), 15000));
}

// It will include all previously executed transactions since the last finalized block.
export async function createAndFinalizeBlockNowait(web3: Web3) {
	const response = await customRequest(web3, "engine_createBlock", [true, true, null]);
	if (!response.result) {
		throw new Error(`Unexpected result: ${JSON.stringify(response)}`);
	}
}

export async function startIceNode(provider?: string): Promise<{
	web3: Web3;
	binary: ChildProcess;
	ethersjs: ethers.providers.JsonRpcProvider;
}> {
	let web3;
	if (!provider || provider == "http") {
		web3 = new Web3(`http://127.0.0.1:${RPC_PORT}`);
	}

	const cmd = BINARY_PATH;

	const args = [
		`--dev`,
		`--validator`, // Required by manual sealing to author the blocks
		`--execution=Native`, // Faster execution using native
		`--no-telemetry`,
		`--no-prometheus`,
		// `--sealing=Manual`,
		`--no-grandpa`,
		`--force-authoring`,
		`-l${ICE_LOG}`,
		`--port=${PORT}`,
		`--rpc-port=${RPC_PORT}`,
		`--ws-port=${WS_PORT}`,
		`--tmp`,
	];

	const binary = spawn(cmd, args);

	binary.on("error", (err) => {
		if ((err as any).errno == "ENOENT") {
			console.error(
				`\x1b[31mMissing Ice binary (${BINARY_PATH}).\nPlease compile the Ice project:\ncargo build\x1b[0m`
			);
		} else {
			console.error(err);
		}
		process.exit(1);
	});

	const binaryLogs = [];
	await new Promise<void>((resolve) => {
		const timer = setTimeout(() => {
			console.error(`\x1b[31m Failed to start Ice Template Node.\x1b[0m`);
			console.error(`Command: ${cmd} ${args.join(" ")}`);
			console.error(`Logs:`);
			console.error(binaryLogs.map((chunk) => chunk.toString()).join("\n"));
			process.exit(1);
		}, SPAWNING_TIME - 2000);

		const onData = async (chunk) => {
			if (DISPLAY_LOG) {
				console.log(chunk.toString());
			}
			binaryLogs.push(chunk);
			if (chunk.toString().match(/finalized #0/)) {
				if (!provider || provider == "http") {
					// This is needed as the EVM runtime needs to warmup with a first call
					await web3.eth.getChainId();
				}

				clearTimeout(timer);
				if (!DISPLAY_LOG) {
					binary.stderr.off("data", onData);
					binary.stdout.off("data", onData);
				}
				// console.log(`\x1b[31m Starting RPC\x1b[0m`);
				resolve();
			}
		};
		binary.stderr.on("data", onData);
		binary.stdout.on("data", onData);
	});

	if (provider == "ws") {
		web3 = new Web3(`ws://127.0.0.1:${WS_PORT}`);
	}

	let ethersjs = new ethers.providers.StaticJsonRpcProvider(`http://127.0.0.1:${RPC_PORT}`, {
		chainId: CHAIN_ID,
		name: "ice-dev",
	});
	return { web3, binary, ethersjs };
}

export async function connectToChain(provider) {
	let web3: Web3;

	if (!provider || provider == "http") {
		web3 = new Web3(`http://127.0.0.1:${RPC_PORT}`);
	}
	if (provider == "ws") {
		web3 = new Web3(`ws://127.0.0.1:${WS_PORT}`);
	}

	let ethersjs = new ethers.providers.StaticJsonRpcProvider(`http://127.0.0.1:${RPC_PORT}`, {
		chainId: CHAIN_ID,
		name: "ice-dev",
	});
	return { web3, ethersjs };
}

export function describeWithIce(
	title: string,
	cb: (context: { web3: Web3; ethersjs?: ethers.providers.JsonRpcProvider }) => void,
	provider?: string
) {
	describe(title, () => {
		let context: {
			web3: Web3;
			ethersjs: ethers.providers.JsonRpcProvider;
		} = { web3: null, ethersjs: null };
		let binary: ChildProcess;
		// Making sure the Ice node has started
		before("Starting Ice Test Node", async function () {
			this.timeout(SPAWNING_TIME);
			const init = await startIceNode(provider);
			context.web3 = init.web3;
			context.ethersjs = init.ethersjs;
			binary = init.binary;
		});

		after(async function () {
			binary.kill();
		});

		cb(context);
	});
}

export function describeWithIceWs(title: string, cb: (context: { web3: Web3 }) => void) {
	describeWithIce(title, cb, "ws");
}
