import { ContractPromise } from "@polkadot/api-contract";
import { step } from "mocha-steps";
import { expect } from "chai";
import { WeightV2 } from "@polkadot/types/interfaces/runtime";
import dotenv from "dotenv-flow";
import { QueryArgs } from "../../interfaces/core";
import { CHAINS, CONTRACTS } from "../../constants";
import { getMetadata } from "../../services";
import { describeWithContext } from "../utils";
import { parseChainFromArgs } from "./helpers";

dotenv.config();

const GAS_LIMIT = "10000000000000";

const QUERY_TIMEOUT = 30_000;

const STATE_TEST_CTX_METADATA = getMetadata(CONTRACTS.stateTestCtx.metadataPath);
const chain = parseChainFromArgs(process.argv);

describeWithContext(
	"\n\n👉 Tests for contracts after network upgrade",
	(context) => {
		step("🌟 Ensure the contract state is intact", async function (done) {
			try {
				this.timeout(QUERY_TIMEOUT);

				const ctxObj = new ContractPromise(
					context.api!,
					STATE_TEST_CTX_METADATA,
					CHAINS[chain].UPGRADE_CTX_ADDRESS,
				);

				const queryOptions: QueryArgs = {
					sender: context.alice!.address,
					args: [],
					txOptions: {
						gasLimit: context.api!.registry.createType("WeightV2", {
							proofSize: GAS_LIMIT,
							refTime: GAS_LIMIT,
						}) as WeightV2,
						storageDepositLimit: null,
					},
				};

				// @ts-ignore
				const { output } = await context.queryContract(
					ctxObj,
					CONTRACTS.stateTestCtx.readMethods.get,
					queryOptions,
				);

				// todo: CRITICAL - Update the expected string accordingly
				// todo: https://github.com/polkadot-js/api/issues/5483
				// expect(output?.toString(), "Invalid contract state").to.equal(
				// 	'{"msg":"ICE/SNOW Network","u8Arr":"0x010203","value":"0x7fffffffffffffffffffffffffffffff","isTrue":true,"myAccount":"npRm3oLNUahbPgUnwjPYw9oLWdEigk6aHzwtvj4nibfAJMYVo","myBalance":123123123123,"myHash":"0xe0d83c067d9abf593a8089ef1f21fc30fafb02a8dd67a862f8ca47eb158735b9","myVec":[1,2,3,4,5,6,7,8,9,10,11],"myStruct":{"id":1,"status":"Invalid","strArr":["Str1","Str2"]},"myIntStruct":{"myU8":255,"myU16":65535,"myU32":4294967295,"myU64":"0xffffffffffffffff","myU128":"0xffffffffffffffffffffffffffffffff","myI8":127,"myI16":32767,"myI32":2147483647,"myI64":"0x7fffffffffffffff","myI128":"0x7fffffffffffffffffffffffffffffff"}}',
				// );

				done();
			} catch (err) {
				done(err);
			}
		});
	},
	chain,
);
