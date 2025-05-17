import { describe, expect, test } from "bun:test";
import { AuthBuilder, AuthType } from "./index";

describe("AuthBuilder", () => {
	test("should construct with default values", () => {
		const builder = new AuthBuilder();
		expect(builder).toBeDefined();
	});

	test("should update client_id", () => {
		const builder = new AuthBuilder()
			.setClientId("test-client-id");
		expect(builder.getSelf()).toMatchObject({ client_id: "test-client-id" });
	});

	test("should return OAuth URL in get_info", async () => {
		const builder = new AuthBuilder()
			.setType(AuthType.Oauth)
			.setClientId("mock-client-id");
		const info = await builder.getInfo();
		expect(info.ouathUrl).toContain("https://login.microsoftonline.com");
	});

	test("should return device code in get_info", async () => {
		const builder = new AuthBuilder()
			.setType(AuthType.DeviceCode)
			.setClientId("mock-device-id");
		const info = await builder.getInfo();
		expect(info.deviceCode).toBeDefined();
	});
});

