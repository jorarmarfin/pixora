import { useState } from "react";
import { Play } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useConvertOne } from "@/features/convert/useConvertOne";
import { t } from "@/lib/i18n";
import { modeIssue } from "@/lib/job";
import { useUiStore } from "@/stores/ui";
import { useConvertSettings } from "@/stores/convertSettings";
import type { QueueEntry } from "@/stores/images";

export function RowActions({ entry }: { entry: QueueEntry }) {
  const convert = useConvertOne();
  const outputDir = useConvertSettings((s) => s.outputDir);
  const conflict = useConvertSettings((s) => s.conflict);
  const section = useUiStore((s) => s.section);
  const issue = useConvertSettings((s) => modeIssue(section, s));
  const [armed, setArmed] = useState(false);
  const busy = entry.status === "processing" || entry.status === "validating";
  const needsConfirm = conflict === "overwrite";

  return (
    <Button
      size="sm"
      variant={armed ? "outline" : "default"}
      disabled={!outputDir || busy || !!issue}
      title={issue ? t(issue) : !outputDir ? t("process.needOutputDir") : undefined}
      aria-label={`${t("process.convert")} ${entry.item.name}`}
      onClick={() => {
        if (needsConfirm && !armed) return setArmed(true);
        setArmed(false);
        void convert(entry.item, needsConfirm);
      }}
      onBlur={() => setArmed(false)}
    >
      <Play className="size-3.5" aria-hidden />
      {armed ? t("process.confirmOverwrite") : t("process.convert")}
    </Button>
  );
}
