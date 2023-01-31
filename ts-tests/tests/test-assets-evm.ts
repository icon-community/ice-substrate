import { expect, use as chaiUse } from "chai";
import chaiAsPromised from "chai-as-promised";
import { Contract, ContractFactory, Signer, Wallet } from "ethers";

import IERC20 from "../build/contracts/IERC20.json";
import IERC20Plus from "../build/contracts/IERC20Plus.json";
import { GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import { customRequest, describeWithIce } from "./util";

chaiUse(chaiAsPromised);

describeWithIce("Ice RPC (AssetsERC20)", (context) => {
	let genesisAccount: Signer;
	let erc20: Contract;
	let erc20Plus: Contract;
	const receiver = "0xe735008ea5683238C3DAf2736a456538818F0A80";

	before("create the contract", async function () {
		this.timeout(15000);
		genesisAccount = new Wallet(GENESIS_ACCOUNT_PRIVATE_KEY, context.ethersjs);
		erc20 = new Contract("ffffffff00000000000000000000000000000001", IERC20.abi, genesisAccount);
		erc20Plus = new Contract("ffffffff00000000000000000000000000000001", IERC20Plus.abi, genesisAccount);
	});

	it("should return total supply", async function () {
		this.timeout(15000);
		const totalSupply = await erc20.totalSupply();
		expect(totalSupply.toString()).to.equal("100");
	});

	it("should return name", async function () {
		this.timeout(15000);
		const name = await erc20.name();
		expect(name.toString()).to.equal("Test Token");
	});

	it("should return decimals", async function () {
		this.timeout(15000);
		const decimals = await erc20.decimals();
		expect(decimals.toString()).to.equal("10");
	});

	it("should return symbol", async function () {
		this.timeout(15000);
		const symbol = await erc20.symbol();
		expect(symbol.toString()).to.equal("TICZ");
	});

	it("should be able to transfer token", async function () {
		this.timeout(15000);
		const prevBalance = await erc20.balanceOf(receiver);
		const tx = await erc20.transfer(receiver, "10");
		await tx.wait();
		const balanceOf = await erc20.balanceOf(receiver);
		expect(balanceOf.toString()).to.equal((Number.parseInt(prevBalance)+10).toString());
	});
	// erc20Plus

	it("should return minbalance", async function () {
		this.timeout(15000);
		const totalSupply = await erc20Plus.minimumBalance();
		expect(totalSupply.toString()).to.equal("1");
	});

	it("should burn token", async function () {
		this.timeout(15000);
		const prevSupply = await erc20Plus.totalSupply();
		const tx= await erc20Plus.burn(genesisAccount.getAddress(),"1", {gasLimit: 5000000});
		await tx.wait();
		const totalSupply = await erc20Plus.totalSupply();
		expect(totalSupply.toString()).to.equal((Number.parseInt(prevSupply)-1).toString());
	});

	it("should mint token", async function () {
		this.timeout(15000);
		const prevSupply = await erc20Plus.totalSupply();
		const tx= await erc20Plus.mint(genesisAccount.getAddress(),"1", {gasLimit: 5000000});
		await tx.wait();
		const totalSupply = await erc20Plus.totalSupply();
		expect(totalSupply.toString()).to.equal((Number.parseInt(prevSupply)+1).toString());
	});
});
