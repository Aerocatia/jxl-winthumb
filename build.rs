extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        winresource::WindowsResource::new().compile().unwrap();
        if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc" {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-cdylib-link-arg=/DEF:{manifest_dir}\\jxl_winthumb.def");
            println!("cargo:rerun-if-changed=jxl_winthumb.def");
        }
    }
}
