fn main() {
    #[cfg(debug_assertions)]
    let cmake_build_type = "RelWithDebInfo";
    #[cfg(not(debug_assertions))]
    let cmake_build_type = "Release";

    let mut cmake_cfg = cmake::Config::new(".");
    cmake_cfg.define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON");

    cmake_cfg.profile(cmake_build_type);

    let cmake_dst = cmake_cfg.build();

    let cmake_lib_dir = cmake_dst.join("lib");
    let cmake_build_dir = cmake_dst.join("build");
    let cmake_include_dir = cmake_dst.join("include");

    println!("cargo:rustc-link-search=native={}", cmake_lib_dir.display());
    println!(
        "cargo:rustc-link-search=native={}",
        cmake_build_dir.join(cmake_build_type).display()
    );
    println!("cargo:rustc-link-lib=static=rsmlui_sys_lib");
    println!("cargo:rustc-link-lib=static=rmlui");
    println!("cargo:rustc-link-lib=static=rmlui_debugger");
    println!("cargo:rustc-link-lib=user32");

    let bindings = bindgen::Builder::default()
        .header("include/Bindings.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .allowlist_type("")
        .allowlist_var("")
        .allowlist_function("")
        .clang_arg(format!("-I{}", cmake_include_dir.display()))
        .clang_arg("-I./include")
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/bindings.rs").unwrap();

    cxx_build::bridge("src/ffi/core.rs")
        .file("src/cxx/Core.cpp")
        .flag_if_supported("/std:c++20")
        .include("rsmlui-sys/include")
        .include(cmake_include_dir)
        .include(cmake_build_dir)
        .compile("rsmlui-cxx");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=rsmlui_config.h.in");
}
