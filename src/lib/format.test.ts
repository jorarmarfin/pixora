import { formatBytes } from "./format";

it("formatea bytes", () => {
  expect(formatBytes(512)).toBe("512 B");
  expect(formatBytes(1536)).toBe("1.5 KB");
  expect(formatBytes(5 * 1024 * 1024)).toBe("5.0 MB");
});
