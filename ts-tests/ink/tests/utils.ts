import { SnowApi } from "../services";
import { describe, before } from "mocha";

const INIT_TIMEOUT = 20_000; // milli sec

export function describeWithContext(title: string, callback: (context: typeof SnowApi) => void, isMainnet?: boolean) {
	describe(title, () => {
		const context = SnowApi;
		before("Initilializing test env", async function () {
			this.timeout(INIT_TIMEOUT);

			await context.initialize(isMainnet);
		});

		callback(context);

		after(async function () {
			SnowApi.cleanUp();
		});
	});
}
