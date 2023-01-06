import { expect } from "chai";
import { ContractFactory, Wallet } from "ethers";
import { AbiItem } from "web3-utils";

import ExplicitRevertReason from "../build/contracts/ExplicitRevertReason.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { describeWithIce } from "./util";

describeWithIce("Ice RPC (Revert Reason)", (context) => {
	let contractAddress;

	const REVERT_W_MESSAGE_BYTECODE = ExplicitRevertReason.bytecode;

	const TEST_CONTRACT_ABI = ExplicitRevertReason.abi;

	before("create the contract", async function () {
		this.timeout(15000);
		let genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(TEST_CONTRACT_ABI, REVERT_W_MESSAGE_BYTECODE, genesisAccount);
		let contract = await factory.deploy();
		contract = await contract.deployed();
		contractAddress = contract.address;
		contract.deployTransaction.hash;
	});

	it("should fail with revert reason", async function () {
		const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI as AbiItem[], contractAddress, {
			from: GENESIS_ACCOUNT,
			gasPrice: (await context.ethersjs.getGasPrice()).toString(),
		});
		try {
			await contract.methods.max10(30).call();
		} catch (error) {
			expect(error.message).to.be.eq(
				"Returned error: VM Exception while processing transaction: revert Value must not be greater than 10."
			);
		}
	});
});
