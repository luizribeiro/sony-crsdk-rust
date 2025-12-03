use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap();

    let sdk_path = workspace_root.join("../app/CRSDK");
    let libs_path = workspace_root.join("libs");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=src/callback_shim.cpp");
    println!("cargo:rerun-if-changed={}", sdk_path.display());

    // Compile the C++ callback shim
    cc::Build::new()
        .cpp(true)
        .file("src/callback_shim.cpp")
        .include(&sdk_path)
        .flag("-std=c++17")
        .compile("callback_shim");

    // Link to Sony SDK libraries (copied to libs/ directory)
    let crsdk_lib = libs_path.join("crsdk");
    let adapters_lib = libs_path.join("adapters");
    let opencv_lib = libs_path.join("opencv");

    println!("cargo:rustc-link-search=native={}", crsdk_lib.display());
    println!("cargo:rustc-link-search=native={}", adapters_lib.display());
    println!("cargo:rustc-link-search=native={}", opencv_lib.display());

    // Link libraries
    println!("cargo:rustc-link-lib=dylib=Cr_Core");
    println!("cargo:rustc-link-lib=dylib=monitor_protocol");
    println!("cargo:rustc-link-lib=dylib=monitor_protocol_pf");
    println!("cargo:rustc-link-lib=dylib=Cr_PTP_IP");
    println!("cargo:rustc-link-lib=dylib=Cr_PTP_USB");
    println!("cargo:rustc-link-lib=dylib=ssh2");
    println!("cargo:rustc-link-lib=dylib=usb-1.0.0");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", sdk_path.display()))
        // Allow C++ features
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        // Allowlist SDK types and functions
        .allowlist_function("SCRSDK::.*")
        .allowlist_type("SCRSDK::.*")
        .allowlist_var("SCRSDK::.*")
        // Enable C++ namespace handling
        .enable_cxx_namespaces()
        // Generate comments from headers
        .generate_comments(true)
        // Use core instead of std
        .use_core()
        // Derive traits
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        // Handle opaque types
        .opaque_type("std::.*")
        .opaque_type(".*::basic_string.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
