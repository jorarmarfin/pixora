#!/usr/bin/env node
/* global process, console */
// Verifica que package.json, Cargo.toml y tauri.conf.json tengan la misma versión
// (y, opcionalmente, que coincida con un tag: `node scripts/check-versions.mjs v1.2.3`).
import { readFileSync } from "node:fs";

const pkg = JSON.parse(readFileSync("package.json", "utf8")).version;
const tauri = JSON.parse(readFileSync("src-tauri/tauri.conf.json", "utf8")).version;
const cargo = readFileSync("src-tauri/Cargo.toml", "utf8").match(/^version\s*=\s*"([^"]+)"/m)?.[1];

const found = { "package.json": pkg, "Cargo.toml": cargo, "tauri.conf.json": tauri };
const tag = process.argv[2]?.replace(/^v/, "");
if (tag) found["tag"] = tag;

const unique = new Set(Object.values(found));
if (unique.size !== 1 || !/^\d+\.\d+\.\d+(-[\w.]+)?$/.test([...unique][0] ?? "")) {
  console.error("Versiones inconsistentes o inválidas:", found);
  process.exit(1);
}
console.log(`Versión ${[...unique][0]} consistente en ${Object.keys(found).join(", ")}`);
