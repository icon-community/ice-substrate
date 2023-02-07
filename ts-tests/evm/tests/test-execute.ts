import { assert, expect } from "chai";
import { step } from "mocha-steps";
import { BLOCK_GAS_LIMIT, GENESIS_ACCOUNT } from "./config";

import { describeWithIce, customRequest } from "./util";

import Test from "../build/contracts/Test.json";

const TEST_CONTRACT_BYTECODE = Test.bytecode;
const TEST_CONTRACT_DEPLOYED_BYTECODE = Test.deployedBytecode;

describeWithIce("Ice RPC (RPC execution)", (context) => {
	step("should call with gas limit under block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_call", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${BLOCK_GAS_LIMIT.toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect(result.result).to.be.equal(TEST_CONTRACT_DEPLOYED_BYTECODE);
	});

	step("should call with gas limit up to 10x block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_call", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${(BLOCK_GAS_LIMIT * 10).toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect(result.result).to.be.equal(TEST_CONTRACT_DEPLOYED_BYTECODE);
	});

	step("shouldn't call with gas limit up higher than 10x block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_call", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${(BLOCK_GAS_LIMIT * 10 + 1).toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect((result as any).error.message).to.be.equal(
			"provided gas limit is too high (can be up to 10x the block gas limit)"
		);
	});

	step("should estimateGas with gas limit under block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_estimateGas", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${BLOCK_GAS_LIMIT.toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect(result.result).to.be.equal("0x2f389");
	});

	step("should estimateGas with gas limit up to 10x block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_estimateGas", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${(BLOCK_GAS_LIMIT * 10).toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect(result.result).to.be.equal("0x2f389");
	});

	step("shouldn't estimateGas with gas limit up higher than 10x block gas limit", async function () {
		const result = await customRequest(context.web3, "eth_estimateGas", [
			{
				from: GENESIS_ACCOUNT,
				gas: `0x${(BLOCK_GAS_LIMIT * 20 + 1).toString(16)}`,
				data: TEST_CONTRACT_BYTECODE,
			},
		]);

		expect(result.result).to.not.exist;
		expect((result as any).error.message).to.be.equal(
			"provided gas limit is too high (can be up to 10x the block gas limit)"
		);
	});
});
