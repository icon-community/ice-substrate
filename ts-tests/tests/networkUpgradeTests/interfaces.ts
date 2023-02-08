import { BigNumber } from "ethers";

export interface ContractResponse {
	number: BigNumber;
	message: String;
	testStruct: {
		num: BigNumber;
		message: String;
	};
	testArray: BigNumber[];
}
