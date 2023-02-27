import { expect } from "chai";
import { AbiItem } from "web3-utils";

import Test from "../build/contracts/Test.json";
import MultiContractTest from "../build/contracts/MultiContractExample.json";
import StorageContract from "../build/contracts/Storage.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, FIRST_CONTRACT_ADDRESS } from "./config";
import { describeWithIce, customRequest } from "./util";
import { ethers } from "ethers";

describeWithIce("Ice RPC (Gas)", (context) => {
	const TEST_CONTRACT_ABI = Test.abi as AbiItem[];

	// Those test are ordered. In general this should be avoided, but due to the time it takes
	// to spin up a ice node, it saves a lot of time.

	const EXTRINSIC_GAS_LIMIT = 65000000;

	it("eth_estimateGas for contract creation", async function () {
		const val = await context.web3.eth.estimateGas({
			from: GENESIS_ACCOUNT,
			data: Test.bytecode,
		});
		expect(val).to.equal(193580);
	});

	it("eth_estimateGas for contract call", async function () {
		const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI, FIRST_CONTRACT_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: "0x1D1A94A2000",
		});

		const val = await contract.methods.multiply(3).estimateGas();
		expect(val).to.equal(22331);
	});

	it("tx gas limit larger EXTRINSIC_GAS_LIMIT", async function () {
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				data: Test.bytecode,
				gas: EXTRINSIC_GAS_LIMIT + 1,
				gasPrice: "0x3B9ACA00",
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);
		const createReceipt = await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		expect((createReceipt as any).error.message).to.equal("exceeds block gas limit");
	});

	it("eth_call contract estimate_gas comparable to real gas consumed  ", async function () {
		this.timeout(50000);
		const testVal = "0x2000000000000000000000000000000000000000000000000000000000000000";

		/* Deploy Contract */
		const contractFactory = new ethers.ContractFactory(
			StorageContract.abi,
			StorageContract.bytecode,
			new ethers.Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs)
		);
		const contract = await (await contractFactory.deploy()).deployed();

		/* estimateGas Comparable to Gas Used */
		const estimateGas = await contract.estimateGas.setStorage(testVal, testVal);
		const receipt = await (await contract.setStorage(testVal, testVal)).wait();
		expect(receipt["cumulativeGasUsed"]?.toNumber() / estimateGas.toNumber()).not.lessThan(0.9).and.not.greaterThan(1.1);
	});

	it("eth_estimateGas compared to real gas used for MultiContractCall", async function () {
		this.timeout(50000);
		let genesisWallet = new ethers.Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);

		/* Deploying storage contract */
		const storageContractFactory = new ethers.ContractFactory(
			StorageContract.abi,
			StorageContract.bytecode,
			genesisWallet
		);
		const storageContract = await storageContractFactory.deploy();

		/* Deploying example Contract */
		const multiContractFactory = new ethers.ContractFactory(
			MultiContractTest.abi,
			MultiContractTest.bytecode,
			genesisWallet
		);
		const multiContract = await (await multiContractFactory.deploy(storageContract.address)).deployed();

		/* compare estimatedGas with actual gas call during method call */
		const estimatedGas = await multiContract.estimateGas.setStorage();
		const receipt = await (await multiContract.setStorage()).wait();

		expect(receipt["cumulativeGasUsed"]?.toNumber() / estimatedGas.toNumber()).not.lessThan(0.9).and.not.greaterThan(1.1);
	});
});
