import { es, type Messages } from "@/locales/es";

type Path<T, P extends string = ""> = {
  [K in keyof T & string]: T[K] extends string
    ? `${P}${K}`
    : Path<T[K], `${P}${K}.`>;
}[keyof T & string];

export type MessageKey = Path<Messages>;

const catalog: Messages = es;

/** Traduce una clave tipada ("nav.convert"). Idioma único (es) por ahora. */
export function t(key: MessageKey): string {
  return key
    .split(".")
    .reduce<unknown>((acc, part) => (acc as Record<string, unknown>)[part], catalog) as string;
}
