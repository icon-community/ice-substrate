import { expect } from "chai";

import { CHAIN_ID } from "./config";
import { describeWithIce } from "./util";

// All test for the RPC

describeWithIce("Ice RPC (Constant)", (context) => {
	it("should have 0 hashrate", async function () {
		expect(await context.web3.eth.getHashrate()).to.equal(0);
	});

	it("should have chainId", async function () {
		// The chainId is defined by the Substrate Chain Id, default to 42
		expect(await context.web3.eth.getChainId()).to.equal(CHAIN_ID);
	});

	it("should have no account", async function () {
		expect(await context.web3.eth.getAccounts()).to.eql([]);
	});
});
