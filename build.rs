extern crate napi_build;

fn main() {
    napi_build::setup();

    // Expose the npm package version (from package.json) so the client can report
    // it in its user agent. The Cargo package version is kept at 0.0.0 by napi, so
    // CARGO_PKG_VERSION is not usable here. The release artifacts are built via
    // `yarn build` from a checkout whose package.json holds the release version,
    // so reading it here picks up the real version regardless of the build env.
    // Falls back to CARGO_PKG_VERSION if package.json can't be read.
    let version = std::fs::read_to_string("package.json")
        .ok()
        .and_then(|pkg| {
            pkg.lines().find_map(|line| {
                line.trim()
                    .strip_prefix("\"version\":")
                    .map(|rest| {
                        rest.trim()
                            .trim_matches(|c| c == ',' || c == '"' || c == ' ')
                            .to_string()
                    })
            })
        })
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    println!("cargo:rustc-env=NPM_PKG_VERSION={version}");
    println!("cargo:rerun-if-changed=package.json");
}
