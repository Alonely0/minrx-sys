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
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");
    println!("cargo:rerun-if-env-changed=WASI_SYSROOT");

    let is_windows = env::var_os("CARGO_CFG_WINDOWS").is_some();
    let is_wasi = env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "wasi");

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

    // MinRX reports errors through setjmp/longjmp. wasi-libc only offers those
    // when compiling with Wasm exception-handling based sjlj lowering, and the
    // runtime support lives in libsetjmp.a.
    if is_wasi {
        build.flag("-mllvm").flag("-wasm-enable-sjlj");
    }

    build.compile("minrx");

    if is_wasi {
        link_wasi_setjmp();
    }

    Ok(())
}

/// Link wasi-sdk's `libsetjmp.a` (`__wasm_setjmp`, `__wasm_longjmp`, ...).
///
/// The sysroot is taken from `WASI_SYSROOT`, falling back to
/// `$WASI_SDK_PATH/share/wasi-sysroot`; `cc` itself already uses
/// `$WASI_SDK_PATH/bin/<target>-clang` as the compiler when it is set.
fn link_wasi_setjmp() {
    let sysroot = env::var_os("WASI_SYSROOT").map(PathBuf::from).or_else(|| {
        env::var_os("WASI_SDK_PATH").map(|p| PathBuf::from(p).join("share/wasi-sysroot"))
    });

    match sysroot {
        Some(sysroot) => {
            let target = env::var("TARGET").unwrap_or_default();
            // wasi-sdk keeps per-target libraries in `lib/<triple>`; older
            // releases only had `wasm32-wasi`.
            for triple in [target.as_str(), "wasm32-wasip1", "wasm32-wasi"] {
                let dir = sysroot.join("lib").join(triple);
                if dir.join("libsetjmp.a").is_file() {
                    println!("cargo:rustc-link-search=native={}", dir.display());
                    break;
                }
            }
        }
        None => println!(
            "cargo:warning=minrx-sys: neither WASI_SDK_PATH nor WASI_SYSROOT is set; \
             install wasi-sdk (https://github.com/WebAssembly/wasi-sdk) and set \
             WASI_SDK_PATH so MinRX can be compiled and linked for WASI"
        ),
    }

    println!("cargo:rustc-link-lib=static=setjmp");
}
