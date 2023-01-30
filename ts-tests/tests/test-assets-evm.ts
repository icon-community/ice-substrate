import { expect, use as chaiUse } from "chai";
import chaiAsPromised from "chai-as-promised";
import { Contract, ContractFactory, Signer, Wallet } from "ethers";

import IERC20 from "../build/contracts/IERC20.json";
import { GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { customRequest, describeWithIce } from "./util";

chaiUse(chaiAsPromised);

describeWithIce("Ice RPC (AssetsERC20)", (context) => {
	let genesisAccount: Signer;
	let contract: Contract;

	before("create the contract", async function () {
		this.timeout(15000);
		genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		contract = new Contract("ffffffff00000000000000000000000000000001", IERC20.abi, genesisAccount);
	});

	it("should return total supply", async function () {
		this.timeout(15000);
		const totalSupply = await contract.totalSupply();
		expect(totalSupply.toString()).to.equal("100");
	});

	it("should return name", async function () {
		this.timeout(15000);
		const totalSupply = await contract.name();
		expect(totalSupply.toString()).to.equal("Test Token");
	});

	it("should return decimals", async function () {
		this.timeout(15000);
		const totalSupply = await contract.decimals();
		expect(totalSupply.toString()).to.equal("10");
	});

	it("should return symbol", async function () {
		this.timeout(15000);
		const totalSupply = await contract.symbol();
		expect(totalSupply.toString()).to.equal("TICZ");
	});

	it("should be able to transfer token", async function () {
		this.timeout(15000);
		const receiver = "0xe735008ea5683238C3DAf2736a456538818F0A80";
		const tx = await contract.transfer(receiver, "10");
		await tx.wait();
		const balanceOf = await contract.balanceOf(receiver);
		expect(balanceOf.toString()).to.equal("10");
	});
});
