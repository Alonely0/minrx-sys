use std::env;
use std::error::Error;
use std::path::PathBuf;

const WRAPPER: &str = "wrapper.h";
const BINDINGS: &str = "bindings.rs";
const MINRX_PATH: &str = "vendor/minrx";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={MINRX_PATH}");

    let bindings = bindgen::Builder::default()
        .header(WRAPPER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings.write_to_file(out_dir.join(BINDINGS))?;

    cc::Build::new()
        .warnings(false)
        .file(format!("{MINRX_PATH}/minrx.c"))
        .compile("minrx");

    Ok(())
}
