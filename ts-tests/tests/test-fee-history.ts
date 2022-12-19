import { ethers } from "ethers";
import { expect } from "chai";
import { step } from "mocha-steps";

import { describeWithIce, customRequest } from "./util";

// We use ethers library in this test as apparently web3js's types are not fully EIP-1559 compliant yet.
describeWithIce("Ice RPC (Fee History)", (context) => {
	step("should return error on non-existent blocks", async function () {
		this.timeout(100000);
		let result = customRequest(context.web3, "eth_feeHistory", ["0x0", "0x1", []])
			.then(() => {
				return Promise.reject({
					message: "Execution succeeded but should have failed",
				});
			})
			.catch((err) => expect(err.message).to.equal("Error getting header at BlockId::Number(1)"));
	});
});
