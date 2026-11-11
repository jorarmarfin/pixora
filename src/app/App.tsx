import { Sidebar } from "@/components/Sidebar";
import { SettingsPanel } from "@/features/convert/SettingsPanel";
import { ImportView } from "@/features/import/ImportView";

export default function App() {
  return (
    <div className="dark flex h-screen bg-background text-foreground">
      <Sidebar />
      <main className="min-w-0 flex-1 p-6">
        <ImportView />
      </main>
      <SettingsPanel />
    </div>
  );
}
