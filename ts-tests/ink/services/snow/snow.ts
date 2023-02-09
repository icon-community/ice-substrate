import { ChildProcessWithoutNullStreams, spawn } from "child_process";
import { sleep } from "./helpers";
import { BINARY_PATH, MAINNET_WSS_URL, LOCAL_WSS_URL, KEYRING_TYPE, CHAIN_PREFIX, ALICE_URI } from "../../constants";
import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { CodePromise, ContractPromise } from "@polkadot/api-contract";
import { KeyringPair } from "@polkadot/keyring/types";
import { DispatchError } from "@polkadot/types/interfaces";
import BigNumber from "bignumber.js";
import { BlockInterface, ContractInterface, QueryArgs, TxOptions } from "../../interfaces/core";

import dotenv from "dotenv-flow";
import { ISubmittableResult } from "@polkadot/types/types";
dotenv.config();

const BUFFER_TIME = 3; // sec

class SnowApi {
	private static binary: undefined | ChildProcessWithoutNullStreams;
	static api: undefined | ApiPromise;
	public static keyring: undefined | Keyring;
	static alice: undefined | KeyringPair;
	static endUserWallets: Array<KeyringPair>;

	static initialize = async (isMainnet?: boolean) => {
		// todo
		if (!isMainnet) {
			SnowApi.startNetwork();
		}
		SnowApi.api = await SnowApi.connectSnowApi(isMainnet ? MAINNET_WSS_URL : LOCAL_WSS_URL);
		SnowApi.keyring = new Keyring({
			type: KEYRING_TYPE,
			ss58Format: CHAIN_PREFIX,
		});
		SnowApi.alice = SnowApi.keyring.addFromUri(ALICE_URI);
		SnowApi.endUserWallets = SnowApi.getEndUserWallets();
	};

	private static getEndUserWallets() {
		let counter = 0;
		const wallets: Array<KeyringPair> = [];
		while (process.env[`END_USER_WALLET_${++counter}_URI`]) {
			wallets.push(SnowApi.keyring?.addFromUri(process.env[`END_USER_WALLET_${counter}_URI`]!)!);
		}
		return wallets;
	}

	static getNonce = async (address: string): Promise<number> => {
		// @ts-ignore
		const { nonce } = await SnowApi.api!.query.system.account(address);
		return parseInt(nonce.toHex());
	};

	static fundEndUserWallets = async (amount: BigNumber): Promise<boolean> => {
		let aliceNonce = await SnowApi.getNonce(SnowApi.alice!.address);

		return new Promise((resolve, reject) => {
			const transferPromises = [];
			for (const endWallet of SnowApi.endUserWallets) {
				const transferPromise = SnowApi.sendBalance(SnowApi.alice!, endWallet.address, amount, {
					nonce: aliceNonce++,
				});
				transferPromises.push(transferPromise);
			}
			Promise.all(transferPromises)
				.then(() => resolve(true))
				.catch((err) => {
					console.log("Error transferring funds to end user wallets");
					reject(err);
				});
		});
	};

	private static connectSnowApi = async (nodeUrl: string) => {
		const provider = new WsProvider(nodeUrl);
		const _api = new ApiPromise({ provider });
		await _api.isReady;
		console.log(`Connected to ${nodeUrl}`);
		return _api;
	};

	private static startNetwork = async () => {
		const cmd = BINARY_PATH;
		const args = [`--dev`];
		SnowApi.binary = spawn(cmd, args);
		SnowApi.binary.on("error", (err) => {
			console.error(err);
			process.exit(-1);
		});
		await sleep(BUFFER_TIME);
	};

	private static checkError = (errorObj: DispatchError | undefined) => {
		// Check if error occurred. If yes, set errorMsg.
		if (errorObj) {
			if (errorObj.isModule) {
				const decoded = SnowApi.api!.registry.findMetaError(errorObj.asModule);
				const { docs, name, section } = decoded;

				throw new Error(`${section}.${name}: ${docs}`);
			}
		}
	};

	static getLastBlock = async (): Promise<BlockInterface> => {
		const blockNum = (await this.api?.query.system.number())?.toString();
		const blockHash = (await this.api?.query.system.blockHash(blockNum))?.toString();
		if (blockNum && blockHash) {
			return { blockHash, blockNumber: parseInt(blockNum) };
		} else {
			throw new Error("Error fetching last block metadata");
		}
	};

	static getBlockHashByNumber = async (blockNum: number): Promise<string> => {
		const blockHash = await (await this.api?.query.system.blockHash(blockNum))?.toString();
		if (!blockHash) throw new Error(`Error getting block hash at block ${blockNum}`);
		return blockHash;
	};

	static deployContract = async (
		ctxMetadata: string,
		ctxWasm: string,
		txOptions: TxOptions,
		args: Array<unknown>,
		deployerWallet: KeyringPair,
	): Promise<ContractInterface> => {
		const ctxCode = new CodePromise(SnowApi.api!, ctxMetadata, ctxWasm);
		const tx = ctxCode.tx.new(txOptions, ...args);

		return new Promise(async (resolve, reject) => {
			const unsub = await tx.signAndSend(deployerWallet, async (result) => {
				// @ts-ignore
				const { status, contract, dispatchError, blockNumber } = result;
				if (status.isInBlock || status.isFinalized) {
					try {
						SnowApi.checkError(dispatchError);
					} catch (err) {
						return reject(err);
					}

					const address = contract.address.toString();
					unsub();
					return resolve({
						address,
						blockHash: status.asInBlock.toString(),
						blockNum: blockNumber.toNumber(),
					});
				}
			});
		});
	};

	static queryContract = async (ctxObj: ContractPromise, queryFunc: string, args: QueryArgs) => {
		try {
			// @ts-ignore
			const { output, result } = await ctxObj.query[queryFunc](args.sender, args.txOptions, ...args.args);

			// @ts-ignore
			if (output?.isErr || result?.isErr) {
				// @ts-ignore
				const errorMsg = output?.toJSON()?.err || result?.toJSON()?.err;
				throw new Error(errorMsg || "Unknown error");
			}

			return { output, result };
		} catch (err) {
			console.error(err);
			throw new Error(`Error querying ${queryFunc} on contract: ${ctxObj.address}`);
		}
	};

	static dryRunTransaction = async (
		ctxObj: ContractPromise,
		writeFunc: string,
		sender: string,
		txOptions: TxOptions,
		args: Array<unknown>,
	) => {
		if (ctxObj?.query?.[writeFunc]) {
			// @ts-ignore
			const res = await ctxObj.query[writeFunc](sender, txOptions, ...args);
			const { result, output, storageDeposit, gasRequired } = res;

			const deposit = storageDeposit?.toHuman();

			// @ts-ignore
			if (output?.isErr || result?.isErr) {
				// @ts-ignore
				const errorMsg = output?.toJSON()?.err || result?.toJSON()?.err;
				throw new Error(errorMsg || "Unknown error");
			}
			return {
				isOk: true,
				result: {
					gasLimit: gasRequired, //otto unit
					// @ts-ignore
					deposit: new BigNumber(deposit?.Charge.split(" ")?.[0]), //ICZ unit
				},
			};
		} else {
			throw new Error("Method doesn't exist");
		}
	};

	static getCodeHash = async (ctxAddress: string): Promise<string> => {
		const ctxDetails = (await SnowApi.api?.query.contracts.contractInfoOf(ctxAddress))?.toHuman();
		if (ctxDetails) {
			// @ts-ignore
			return ctxDetails.codeHash;
		} else {
			throw new Error(`Could not get code hash for ${ctxAddress}`);
		}
	};

	static getBalance = async (address: string, reserve?: boolean): Promise<BigNumber> => {
		const bal = await SnowApi.api?.query.system.account(address);
		// @ts-ignore
		return reserve ? new BigNumber(bal?.data.reserved.toBigInt()) : new BigNumber(bal?.data.free.toBigInt());
	};

	static writeContract = (
		from: KeyringPair,
		ctxObj: ContractPromise,
		writeFunc: string,
		txOptions: TxOptions,
		funcArgs: Array<unknown>,
		tip?: string,
		nonce?: number,
		callback?: (result: ISubmittableResult) => void,
	): Promise<BlockInterface> => {
		const tx = ctxObj.tx[writeFunc](txOptions, ...funcArgs);

		return new Promise(async (resolve, reject) => {
			try {
				const unsub = await tx.signAndSend(from, { nonce: nonce ?? -1, tip }, async (result) => {
					// @ts-ignore
					const { status, dispatchError, blockNumber, events } = result;
					if (status.isInBlock || status.isFinalized) {
						try {
							SnowApi.checkError(dispatchError);
						} catch (err) {
							return reject(err);
						}
						unsub();
						if (callback) callback(result);
						resolve({
							blockHash: status.asInBlock.toString(),
							blockNumber: blockNumber.toNumber(),
							events,
						});
					}
				});
			} catch (err) {
				return reject(err);
			}
		});
	};

	static sendBalance = (
		sender: KeyringPair,
		receiverAddr: string,
		weiAmount: BigNumber,
		options?: { nonce: number },
	) => {
		return new Promise(async (resolve, reject) => {
			const unsub = await SnowApi.api!.tx.balances.transfer(receiverAddr, weiAmount.toFixed(0)).signAndSend(
				sender,
				options ?? {},
				({ status, dispatchError }) => {
					if (status.isInBlock || status.isFinalized) {
						try {
							SnowApi.checkError(dispatchError);
						} catch (err) {
							return reject(err);
						}
						unsub();
						return resolve(true);
					}
				},
			);
		});
	};

	// static setBalance = () => {};

	static cleanUp = () => {
		this.api?.disconnect();
		this.binary?.kill();
	};
}

export default SnowApi;
