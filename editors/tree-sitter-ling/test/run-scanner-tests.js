const { mkdirSync } = require("node:fs");
const { resolve } = require("node:path");
const { spawnSync } = require("node:child_process");

const cacheDirectory = resolve(".cache");
const executable = resolve(
  cacheDirectory,
  process.platform === "win32" ? "scanner-state-tests.exe" : "scanner-state-tests",
);
const compiler = process.env.CC || (process.platform === "win32" ? "gcc" : "cc");

mkdirSync(cacheDirectory, { recursive: true });

const compile = spawnSync(
  compiler,
  [
    "-std=c11",
    "-Wall",
    "-Wextra",
    "-Werror",
    "-Isrc",
    "test/scanner_state.c",
    "-o",
    executable,
  ],
  { stdio: "inherit" },
);

if (compile.error) {
  throw compile.error;
}
if (compile.status !== 0) {
  process.exit(compile.status ?? 1);
}

const run = spawnSync(executable, [], { stdio: "inherit" });
if (run.error) {
  throw run.error;
}
process.exit(run.status ?? 1);
