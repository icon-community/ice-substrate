import { Contract, BigNumber } from "ethers";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ContractResponse } from "./interfaces";
import { ethersProvider } from "./api";
import UpgradeCtx from "../../build/contracts/NetworkUpgrade.json";
import { SNOW_UPGRADE_CTX_ADDRESS } from "../config";

describe("Tests for checking existing contracts storage", () => {
	step("Ensure the contract state is intact", async function (done) {
		this.timeout(10_000);

		const contract: Contract = new Contract(SNOW_UPGRADE_CTX_ADDRESS, UpgradeCtx.abi, ethersProvider);

		const { number, message, testStruct, testArray }: ContractResponse = await contract.get();

		expect(number.toNumber(), "Returned an invalid number").to.equal(20);
		expect(message, "Returned an invalid string").to.equal("SNOW Network");
		expect(testStruct.num.toNumber(), "Returned an invalid struct").to.equal(20);
		expect(testStruct.message, "Returned an invalid struct").to.equal("SNOW Network");
		expect(testArray, "Returned an invalid array").to.eql([BigNumber.from(20), BigNumber.from(40)]);

		done();
	});
});
