import { expect } from "chai";
import { ethers } from "ethers";
import { step } from "mocha-steps";

import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { describeWithIce, customRequestEther } from "./util";

describeWithIce("Ice RPC (Nonce)", (context) => {
	const TEST_ACCOUNT = "0x1111111111111111111111111111111111111111";

	step("get nonce", async function () {
		this.timeout(20_000);
		const value = "0x200000000000000000"; //balance should be greater than 10_000_000_000_000_000
		const gasPrice = await context.ethersjs.getGasPrice();

		const tx = await new ethers.Wallet(GENESIS_ACCOUNT_PRIVATE_KEY).signTransaction({
			to: TEST_ACCOUNT,
			value: value,
			gasPrice: gasPrice,
			gasLimit: "0x100000",
		});

		expect(await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT, "earliest")).to.eq(0);

		await customRequestEther(context.ethersjs, tx);

		expect(await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT, "latest")).to.eq(1);
		expect(await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT, "earliest")).to.eq(0);
	});
});
