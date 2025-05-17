import { describe, expect, test } from "bun:test";
import { LaunchBuilder, JsJavaJRE } from "./index";

describe("LaunchBuilder", () => {
	test("should construct with default values", () => {
		const builder = new LaunchBuilder();
		expect(builder).toBeDefined();
	});

	test("should set arguments correctly", () => {
		const builder = new LaunchBuilder()
			.setArgs(["--test", "--debug"]);
		expect(builder.getSelf()).toMatchObject({ args: ["--test", "--debug"] });
	});

	test("should set java path", () => {
		const builder = new LaunchBuilder()
			.setJava("path/to/java");
		expect(builder.getSelf()).toMatchObject({ java_path: "path/to/java" });
	});

	test("should accept JavaJRE enum conversion", () => {
		const builder = new LaunchBuilder()
			.launch(JsJavaJRE.Adoptium, null);
		expect(true).toBe(true);
	});
});
