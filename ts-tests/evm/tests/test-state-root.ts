import { expect } from "chai";
import { step } from "mocha-steps";

import { createAndFinalizeBlock, describeWithIce } from "./util";

describeWithIce("Ice RPC (State root hash)", (context) => {
	let block;
	step("should calculate a valid intermediate state root hash", async function () {
		block = await context.web3.eth.getBlock("latest");
		expect(block.stateRoot.length).to.be.equal(66); // 0x prefixed
	});

	step("hash should be unique between blocks", async function () {
		this.timeout(30000);
		await createAndFinalizeBlock(context.web3);
		const anotherBlock = await context.web3.eth.getBlock("latest");
		expect(block.stateRoot).to.not.be.equal(anotherBlock.stateRoot);
	});
});
