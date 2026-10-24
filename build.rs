use std::env;
use std::error::Error;
use std::path::PathBuf;

const WRAPPER: &str = "wrapper.h";
const BINDINGS: &str = "bindings.rs";
const MINRX_PATH: &str = "vendor/minrx";
/// Compatibility headers for platforms that lack them (see `shim/langinfo.h`).
const WINDOWS_SHIM_PATH: &str = "shim";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={MINRX_PATH}");
    println!("cargo:rerun-if-changed={WINDOWS_SHIM_PATH}");

    let is_windows = env::var_os("CARGO_CFG_WINDOWS").is_some();

    let bindings = bindgen::Builder::default()
        .header(WRAPPER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join(BINDINGS))?;

    let mut build = cc::Build::new();
    build.warnings(false).file(format!("{MINRX_PATH}/minrx.c"));

    // MinRX includes <langinfo.h> for `nl_langinfo(CODESET)`, which MSVC and
    // MinGW don't provide.
    if is_windows {
        build.include(WINDOWS_SHIM_PATH);
    }

    build.compile("minrx");

    Ok(())
}
