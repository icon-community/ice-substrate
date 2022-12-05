import { expect } from "chai";
import { step } from "mocha-steps";

import { createAndFinalizeBlock, describeWithIce } from "./util";

describeWithIce("Ice RPC (State root hash)", (context) => {
	let block;
	step("should calculate a valid intermediate state root hash", async function () {
		block = await context.web3.eth.getBlock("latest");
		console.log({ block });
		expect(block.stateRoot.length).to.be.equal(66); // 0x prefixed
		// expect(block.stateRoot).to.not.be.equal("0x1b09325951dfa631735e3455c08e0ba284d7e4940e3502e955b39616971882e8");
	});

	step("hash should be unique between blocks", async function () {
		this.timeout(30000);
		await createAndFinalizeBlock(context.web3);
		const anotherBlock = await context.web3.eth.getBlock("latest");
		console.log({
			anotherBlock,
			block,
		});
		expect(block.stateRoot).to.not.be.equal(anotherBlock.stateRoot);
	});
});
