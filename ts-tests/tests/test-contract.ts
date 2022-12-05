import { expect, use as chaiUse } from "chai";
import chaiAsPromised from "chai-as-promised";
import { Contract, ContractFactory, Signer, Wallet } from "ethers";

import Test from "../build/contracts/Test.json";
import { GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { customRequest, describeWithIce } from "./util";

chaiUse(chaiAsPromised);

describeWithIce("Ice RPC (Contract)", (context) => {
	const TEST_CONTRACT_BYTECODE = Test.bytecode;
	const TEST_CONTRACT_DEPLOYED_BYTECODE = Test.deployedBytecode;
	let genesisAccount: Signer;
	let contract: Contract;

	it("contract creation should return transaction hash", async function () {
		// await createAndFinalizeBlock(context.web3);
		this.timeout(15000);

		genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(Test.abi, Test.bytecode, genesisAccount);
		contract = await factory.deploy();

		// // Verify the contract is not yet stored
		expect(await customRequest(context.web3, "eth_getCode", [contract.address])).to.deep.equal({
			id: 1,
			jsonrpc: "2.0",
			result: "0x",
		});

		//wait until contract deployed
		await contract.deployed();
		// // Verify the contract is stored after the block is produced
		expect(await customRequest(context.web3, "eth_getCode", [contract.address])).to.deep.equal({
			id: 1,
			jsonrpc: "2.0",
			result: TEST_CONTRACT_DEPLOYED_BYTECODE,
		});
	});

	it("eth_call contract create should return code", async function () {
		expect(
			await context.web3.eth.call({
				data: TEST_CONTRACT_BYTECODE,
			})
		).to.be.eq(TEST_CONTRACT_DEPLOYED_BYTECODE);
	});

	it("eth_call at missing block returns error", async function () {
		const nonExistingBlockNumber = "999999";
		return expect(
			context.web3.eth.call(
				{
					data: TEST_CONTRACT_BYTECODE,
				},
				nonExistingBlockNumber
			)
		).to.eventually.rejectedWith("header not found");
	});
});
