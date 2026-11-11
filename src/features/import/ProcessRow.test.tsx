import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { api } from "@/lib/tauri";
import { t } from "@/lib/i18n";
import { defaultSettings, useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import { makeItem } from "@/test/factories";
import type { JobResult } from "@/types/ipc";
import { ImageList } from "./ImageList";

const ok: JobResult = {
  id: makeItem(1).id,
  status: "completed",
  output_path: "/salida/img 1-pixora.webp",
  width: 800,
  height: 600,
  input_bytes: 2048,
  output_bytes: 512,
  duration_ms: 5,
  error: null,
};

beforeEach(() => {
  vi.clearAllMocks();
  act(() => {
    useImagesStore.getState().clear();
    useImagesStore.getState().addItems([makeItem(1)], []);
    useConvertSettings.setState({ ...defaultSettings, outputDir: "/salida" });
  });
});

describe("convertir una imagen", () => {
  it("envía el trabajo y muestra el ahorro", async () => {
    vi.mocked(api.processSingle).mockResolvedValueOnce(ok);
    render(<ImageList />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir img 1\.png/ }));
    expect(await screen.findByText(/75% ahorro/)).toBeInTheDocument();
    const [job, confirmed] = vi.mocked(api.processSingle).mock.calls[0];
    expect(job).toMatchObject({ output_directory: "/salida", output_format: "webp" });
    expect(confirmed).toBe(false);
  });

  it("muestra el error por archivo sin romper la lista", async () => {
    vi.mocked(api.processSingle).mockResolvedValueOnce({
      ...ok,
      status: "failed",
      error: { code: "corrupt", detail: null },
    });
    render(<ImageList />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir img 1\.png/ }));
    expect(await screen.findByText(t("jobError.corrupt"))).toBeInTheDocument();
    expect(screen.getByText("img 1.png")).toBeInTheDocument();
  });

  it("falla de IPC se reporta como error del motor", async () => {
    vi.mocked(api.processSingle).mockRejectedValueOnce(new Error("boom"));
    render(<ImageList />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir img 1\.png/ }));
    expect(await screen.findByText(t("jobError.engine"))).toBeInTheDocument();
  });

  it("sin carpeta de salida el botón está deshabilitado", () => {
    act(() => useConvertSettings.setState({ outputDir: "" }));
    render(<ImageList />);
    expect(screen.getByRole("button", { name: /Convertir img 1\.png/ })).toBeDisabled();
  });

  it("sobrescribir pide confirmación antes de enviar", async () => {
    act(() => useConvertSettings.setState({ conflict: "overwrite" }));
    vi.mocked(api.processSingle).mockResolvedValueOnce(ok);
    render(<ImageList />);
    const btn = screen.getByRole("button", { name: /Convertir img 1\.png/ });
    await userEvent.click(btn);
    expect(api.processSingle).not.toHaveBeenCalled();
    await userEvent.click(screen.getByRole("button", { name: /Convertir img 1\.png/ }));
    await waitFor(() => expect(api.processSingle).toHaveBeenCalledTimes(1));
    expect(vi.mocked(api.processSingle).mock.calls[0][1]).toBe(true);
  });
});
