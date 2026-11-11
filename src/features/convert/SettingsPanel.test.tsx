import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { t } from "@/lib/i18n";
import { defaultSettings, useConvertSettings } from "@/stores/convertSettings";
import { SettingsPanel } from "./SettingsPanel";

beforeEach(() => useConvertSettings.setState(defaultSettings));

describe("SettingsPanel", () => {
  it("PNG deshabilita calidad y modo", async () => {
    render(<SettingsPanel />);
    await userEvent.selectOptions(screen.getByLabelText(t("settings.format")), "png");
    expect(screen.getByLabelText(t("settings.mode"))).toBeDisabled();
    expect(screen.getByLabelText(/Calidad/)).toBeDisabled();
  });

  it("muestra campos según el tipo de redimensión", async () => {
    render(<SettingsPanel />);
    expect(screen.queryByLabelText(t("settings.width"))).not.toBeInTheDocument();
    await userEvent.selectOptions(screen.getByLabelText(t("settings.resize")), "box");
    expect(screen.getByLabelText(t("settings.width"))).toBeInTheDocument();
    expect(screen.getByLabelText(t("settings.height"))).toBeInTheDocument();
    expect(screen.getByLabelText(t("settings.fit"))).toBeInTheDocument();
  });
});
