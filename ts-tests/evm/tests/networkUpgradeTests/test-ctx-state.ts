import { Contract, BigNumber } from "ethers";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ContractResponse } from "./interfaces";
import { getEthersProvider, parseChainFromArgs } from "./helpers";
import UpgradeCtx from "../../build/contracts/NetworkUpgrade.json";
import { CHAINS } from "../config";

describe("Tests for checking existing contracts storage", () => {
	step("Ensure the contract state is intact", async function (done) {
		try {
			this.timeout(10_000);
			const chainName = parseChainFromArgs(process.argv);

			const contract: Contract = new Contract(
				CHAINS[chainName].UPGRADE_CTX_ADDRESS,
				UpgradeCtx.abi,
				getEthersProvider(chainName)
			);

			const {
				number,
				message,
				num_array,
				fixed_str_array,
				chain_id,
				addr,
				d,
				simple_struct,
				bytes_struct,
				message_bytes,
			}: ContractResponse = await contract.get();

			expect(number.toNumber(), "Returned an invalid number").to.equal(20);
			expect(message, "Returned an invalid string").to.equal("SNOW Network");
			expect(num_array, "Returned an invalid uint256 array").to.eql([BigNumber.from(20), BigNumber.from(40)]);
			expect(chain_id, "Returned an invalid mapping array").to.eql([552, 553]);
			expect(fixed_str_array, "Returned an invalid string array").to.eql(["ICE", "SNOW", "Arctic", "Frost"]);
			expect(addr, "Returned an invalid H160 address").to.equal("0x8eFcaF2C4eBbf88Bf07f3BB44a2869C4C675AD7A");
			expect(d, "Returned an invalid enum").to.equal(6);
			expect(message_bytes, "Returned an invalid bytes").to.equal("0x534e4f57204e6574776f726b");

			expect(bytes_struct.one_char, "Returned an invalid bytes struct").to.equal("0x61");
			expect(bytes_struct.three_char, "Returned an invalid bytes struct").to.equal("0x313233");
			expect(bytes_struct.four_char, "Returned an invalid bytes struct").to.equal("0x61316232");
			expect(bytes_struct.sixteen_char, "Returned an invalid bytes struct").to.equal(
				"0x21402324255e262a2829313233343536"
			);
			expect(bytes_struct.thirtytwo_char, "Returned an invalid bytes struct").to.equal(
				"0x6162636465666768696a6b6c6d6e6f707172737475767778797a313233343536"
			);

			expect(simple_struct.num1, "Returned an invalid simple struct").to.equal(127);
			expect(simple_struct.bt, "Returned an invalid simple struct").to.equal("0x31");
			expect(simple_struct.b1, "Returned an invalid simple struct").to.equal(true);
			expect(simple_struct.addr, "Returned an invalid simple struct").to.equal(
				"0x8eFcaF2C4eBbf88Bf07f3BB44a2869C4C675AD7A"
			);
			expect(simple_struct.name, "Returned an invalid simple struct").to.equal("SNOW");

			done();
		} catch (e) {
			done(e);
		}
	});
});
