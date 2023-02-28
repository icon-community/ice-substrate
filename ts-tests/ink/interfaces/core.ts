import { EventRecord } from "@polkadot/types/interfaces/system";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
export interface TxOptions {
	gasLimit: WeightV2;
	storageDepositLimit?: string | null;
	tip?: string;
	value?: string;
}

export interface ContractInterface {
	address: string | undefined;
	codeHash?: string;
	blockHash?: string;
	blockNum?: number;
	metadata?: string;
	wasm?: string;
}

export interface QueryArgs {
	txOptions: TxOptions;
	sender: string;
	args: Array<unknown>;
}

export interface BlockInterface {
	blockHash: string;
	blockNumber: number;
	events?: Array<EventRecord>;
}
