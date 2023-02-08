import { Contract, ContractFactory, Signer, Wallet } from "ethers";
import dotenv from "dotenv";

import { ethersProvider } from "./api";
import UpgradeCtx from "../../build/contracts/NetworkUpgrade.json";

dotenv.config();
const MAINNET_DEPLOYER_KEY = process.env["MAINNET_DEPLOYER_KEY"];

async function deployContract() {
	let deployer: Signer = new Wallet(MAINNET_DEPLOYER_KEY, ethersProvider);
	let factory = new ContractFactory(UpgradeCtx.abi, UpgradeCtx.bytecode, deployer);
	const contract: Contract = await factory.deploy(20, "SNOW Network");

	//wait until contract deployed
	await contract.deployed();

	console.log("Deployer wallet: ", await deployer.getAddress());
	console.log("Contract address: ", contract.address);
	console.log("\nSuccessfully deployed contract");
}

deployContract();
