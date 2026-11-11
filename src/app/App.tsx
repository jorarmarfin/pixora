import { useEffect } from "react";
import { Sidebar } from "@/components/Sidebar";
import { SettingsPanel } from "@/features/convert/SettingsPanel";
import { ImportView } from "@/features/import/ImportView";
import { ToolsView } from "@/features/tools/ToolsView";
import { PresetsView } from "@/features/presets/PresetsView";
import { PrefsView } from "@/features/settings/PrefsView";
import { useAppConfig } from "@/stores/appConfig";
import { useUiStore } from "@/stores/ui";

export default function App() {
  const section = useUiStore((s) => s.section);
  useEffect(() => {
    void useAppConfig.getState().load();
  }, []);

  const page =
    section === "tools" ? (
      <ToolsView />
    ) : section === "settings" ? (
      <PrefsView />
    ) : section === "presets" ? (
      <PresetsView />
    ) : (
      <ImportView />
    );
  return (
    <div className="flex h-screen bg-background text-foreground">
      <Sidebar />
      <main className="min-w-0 flex-1 overflow-auto p-6">{page}</main>
      {section !== "settings" && section !== "presets" && section !== "tools" && <SettingsPanel />}
    </div>
  );
}
