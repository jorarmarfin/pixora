import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { api } from "@/lib/tauri";
import { useAppConfig } from "@/stores/appConfig";
import { defaultSettings, useConvertSettings } from "@/stores/convertSettings";
import type { LoadedConfig, Preset } from "@/types/ipc";
import { PrefsView } from "@/features/settings/PrefsView";
import { PresetPicker } from "./PresetPicker";
import { PresetsView } from "./PresetsView";

const thumbs: Preset = {
  id: "thumbnails",
  name: "Miniaturas",
  builtin: true,
  settings: {
    format: "webp",
    quality_mode: "lossy",
    quality: 75,
    resize_kind: "box",
    width: 320,
    height: 320,
    percent: 50,
    fit: "cover",
    metadata: "strip",
  },
};
const web: Preset = {
  ...thumbs,
  id: "web",
  name: "Web optimizada",
  settings: { ...thumbs.settings, quality: 80, resize_kind: "width", width: 1920 },
};

const loaded = (over: Partial<LoadedConfig> = {}): LoadedConfig => ({
  version: 1,
  theme: "dark",
  concurrency: 3,
  default_output_dir: "",
  last_preset_id: null,
  presets: [],
  builtin_presets: [web, thumbs],
  recovered: false,
  ...over,
});

beforeEach(() => {
  vi.clearAllMocks();
  document.documentElement.className = "dark";
  act(() => {
    useConvertSettings.setState({ ...defaultSettings });
    useAppConfig.setState({ userPresets: [], builtinPresets: [], theme: "dark", concurrency: 3 });
  });
});

describe("config", () => {
  it("carga, aplica tema, carpeta por defecto y último preset", async () => {
    vi.mocked(api.loadConfig).mockResolvedValueOnce(
      loaded({ theme: "light", default_output_dir: "/out", last_preset_id: "thumbnails" }),
    );
    await act(() => useAppConfig.getState().load());
    expect(document.documentElement.classList.contains("dark")).toBe(false);
    const ui = useConvertSettings.getState();
    expect(ui.outputDir).toBe("/out");
    expect(ui.resizeKind).toBe("box");
    expect(ui.width).toBe(320);
  });

  it("si falla la carga usa valores por defecto sin romper", async () => {
    vi.mocked(api.loadConfig).mockRejectedValueOnce(new Error("x"));
    await act(() => useAppConfig.getState().load());
    expect(useAppConfig.getState().loaded).toBe(true);
    expect(useAppConfig.getState().theme).toBe("dark");
  });
});

describe("selector de presets", () => {
  beforeEach(() => act(() => useAppConfig.setState({ builtinPresets: [web, thumbs] })));

  it("aplica un preset y lo detecta; al editar pasa a Personalizado", async () => {
    render(<PresetPicker />);
    await userEvent.selectOptions(screen.getByLabelText("Preset"), "thumbnails");
    expect(useConvertSettings.getState().width).toBe(320);
    expect(screen.getByLabelText("Preset")).toHaveValue("thumbnails");
    expect(api.saveConfig).toHaveBeenCalled();

    act(() => useConvertSettings.getState().patch({ quality: 10 }));
    expect(screen.getByLabelText("Preset")).toHaveValue("custom");
  });

  it("guarda los ajustes actuales como preset propio", async () => {
    act(() => useConvertSettings.getState().patch({ quality: 42 }));
    render(<PresetPicker />);
    await userEvent.click(screen.getByRole("button", { name: /Guardar como preset/ }));
    await userEvent.type(screen.getByLabelText("Nombre del preset"), "Mi preset{Enter}");
    await waitFor(() => expect(useAppConfig.getState().userPresets).toHaveLength(1));
    const saved = vi.mocked(api.saveConfig).mock.lastCall![0];
    expect(saved.presets[0]).toMatchObject({ name: "Mi preset", settings: { quality: 42 } });
    expect(screen.getByLabelText("Preset")).toHaveValue(useAppConfig.getState().userPresets[0].id);
  });
});

describe("vista de presets", () => {
  it("lista integrados y propios; solo los propios se eliminan", async () => {
    const mine: Preset = { ...web, id: "u1", name: "Propio", builtin: false };
    act(() => useAppConfig.setState({ builtinPresets: [web], userPresets: [mine] }));
    render(<PresetsView />);
    expect(screen.getAllByRole("button", { name: /Eliminar/ })).toHaveLength(1);
    await userEvent.click(screen.getByRole("button", { name: "Eliminar Propio" }));
    expect(useAppConfig.getState().userPresets).toHaveLength(0);
    expect(screen.getByText(/Aún no tienes presets propios/)).toBeInTheDocument();
  });
});

describe("ajustes", () => {
  it("cambia tema y concurrencia y los persiste", async () => {
    render(<PrefsView />);
    await userEvent.selectOptions(screen.getByLabelText("Tema"), "light");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
    expect(vi.mocked(api.saveConfig).mock.lastCall![0].theme).toBe("light");
    act(() => void useAppConfig.getState().update({ concurrency: 5 }));
    expect(vi.mocked(api.saveConfig).mock.lastCall![0].concurrency).toBe(5);
  });

  it("avisa si la configuración fue restaurada", () => {
    act(() => useAppConfig.setState({ recovered: true }));
    render(<PrefsView />);
    expect(screen.getByRole("alert")).toHaveTextContent(/dañada/);
  });
});
