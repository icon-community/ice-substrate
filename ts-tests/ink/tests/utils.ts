import { SnowApi } from "../services";
import { describe, before } from "mocha";
import { CHAINS } from "../constants";

const INIT_TIMEOUT = 20_000; // milli sec

export function describeWithContext(
	title: string,
	callback: (context: typeof SnowApi) => void,
	chain?: keyof typeof CHAINS,
) {
	describe(title, () => {
		const context = SnowApi;
		before("Initilializing test env", async function () {
			this.timeout(INIT_TIMEOUT);

			await context.initialize(chain);
		});

		callback(context);

		after(async function () {
			SnowApi.cleanUp();
		});
	});
}
