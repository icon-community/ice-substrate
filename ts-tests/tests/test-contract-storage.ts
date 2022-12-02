import { expect } from "chai";
import { ContractFactory, Wallet } from "ethers";
import { AbiItem } from "web3-utils";

import Test from "../build/contracts/Storage.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, FIRST_CONTRACT_ADDRESS } from "./config";
import { createAndFinalizeBlock, customRequest, describeWithIce } from "./util";

describeWithIce("Ice RPC (Contract storage)", (context) => {
	it("eth_getStorageAt", async function () {
		this.timeout(15000);
		const genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(Test.abi, Test.bytecode, genesisAccount);
		const contract = await factory.deploy();

		let contractAddress = contract.address;

		expect(
			await contract.getStorage("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")
		).to.equal("0x0000000000000000000000000000000000000000000000000000000000000000");

		expect(
			await context.ethersjs.getStorageAt(
				contract.address,
				"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
			)
		).to.equal("0x0000000000000000000000000000000000000000000000000000000000000000");

		await contract.setStorage(
			"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
			"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
		);

		// expect(
		// 	await contract.getStorage("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")
		// ).to.equal("0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");

		// expect(
		// 	await context.ethersjs.getStorageAt(
		// 		contract.address,
		// 		"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc"
		// 	)
		// ).to.equal("0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");

		// expect(
		// 	await context.ethersjs.getStorageAt(
		// 		contract.address,
		// 		"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
		// 		"earliest"
		// 	)
		// ).to.equal("0x0000000000000000000000000000000000000000000000000000000000000000");

		// expect(
		// 	await context.ethersjs.getStorageAt(
		// 		contract.address,
		// 		"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
		// 		"latest"
		// 	)
		// ).to.equal("0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");

		// expect(getStorage0.result).to.be.eq("0x0000000000000000000000000000000000000000000000000000000000000000");

		// const tx1 = await context.web3.eth.accounts.signTransaction(
		// 	{
		// 		from: GENESIS_ACCOUNT,
		// 		to: contractAddress,
		// 		data: contract.methods
		// 			.setStorage(
		// 				"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
		// 				"0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
		// 			)
		// 			.encodeABI(),
		// 		value: "0x00",
		// 		gasPrice: "0x3B9ACA00",
		// 		gas: "0x500000",
		// 	},
		// 	GENESIS_ACCOUNT_PRIVATE_KEY
		// );

		// await customRequest(context.web3, "eth_sendRawTransaction", [tx1.rawTransaction]);

		// let getStoragePending = await customRequest(context.web3, "eth_getStorageAt", [
		// 	contractAddress,
		// 	"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
		// 	"pending",
		// ]);

		// const expectedStorage = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

		// expect(getStoragePending.result).to.be.eq(expectedStorage);

		// // await createAndFinalizeBlock(context.web3);

		// let getStorage1 = await customRequest(context.web3, "eth_getStorageAt", [
		// 	contractAddress,
		// 	"0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc",
		// 	"latest",
		// ]);

		// expect(getStorage1.result).to.be.eq(expectedStorage);
	});

	// it("SSTORE cost should properly take into account transaction initial value", async function () {
	// 	this.timeout(5000);

	// 	let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);

	// 	await context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
	// 	const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI, FIRST_CONTRACT_ADDRESS, {
	// 		from: GENESIS_ACCOUNT,
	// 		gasPrice: "0x3B9ACA00",
	// 	});

	// 	const promisify = (inner) => new Promise((resolve, reject) => inner(resolve, reject));

	// 	let tx1 = contract.methods
	// 		.setStorage("0x2A", "0x1")
	// 		.send({ from: GENESIS_ACCOUNT, gas: "0x100000", nonce: nonce++ });

	// 	let tx2 = contract.methods
	// 		.setStorage("0x2A", "0x1")
	// 		.send({ from: GENESIS_ACCOUNT, gas: "0x100000", nonce: nonce++ });

	// 	let tx3 = contract.methods
	// 		.setStorage("0x2A", "0x2")
	// 		.send(
	// 			{ from: GENESIS_ACCOUNT, gas: "0x100000", nonce: nonce++ },
	// 			async (hash) => await createAndFinalizeBlock(context.web3)
	// 		);

	// 	tx1 = await tx1;
	// 	tx2 = await tx2;
	// 	tx3 = await tx3;

	// 	// cost minus SSTORE
	// 	const baseCost = 24029;

	// 	// going from unset storage to some value (original = 0)
	// 	expect(tx1.gasUsed - baseCost).to.be.eq(20000);
	// 	// in London config, setting back the same value have cost of warm read
	// 	expect(tx2.gasUsed - baseCost).to.be.eq(100);
	// 	// - the original storage didn't change in the current transaction
	// 	// - the original storage is not zero (otherwise tx1)
	// 	expect(tx3.gasUsed - baseCost).to.be.eq(2900);
	// });
});
