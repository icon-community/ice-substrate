import { ContractPromise } from "@polkadot/api-contract";
import { step } from "mocha-steps";
import { expect } from "chai";
import { QueryArgs } from "../../interfaces/core";
import { CONTRACTS } from "../../constants";
import { getMetadata } from "../../services";
import { describeWithContext } from "../utils";

const GAS_LIMIT = "10000000000000";

const QUERY_TIMEOUT = 30_000;

const STATE_CHECK_CTX_METADATA = getMetadata(CONTRACTS.stateCheckCtx.metadataPath);
const STATE_CHECK_CTX_ADDRESS = process.env["STATE_CHECK_CTX_ADDRESS"];

describeWithContext(
	"\n\n👉 Tests for contracts after network upgrade",
	(context) => {
		step("Ensure the contract state is intact", async function (done) {
			this.timeout(QUERY_TIMEOUT);

			const ctxObj = new ContractPromise(context.api!, STATE_CHECK_CTX_METADATA, STATE_CHECK_CTX_ADDRESS!);

			const queryOptions: QueryArgs = {
				sender: context.alice!.address,
				args: [],
				txOptions: { gasLimit: GAS_LIMIT, storageDepositLimit: null },
			};

			// @ts-ignore
			const { output } = await context.queryContract(
				ctxObj,
				CONTRACTS.stateCheckCtx.readMethods.get,
				queryOptions,
			);

			expect(output?.toString(), "Invalid contract state").to.equal(
				'{"msg":"SNOW","hash":"0x6464646464646464646464646464646464646464646464646464646464646464","value":100,"structure":{"val":100,"name":"SNOW"}}',
			);

			done();
		});
	},
	true,
);
