import { expect } from "chai";
import { AbiItem } from "web3-utils";

import Test from "../build/contracts/Test.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, FIRST_CONTRACT_ADDRESS } from "./config";
import { describeWithIce, customRequest } from "./util";

describeWithIce("Ice RPC (Gas)", (context) => {
	const TEST_CONTRACT_ABI = Test.abi as AbiItem[];

	// Those test are ordered. In general this should be avoided, but due to the time it takes
	// to spin up a ice node, it saves a lot of time.

	const EXTRINSIC_GAS_LIMIT = 65000000;

	it("eth_estimateGas for contract creation", async function () {
		const val = await context.web3.eth.estimateGas({
			from: GENESIS_ACCOUNT,
			data: Test.bytecode,
		});

		expect(val).to.equal(193417);
	});

	it("eth_estimateGas for contract call", async function () {
		const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI, FIRST_CONTRACT_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: "0x1D1A94A2000",
		});

		const val = await contract.methods.multiply(3).estimateGas();
		expect(val).to.equal(21204);
	});

	it("tx gas limit larger EXTRINSIC_GAS_LIMIT", async function () {
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				data: Test.bytecode,
				gas: EXTRINSIC_GAS_LIMIT + 1,
				gasPrice: "0x3B9ACA00",
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);
		const createReceipt = await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		expect((createReceipt as any).error.message).to.equal("exceeds block gas limit");
	});
});
