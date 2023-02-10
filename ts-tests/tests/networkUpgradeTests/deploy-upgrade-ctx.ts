import { Contract, ContractFactory, Signer, Wallet } from "ethers";
import dotenv from "dotenv";

import UpgradeCtx from "../../build/contracts/NetworkUpgrade.json";
import { parseChainFromArgs, getEthersProvider } from "./helpers";

dotenv.config();
const EVM_CTX_DEPLOYER_KEY = process.env["EVM_CTX_DEPLOYER_KEY"];
const chain = parseChainFromArgs(process.argv);

async function deployContract() {
	let deployer: Signer = new Wallet(EVM_CTX_DEPLOYER_KEY, getEthersProvider(chain));
	let factory = new ContractFactory(UpgradeCtx.abi, UpgradeCtx.bytecode, deployer);

	console.log(`Deploying contract to: ${chain}`);
	console.log("Deployer wallet: ", await deployer.getAddress());

	const contract: Contract = await factory.deploy(20, "SNOW Network");
	await contract.deployed(); //wait until deployed

	console.log("Contract address: ", contract.address);
	console.log("\nSuccessfully deployed contract");
}

deployContract();
