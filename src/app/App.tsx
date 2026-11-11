import { Sidebar } from "@/components/Sidebar";
import { EmptyState } from "@/features/import/EmptyState";

export default function App() {
  return (
    <div className="dark flex h-screen bg-background text-foreground">
      <Sidebar />
      <main className="flex-1 p-6">
        <EmptyState />
      </main>
    </div>
  );
}
