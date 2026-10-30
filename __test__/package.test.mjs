// Consistency checks between the committed generated files, package
// metadata and the built binding.
import test from "ava";
import { readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const read = (p) => readFileSync(new URL(`../${p}`, import.meta.url), "utf8");
const pkg = JSON.parse(read("package.json"));
const dts = read("index.d.ts");
const lib = require("../index.js");

// #63: setLogLevel existed in Rust but index.js/index.d.ts weren't regenerated.
test("runtime exports match index.d.ts", (t) => {
  const values = [...dts.matchAll(/^export declare (?:class|function|enum) (\w+)/gm)].map((m) => m[1]);
  // String enums are emitted as `export type X = 'A' | ...` plus a runtime object.
  const stringEnums = [...dts.matchAll(/^export type (\w+) =\s*(?:\/\*[\s\S]*?\*\/\s*)*'/gm)].map((m) => m[1]);
  t.deepEqual(Object.keys(lib).sort(), [...values, ...stringEnums].sort());
  t.is(typeof lib.setLogLevel, "function");
});

// #30: `const enum` declarations are erased by TypeScript, leaving no runtime value.
test("no const enums in index.d.ts", (t) => {
  t.notRegex(dts, /const enum/);
});

// #47 and release bumps: the platform packages and index.js's version check
// must all agree with package.json.
test("versions are consistent across packages", (t) => {
  const indexJsVersions = new Set([...read("index.js").matchAll(/bindingPackageVersion !== '([^']+)'/g)].map((m) => m[1]));
  t.deepEqual([...indexJsVersions], [pkg.version]);

  const platformDirs = readdirSync(new URL("../npm", import.meta.url));
  t.true(platformDirs.length > 0);
  for (const dir of platformDirs) {
    t.is(JSON.parse(read(`npm/${dir}/package.json`)).version, pkg.version, `npm/${dir}`);
  }
});
