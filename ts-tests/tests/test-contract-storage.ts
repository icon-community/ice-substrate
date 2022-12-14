import { expect } from "chai";
import { ContractFactory, Wallet } from "ethers";

import Test from "../build/contracts/Storage.json";
import { GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { describeWithIce } from "./util";

describeWithIce("Ice RPC (Contract storage)", (context) => {
	it("eth_getStorageAt", async function () {
		this.timeout(15000);
		const genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(Test.abi, Test.bytecode, genesisAccount);
		const contract = await factory.deploy();

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
	});
});
