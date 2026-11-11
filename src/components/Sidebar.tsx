import { ArrowRightLeft, Minimize2, Scaling, Wrench, Sparkles, Settings } from "lucide-react";
import { cn } from "@/lib/utils";
import { t, type MessageKey } from "@/lib/i18n";
import { useUiStore, type SectionId } from "@/stores/ui";

const items: { id: SectionId; icon: typeof Settings; label: MessageKey }[] = [
  { id: "convert", icon: ArrowRightLeft, label: "nav.convert" },
  { id: "optimize", icon: Minimize2, label: "nav.optimize" },
  { id: "resize", icon: Scaling, label: "nav.resize" },
  { id: "tools", icon: Wrench, label: "nav.tools" },
  { id: "presets", icon: Sparkles, label: "nav.presets" },
  { id: "settings", icon: Settings, label: "nav.settings" },
];

export function Sidebar() {
  const { section, setSection } = useUiStore();
  return (
    <aside className="flex w-56 shrink-0 flex-col gap-6 border-r border-border bg-card p-4">
      <div className="flex items-center gap-3 px-2">
        <img src="/logo.png" alt="" className="size-9 rounded-xl" />
        <span className="text-xl font-bold tracking-tight">{t("app.name")}</span>
      </div>
      <nav aria-label={t("nav.main")} className="flex flex-col gap-1">
        {items.map(({ id, icon: Icon, label }) => (
          <button
            key={id}
            type="button"
            onClick={() => setSection(id)}
            aria-current={section === id ? "page" : undefined}
            className={cn(
              "flex items-center gap-3 rounded-lg px-3 py-2 text-left text-sm font-medium transition hover:bg-muted",
              section === id && "bg-brand-gradient text-white hover:brightness-110",
            )}
          >
            <Icon className="size-4" aria-hidden />
            {t(label)}
          </button>
        ))}
      </nav>
      <p className="mt-auto px-2 text-xs text-muted-foreground">{t("app.tagline")}</p>
    </aside>
  );
}
