use cmake;
use std::collections::HashMap;
use std::path::Path;

pub fn main() {
    let mut settings = config::Config::default();
    let osd_root = std::env::var("OPENSUBDIV_ROOT")
        .expect("OPENSUBDIV_ROOT env var must be set to root of OpenSubdiv installation");

    let inc_osd = Path::new(&osd_root).join("include");
    let lib_osd = Path::new(&osd_root).join("lib");

    let dst_capi = cmake::Config::new("osd-capi")
        .define("INC_OSD", &inc_osd)
        .always_configure(false)
        .build();

    println!("cargo:rustc-link-search=native={}", dst_capi.display());
    println!("cargo:rustc-link-lib=static=osd-capi");

    println!("cargo:rustc-link-search=native={}", lib_osd.display());
    println!("cargo:rustc-link-lib=dylib=osdCPU");
    println!("cargo:rustc-link-lib=dylib=osdGPU");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

}
