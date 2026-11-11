import { defaultSettings, useConvertSettings } from "./convertSettings";

beforeEach(() => useConvertSettings.setState(defaultSettings));

it("PNG fuerza compresión sin pérdida", () => {
  useConvertSettings.getState().patch({ format: "png" });
  expect(useConvertSettings.getState().qualityMode).toBe("lossless");
});

it("por defecto no sobrescribe (renombra)", () => {
  expect(useConvertSettings.getState().conflict).toBe("rename");
});
