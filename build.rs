#[cfg(feature = "docs-rs")]
fn main() {}

#[cfg(not(feature = "docs-rs"))]
fn main() {
    use std::env;

    let mut cxx = cxx_build::bridge("src/lib.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "linux" => {
            let lib = pkg_config::probe_library("libsecret-1").unwrap();
            for dir in lib.include_paths.into_iter() {
                cxx.include(dir);
            }

            cxx.file("node-keytar/src/keytar_posix.cc")
                .flag("-fexceptions")
                .flag("-Wno-missing-field-initializers")
                .flag("-Wno-deprecated-declarations");
        }
        "macos" => {
            println!("cargo:rustc-link-lib=framework=AppKit");
            println!("cargo:rustc-link-lib=framework=Security");

            cxx.file("node-keytar/src/keytar_mac.cc");
        }
        "windows" => {
            cxx.file("node-keytar/src/keytar_win.cc");
        }
        _ => panic!("unsupported TARGET_OS: {}", target_os),
    }

    cxx.file("src/lib.cc")
        .flag_if_supported("-std=c++14")
        .compile("keytar-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.cc");
}
