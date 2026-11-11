import { defaultSettings } from "@/stores/convertSettings";
import { makeItem } from "@/test/factories";
import { buildJob, buildResize, savingsPercent } from "./job";

describe("buildJob", () => {
  it("mapea ajustes al contrato del backend", () => {
    const job = buildJob(makeItem(1), {
      ...defaultSettings,
      outputDir: "/salida",
      format: "jpeg",
      quality: 70,
      resizeKind: "box",
      width: 300,
      height: 200,
      fit: "cover",
    });
    expect(job).toMatchObject({
      id: "/fotos/img 1.png",
      output_directory: "/salida",
      output_format: "jpeg",
      quality_value: 70,
      resize: { kind: "box", width: 300, height: 200, fit: "cover" },
      conflict_policy: "rename",
      metadata_policy: "strip",
    });
  });

  it("PNG siempre va sin pérdida", () => {
    const job = buildJob(makeItem(1), { ...defaultSettings, format: "png", qualityMode: "lossy" });
    expect(job.quality_mode).toBe("lossless");
  });

  it("resize por tipo", () => {
    expect(buildResize({ ...defaultSettings, resizeKind: "none" })).toEqual({ kind: "none" });
    expect(buildResize({ ...defaultSettings, resizeKind: "percent", percent: 25 })).toEqual({
      kind: "percent",
      value: 25,
    });
  });
});

describe("savingsPercent", () => {
  it("calcula ahorro y aumento", () => {
    expect(savingsPercent(1000, 250)).toBe(75);
    expect(savingsPercent(1000, 1200)).toBe(-20);
    expect(savingsPercent(0, 10)).toBe(0);
  });
});
