// import { expect } from "chai";
// import { step } from "mocha-steps";

// import { createAndFinalizeBlock, describeWithIce } from "./util";

// describeWithIce("Ice RPC (State root hash)", (context) => {
// 	let block;
// 	step("should calculate a valid intermediate state root hash", async function () {
// 		block = await context.web3.eth.getBlock(0);
// 		console.log({ block });
// 		// expect(block.stateRoot.length).to.be.equal(66); // 0x prefixed
// 		// expect(block.stateRoot).to.not.be.equal("0x0000000000000000000000000000000000000000000000000000000000000000");
// 	});

// 	step("hash should be unique between blocks", async function () {
// 		const anotherBlock = await context.web3.eth.getBlock(2);
// 		console.log({
// 			anotherBlock,
// 			block,
// 		});
// 		// expect(block.stateRoot).to.not.be.equal(anotherBlock.stateRoot);
// 	});
// });
