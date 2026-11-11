import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { api, onBatchProgress } from "@/lib/tauri";
import { useBatchStore } from "@/stores/batch";
import { defaultSettings, useConvertSettings } from "@/stores/convertSettings";
import { useImagesStore } from "@/stores/images";
import { makeItem } from "@/test/factories";
import type { BatchProgress, BatchReport, JobResult } from "@/types/ipc";
import { BatchBar } from "./BatchBar";
import { BatchResults } from "./BatchResults";

const res = (i: number, over: Partial<JobResult> = {}): JobResult => ({
  id: makeItem(i).id,
  status: "completed",
  output_path: `/salida/${i}.webp`,
  width: 1,
  height: 1,
  input_bytes: 1000,
  output_bytes: 400,
  duration_ms: 1,
  error: null,
  ...over,
});

const summary = (over = {}) => ({
  total: 3,
  completed: 2,
  failed: 1,
  skipped: 0,
  cancelled: 0,
  input_bytes: 2000,
  output_bytes: 800,
  saved_bytes: 1200,
  duration_ms: 10,
  was_cancelled: false,
  ...over,
});

beforeEach(() => {
  vi.clearAllMocks();
  act(() => {
    useImagesStore.getState().clear();
    useImagesStore.getState().addItems([makeItem(1), makeItem(2), makeItem(3)], []);
    useBatchStore.setState({ running: false, cancelling: false, summary: null, total: 0, done: 0 });
    useConvertSettings.setState({ ...defaultSettings, outputDir: "/salida" });
  });
});

describe("lote", () => {
  it("lanza el lote, refleja el progreso y publica el resumen", async () => {
    let emit: (p: BatchProgress) => void = () => {};
    vi.mocked(onBatchProgress).mockImplementationOnce(async (cb) => {
      emit = cb;
      return () => {};
    });
    let finish: (r: BatchReport) => void = () => {};
    vi.mocked(api.startBatch).mockReturnValueOnce(new Promise((r) => (finish = r)));

    render(<BatchBar />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir todo \(3\)/ }));
    expect(api.startBatch).toHaveBeenCalledTimes(1);
    const [jobs] = vi.mocked(api.startBatch).mock.calls[0];
    expect(jobs).toHaveLength(3);

    act(() => emit({ total: 3, done: 1, started: [makeItem(2).id], finished: [res(1)] }));
    expect(screen.getByRole("progressbar")).toHaveAttribute("aria-valuenow", "33");
    const st = useImagesStore.getState().entries;
    expect(st[0].status).toBe("completed");
    expect(st[1].status).toBe("processing");

    await act(async () =>
      finish({
        summary: summary(),
        results: [
          res(1),
          res(2),
          res(3, { status: "failed", error: { code: "corrupt", detail: null } }),
        ],
      }),
    );
    expect(useBatchStore.getState().running).toBe(false);
    expect(useImagesStore.getState().entries[2].status).toBe("failed");
  });

  it("cancelar llama al backend y marca 'cancelando'", async () => {
    vi.mocked(api.startBatch).mockReturnValueOnce(new Promise(() => {}));
    render(<BatchBar />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir todo/ }));
    await userEvent.click(await screen.findByRole("button", { name: "Cancelar" }));
    expect(api.cancelBatch).toHaveBeenCalled();
    expect(await screen.findByText("Cancelando…")).toBeInTheDocument();
  });

  it("sin carpeta de salida no se puede iniciar", () => {
    act(() => useConvertSettings.setState({ outputDir: "" }));
    render(<BatchBar />);
    expect(screen.getByRole("button", { name: /Convertir todo/ })).toBeDisabled();
  });

  it("sobrescribir exige doble confirmación", async () => {
    act(() => useConvertSettings.setState({ conflict: "overwrite" }));
    vi.mocked(api.startBatch).mockResolvedValueOnce({
      summary: summary({ total: 3 }),
      results: [],
    });
    render(<BatchBar />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir todo/ }));
    expect(api.startBatch).not.toHaveBeenCalled();
    await userEvent.click(screen.getByRole("button", { name: "Confirmar sobrescritura" }));
    await waitFor(() => expect(api.startBatch).toHaveBeenCalled());
    expect(vi.mocked(api.startBatch).mock.calls[0][1]).toBe(true);
  });

  it("si el comando falla, el lote termina y las imágenes quedan con error", async () => {
    vi.mocked(api.startBatch).mockRejectedValueOnce(new Error("x"));
    render(<BatchBar />);
    await userEvent.click(screen.getByRole("button", { name: /Convertir todo/ }));
    await waitFor(() => expect(useBatchStore.getState().running).toBe(false));
    expect(useImagesStore.getState().entries.every((e) => e.status === "failed")).toBe(true);
  });
});

describe("resultados", () => {
  it("muestra conteos, ahorro, errores y abre la carpeta", async () => {
    act(() => {
      useImagesStore
        .getState()
        .setResult(
          makeItem(3).id,
          res(3, { status: "failed", error: { code: "corrupt", detail: null } }),
        );
      useBatchStore.setState({ summary: summary(), outputDir: "/salida" });
    });
    render(<BatchResults />);
    expect(screen.getByText(/60%/)).toBeInTheDocument();
    expect(screen.getByText(/Archivo corrupto/)).toBeInTheDocument();
    await userEvent.click(screen.getByRole("button", { name: /Abrir carpeta de destino/ }));
    expect(api.revealFolder).toHaveBeenCalledWith("/salida");
    await userEvent.click(screen.getByRole("button", { name: "Cerrar" }));
    expect(useBatchStore.getState().summary).toBeNull();
  });

  it("indica cancelación", () => {
    act(() => useBatchStore.setState({ summary: summary({ was_cancelled: true, cancelled: 1 }) }));
    render(<BatchResults />);
    expect(screen.getByText("Lote cancelado")).toBeInTheDocument();
  });
});
