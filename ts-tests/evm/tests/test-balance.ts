import { expect } from "chai";
import { ethers } from "ethers";
import { step } from "mocha-steps";

import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, GENESIS_ACCOUNT_BALANCE, EXISTENTIAL_DEPOSIT } from "./config";
import { describeWithIce, customRequestEther } from "./util";

describeWithIce("Ice RPC (Balance)", (context) => {
	const TEST_ACCOUNT = "0x4ebaae1dce71f2536d502ab2a0d4dce7fc740140";

	step("genesis balance is setup correctly", async function () {
		expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(GENESIS_ACCOUNT_BALANCE);
	});

	step("balance to be updated after transfer", async function () {
		this.timeout(50000);
		const value = "0x200000000000000000"; //balance should be greater than 10_000_000_000_000_000
		const gasPrice = await context.ethersjs.getGasPrice();

		const tx = await new ethers.Wallet(GENESIS_ACCOUNT_PRIVATE_KEY).signTransaction({
			to: TEST_ACCOUNT,
			value: value,
			gasPrice: gasPrice,
			gasLimit: "0x100000",
		});
		await customRequestEther(context.ethersjs, tx);

		const expectedGenesisBalance = (
			BigInt(GENESIS_ACCOUNT_BALANCE) -
			BigInt(21000) * gasPrice.toBigInt() -
			BigInt(value)
		).toString();

		const expectedTestBalance = (BigInt(value) - BigInt(EXISTENTIAL_DEPOSIT)).toString();
		expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(expectedGenesisBalance);
		expect(await context.web3.eth.getBalance(TEST_ACCOUNT)).to.equal(expectedTestBalance);
	});
});
