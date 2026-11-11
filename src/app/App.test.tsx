import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "./App";
import { t } from "@/lib/i18n";

describe("App", () => {
  it("muestra la barra lateral y el estado vacío", () => {
    render(<App />);
    expect(screen.getByRole("navigation", { name: "Principal" })).toBeInTheDocument();
    for (const k of ["convert", "optimize", "resize", "tools", "presets", "settings"] as const) {
      expect(screen.getByRole("button", { name: t(`nav.${k}`) })).toBeInTheDocument();
    }
    expect(screen.getByText(t("views.emptyTitle"))).toBeInTheDocument();
  });

  it("marca la sección activa", async () => {
    render(<App />);
    const btn = screen.getByRole("button", { name: t("nav.resize") });
    await userEvent.click(btn);
    expect(btn).toHaveAttribute("aria-current", "page");
  });
});
