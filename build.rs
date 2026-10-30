extern crate napi_build;

use std::path::Path;

fn main() {
    napi_build::setup();

    // Expose the npm package version (from package.json) so the client can report
    // it in its user agent. The Cargo package version is kept at 0.0.0 by napi, so
    // CARGO_PKG_VERSION is not usable here. The release artifacts are built via
    // `yarn build` from a checkout whose package.json holds the release version,
    // so reading it here picks up the real version regardless of the build env.
    // There is deliberately no fallback: silently reporting 0.0.0 is the bug this
    // fixes, so a missing or malformed package.json fails the build instead.
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by cargo");
    let package_json = Path::new(&manifest_dir).join("package.json");

    // Required, not just an optimisation: napi_build emits rerun-if-env-changed
    // directives, which disables cargo's default rerun on any file change.
    println!("cargo:rerun-if-changed={}", package_json.display());

    let contents = std::fs::read_to_string(&package_json)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", package_json.display()));
    let pkg: serde_json::Value = serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", package_json.display()));
    let version = pkg
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or_else(|| panic!("no string \"version\" field in {}", package_json.display()));

    println!("cargo:rustc-env=NPM_PKG_VERSION={version}");
}
