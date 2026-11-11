import { defaultSettings } from "@/stores/convertSettings";
import { makeItem } from "@/test/factories";
import { buildJob, buildResize, modeIssue, savingsPercent } from "./job";

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

describe("modos Comprimir y Redimensionar", () => {
  const base = { ...defaultSettings, format: "webp" as const, resizeKind: "width" as const };

  it("Comprimir conserva el formato de cada imagen y no redimensiona", () => {
    const jpg = makeItem(1, { format: "jpeg" });
    const job = buildJob(jpg, base, "optimize");
    expect(job.output_format).toBe("jpeg");
    expect(job.resize).toEqual({ kind: "none" });
    expect(buildJob(makeItem(2, { format: "png" }), base, "optimize").quality_mode).toBe(
      "lossless",
    );
  });

  it("Redimensionar conserva el formato y aplica el tamaño", () => {
    const job = buildJob(makeItem(1, { format: "png" }), { ...base, width: 640 }, "resize");
    expect(job.output_format).toBe("png");
    expect(job.resize).toEqual({ kind: "width", px: 640 });
  });

  it("Convertir usa el formato elegido", () => {
    expect(buildJob(makeItem(1, { format: "png" }), base, "convert").output_format).toBe("webp");
  });

  it("Redimensionar exige elegir un tamaño", () => {
    expect(modeIssue("resize", { ...base, resizeKind: "none" })).toBe("process.needResize");
    expect(modeIssue("resize", base)).toBeNull();
    expect(modeIssue("optimize", { ...base, resizeKind: "none" })).toBeNull();
  });
});
