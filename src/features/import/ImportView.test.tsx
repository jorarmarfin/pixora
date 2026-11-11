import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { api, pickImageFiles } from "@/lib/tauri";
import { t } from "@/lib/i18n";
import { useImagesStore } from "@/stores/images";
import { makeItem } from "@/test/factories";
import { ImportView } from "./ImportView";

beforeEach(() => {
  act(() => useImagesStore.getState().clear());
  vi.clearAllMocks();
});

describe("ImportView", () => {
  it("muestra estado vacío", () => {
    render(<ImportView />);
    expect(screen.getByText(t("views.emptyTitle"))).toBeInTheDocument();
  });

  it("importa archivos elegidos y los lista con metadatos", async () => {
    vi.mocked(pickImageFiles).mockResolvedValueOnce(["/fotos/img 1.png"]);
    vi.mocked(api.importPaths).mockResolvedValueOnce({
      items: [makeItem(1)],
      skipped: [{ name: "roto.png", reason: "corrupt" }],
    });
    render(<ImportView />);
    await userEvent.click(screen.getByRole("button", { name: t("import.addFiles") }));
    expect(await screen.findByText("img 1.png")).toBeInTheDocument();
    expect(screen.getByText("800 × 600")).toBeInTheDocument();
    expect(screen.getByRole("alert")).toHaveTextContent("roto.png");
    expect(api.importPaths).toHaveBeenCalledWith(["/fotos/img 1.png"], true);
  });

  it("agregar 100 imágenes no renderiza 100 filas (virtualizado) y es rápido", async () => {
    render(<ImportView />);
    const items = Array.from({ length: 100 }, (_, i) => makeItem(i));
    const t0 = performance.now();
    act(() => {
      useImagesStore.getState().addItems(items, []);
    });
    await waitFor(() => expect(screen.getAllByRole("listitem").length).toBeGreaterThan(0));
    expect(performance.now() - t0).toBeLessThan(1000);
    expect(screen.getAllByRole("listitem").length).toBeLessThan(100);
    expect(screen.getByText(/100 imágenes/)).toBeInTheDocument();
  });

  it("quita una imagen", async () => {
    render(<ImportView />);
    act(() => {
      useImagesStore.getState().addItems([makeItem(1), makeItem(2)], []);
    });
    await userEvent.click(screen.getByRole("button", { name: /Quitar img 1\.png/ }));
    expect(screen.queryByText("img 1.png")).not.toBeInTheDocument();
    expect(screen.getByText("img 2.png")).toBeInTheDocument();
  });
});
