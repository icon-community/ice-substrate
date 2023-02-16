import { Contract, ContractFactory, Signer, Wallet } from "ethers";
import { expect } from "chai";
import { step } from "mocha-steps";
import dotenv from "dotenv";

import { getEthersProvider, parseChainFromArgs } from "./helpers";
import SetterGetterCtx from "../../build/contracts/SetterGetter.json";

dotenv.config();

const chainName = parseChainFromArgs(process.argv);
const EVM_CTX_DEPLOYER_KEY = process.env["EVM_CTX_DEPLOYER_KEY"];
let ctxAddress: string | null = null;

describe("Tests for deploying and calling write method on a contract", () => {

	const deployer: Signer = new Wallet(EVM_CTX_DEPLOYER_KEY, getEthersProvider(chainName));

	step("Deploy SetterGetter contract successfully", async function (done) {
		try {
			this.timeout(20_000);

			const factory = new ContractFactory(SetterGetterCtx.abi, SetterGetterCtx.bytecode, deployer);

			console.log(`Deploying contract to: ${chainName}`);
			console.log("Deployer wallet: ", await deployer.getAddress());

			const contract: Contract = await factory.deploy();
			await contract.deployed(); //wait until deployed

			ctxAddress = contract.address;
            done();
		} catch (e) {
			done(e);
		}
	});

	step("Ensure the write method can be called in the contract", async function (done) {
		try {
			this.timeout(20_000);

			const contract: Contract = new Contract(ctxAddress, SetterGetterCtx.abi, deployer);

			const tx = await contract.store(3);
            await tx.wait();

			const num = await contract.retrieve();

			expect(num).to.equal(3);
			done();
		} catch (e) {
			done(e);
		}
	});
});
