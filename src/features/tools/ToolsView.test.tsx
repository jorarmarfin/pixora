import { render, screen } from "@testing-library/react";
import { ToolsView } from "./ToolsView";

it("muestra el aviso de en construcción", () => {
  render(<ToolsView />);
  expect(screen.getByRole("heading", { name: "En construcción" })).toBeInTheDocument();
});
