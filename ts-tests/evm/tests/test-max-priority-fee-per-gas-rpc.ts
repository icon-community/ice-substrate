import { ethers } from "ethers";
import { expect } from "chai";
import { step } from "mocha-steps";

import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, CHAIN_ID } from "./config";
import { describeWithIce, customRequest } from "./util";

// We use ethers library in this test as apparently web3js's types are not fully EIP-1559 compliant yet.
describeWithIce("Ice RPC (Max Priority Fee Per Gas)", (context) => {
	async function sendTransaction(context, payload: any) {
		let signer = new ethers.Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		// Ethers internally matches the locally calculated transaction hash against the one returned as a response.
		// Test would fail in case of mismatch.
		const tx = await signer.sendTransaction(payload);
		return tx;
	}

	let nonce = 0;

	step("should default to zero on genesis", async function () {
		let result = await customRequest(context.web3, "eth_maxPriorityFeePerGas", []);
		expect(result.result).to.be.eq("0x0");
	});

	step("should default to zero on empty blocks", async function () {
		let result = await customRequest(context.web3, "eth_maxPriorityFeePerGas", []);
		expect(result.result).to.be.eq("0x0");
	});
});
