import { BigNumber } from "ethers";

export interface ContractResponse {
	number: BigNumber;
	message: String;
	testStruct: {
		num: BigNumber;
		message: String;
	};
	num_array: BigNumber[];
	fixed_str_array: String[];
	chain_id: BigNumber[];
	addr: string;
	d: number;
	simple_struct: {
		num1: number;
		bt: string;
		b1: boolean;
		addr: string;
		name: string;
	}
	message_bytes: string;
	bytes_struct: {
		one_char: string;
		three_char: string;
		four_char: string;
		sixteen_char: string;
		thirtytwo_char: string;
	}
}
