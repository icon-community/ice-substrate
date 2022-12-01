import { TransactionReceipt } from "@ethersproject/abstract-provider";
import { expect } from "chai";
import { deployContract, deployMockContract } from "ethereum-waffle";
import { Contract, ContractFactory, ethers, Signer, Wallet } from "ethers";
import { AbiItem } from "web3-utils";

import Test from "../build/contracts/Test.json";
import {
	GENESIS_ACCOUNT,
	GENESIS_ACCOUNT_PRIVATE_KEY,
	FIRST_CONTRACT_ADDRESS,
	BLOCK_HASH_COUNT,
	BLOCK_GAS_LIMIT,
	CHAIN_ID,
} from "./config";
import { createAndFinalizeBlockNowait, customRequest, customRequestEther, describeWithIce } from "./util";

describeWithIce("Ice RPC (Contract Methods)", (context) => {
	const TEST_CONTRACT_BYTECODE = Test.bytecode;
	const TEST_CONTRACT_ABI = Test.abi;
	// Those test are ordered. In general this should be avoided, but due to the time it takes
	// to spin up a ice node, it saves a lot of time.
	let genesisAccount: Signer;
	let contract: Contract;

	before("create the contract", async function () {
		this.timeout(15000);
		genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		let factory = new ContractFactory(TEST_CONTRACT_ABI, TEST_CONTRACT_BYTECODE, genesisAccount);
		contract = await factory.deploy();
	});

	// it("get transaction by hash", async () => {
	// 	const latestBlock = await context.web3.eth.getBlock(1);
	// 	expect(latestBlock.transactions.length).to.equal(1);
	// 	const txHash = latestBlock.transactions[0];
	// 	const tx = await context.ethersjs.getTransaction(txHash);
	// 	expect(tx.hash).to.equal(txHash);
	// });

	it("should return contract method result", async function () {
		this.timeout(15000);
		expect((await contract.multiply(3)).toString()).to.equal("21");
	});
	it("should get correct environmental block number", async function () {
		// Solidity `block.number` is expected to return the same height at which the runtime call was made.
		this.timeout(15000);
		let height = await contract.currentBlock();
		let current_block_number = await context.ethersjs.getBlockNumber();
		expect(height?.toString()).to.eq(current_block_number.toString());
	});

	it("should get correct environmental block hash", async function () {
		this.timeout(15000);
		// Solidity `blockhash` is expected to return the ethereum block hash at a given height.
		let number = await context.ethersjs.getBlockNumber();

		expect(await contract.blockHash(number - 1)).to.not.eq(
			"0x0000000000000000000000000000000000000000000000000000000000000000"
		);

		// expect(await contract.blockHash(number)).to.eq(
		// 	"0x0000000000000000000000000000000000000000000000000000000000000000"
		// );
	});

	it("should get correct environmental block gaslimit", async function () {
		expect((await contract.gasLimit()).toString()).to.eq(BLOCK_GAS_LIMIT.toString());
	});

	// // Requires error handling
	it("should fail for missing parameters", async function () {
		const mock = new Contract(
			contract.address,
			[
				{
					...Test.abi.filter(function (entry) {
						return entry.name === "multiply";
					})[0],
					inputs: [],
				},
			],
			genesisAccount
		);

		await mock.multiply().catch((err) => {
			expect(err.message).to.match(
				new RegExp(`missing revert data in call exception; Transaction reverted without`)
			);
		});
	});

	// Requires error handling
	it("should fail for too many parameters", async function () {
		const mock = new Contract(
			contract.address,
			[
				{
					...Test.abi.filter(function (entry) {
						return entry.name === "multiply";
					})[0],
					inputs: [
						{ internalType: "uint256", name: "a", type: "uint256" },
						{ internalType: "uint256", name: "b", type: "uint256" },
					],
				},
			],
			genesisAccount
		);
		await mock.multiply(3, 4).catch((err) => {
			expect(err.message).to.match(
				new RegExp(`missing revert data in call exception; Transaction reverted without`)
			);
		});
	});

	// Requires error handling
	it("should fail for invalid parameters", async function () {
		const mock = new Contract(
			contract.address,
			[
				{
					...Test.abi.filter(function (entry) {
						return entry.name === "multiply";
					})[0],
					inputs: [{ internalType: "address", name: "a", type: "address" }],
				},
			],
			genesisAccount
		);
		await mock.multiply("0x0123456789012345678901234567890123456789").catch((err) => {
			expect(err.message).to.match(
				new RegExp(`missing revert data in call exception; Transaction reverted without`)
			);
		});
	});
});
