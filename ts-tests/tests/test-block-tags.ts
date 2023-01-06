import { expect } from "chai";
import { step } from "mocha-steps";

import { describeWithIce } from "./util";

describeWithIce("Ice RPC (BlockNumber tags)", (context) => {
	step("`earliest` returns genesis", async function () {
		expect((await context.web3.eth.getBlock("earliest")).number).to.equal(0);
	});
});
