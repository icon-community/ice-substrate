import { assert } from "chai";
import { ContractFactory, Wallet } from "ethers";
import { AbiItem } from "web3-utils";

import ECRecoverTests from "../build/contracts/ECRecoverTests.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { describeWithIce } from "./util";

describeWithIce("Ice RPC (Precompile)", (context) => {
	const TEST_CONTRACT_BYTECODE = ECRecoverTests.bytecode;
	const TEST_CONTRACT_ABI = ECRecoverTests.abi;

	let web3;
	let contractAddress = "";

	before(async function () {
		this.timeout(80000);
		web3 = context.web3;

		const genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(TEST_CONTRACT_ABI, TEST_CONTRACT_BYTECODE, genesisAccount);
		const contract = await factory.deploy();
		await contract.deployed();
		contractAddress = contract.address;
		web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
		web3.eth.defaultAccount = web3.eth.accounts.wallet[0].address;
	});

	// Those test are ordered. In general this should be avoided, but due to the time it takes
	// to spin up a ice node, it saves a lot of time.

	it("should perform ecrecover", async function () {
		this.timeout(15000);

		const web3 = context.web3;

		const message =
			"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Tubulum fuisse, qua illum, cuius is condemnatus est rogatione, P. Eaedem res maneant alio modo.";
		const messageHex = "0x" + Buffer.from(message).toString("hex");
		const sig = (await web3.eth.sign(messageHex, GENESIS_ACCOUNT)).slice(2);
		const r = `${sig.slice(0, 64)}`;
		const s = `${sig.slice(64, 128)}`;
		const v = `${sig.slice(128, 130)}`;
		const sigPart = `${Buffer.alloc(31).toString("hex")}${v}${r}${s}`;
		const hash = web3.utils.sha3("\x19Ethereum Signed Message:\n" + message.length + message).slice(2);

		const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI as AbiItem[], contractAddress, {
			from: GENESIS_ACCOUNT,
			gasPrice: "0x1D1A94A200000",
		});

		await contract.methods.ecrecover(`0x${hash.toString()}${sigPart}`).call();
	});

	it("should perform identity directly", async () => {
		const message = "0x1234567890";
		const callResult = await web3.eth.call({
			to: "0000000000000000000000000000000000000004",
			from: GENESIS_ACCOUNT,
			data: message,
		});
		assert.equal(callResult, message);
	});
});
