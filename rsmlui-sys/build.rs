use std::path::PathBuf;

use cxx_build::CFG;

const RSMLUI_SYS_LIB_NAME: &str = "rsmlui_dummy";
const RMLUI_OUTPUT_NAME: &str = "rmlui";

const DEFINTIONS: &[(&str, &str)] = &[
    ("BUILD_SHARED_LIBS", "OFF"),
    ("RMLUI_STATIC_LIB", "ON"),
    ("RMLUI_FONT_ENGINE", "none"),
    ("RMLUI_DEBUG", "ON"),
    ("RMLUI_NO_THIRDPARTY_CONTAINERS", "ON"),
    ("RMLUI_DISABLE_INCLUDE_WINDOWS", "ON"),
    ("UNICODE", "ON"),
    ("_UNICODE", "ON"),
    ("WIN32_LEAN_AND_MEAN", "ON"),
];

fn main() {
    // we already sorta have a prefix because of the `include/rsmlui` folder structure, so no need for another
    CFG.include_prefix = "";

    let cxx_include_dir = format!("{}/cxxbridge/include", std::env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("include/rsmlui/Bindings.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}", cxx_include_dir))
        .clang_arg("-I./include")
        .clang_arg("-I./RmlUi/Include")
        .allowlist_type("")
        .allowlist_var("")
        .allowlist_function("")
        .clang_args(
            DEFINTIONS
                .iter()
                .map(|(key, value)| format!("-D{key}={}", *value)),
        )
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file("src/bindings.rs").unwrap();

    // cxx MUST come before cmake so the codegenned headers exist
    // when cmake does the build, as these headers are part of the `compile_commands.json`
    // used for intellisense
    let mut bridge = cxx_build::bridge("src/ffi/core.rs");

    for (key, value) in DEFINTIONS {
        bridge.define(key, *value);
    }

    bridge
        .file("src/cxx/Core.cpp")
        .flag_if_supported("/std:c++20")
        .include(&cxx_include_dir)
        .include("./include")
        .include("./RmlUi/Include")
        .compile("rsmlui-cxx");

    #[cfg(debug_assertions)]
    let cmake_build_type = "RelWithDebInfo";
    #[cfg(not(debug_assertions))]
    let cmake_build_type = "Release";

    let mut cmake_cfg = cmake::Config::new(".");

    cmake_cfg.build_target("all");
    cmake_cfg.generator("Ninja");

    cmake_cfg
        .define("RMLUI_OUTPUT_NAME", RMLUI_OUTPUT_NAME)
        .define("RSMLUI_SYS_LIB_NAME", RSMLUI_SYS_LIB_NAME)
        .define("RSMLUI_CXX_INCLUDE_DIR", cxx_include_dir)
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON");

    for (key, value) in DEFINTIONS {
        cmake_cfg.define(key, *value);
    }

    cmake_cfg.profile(cmake_build_type);

    let cmake_dst = cmake_cfg.build();

    let cmake_build_dir = cmake_dst.join("build");

    let copy_src = cmake_build_dir.join("compile_commands.json");
    let copy_dest =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("compile_commands.json");

    std::fs::copy(copy_src, copy_dest).expect("failed to copy compile commands");

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_build_dir.display()
    );

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=rsmlui_config.h.in");
    println!("cargo:rerun-if-changed=dummy.cpp");

    println!("cargo:rustc-link-lib=static={}", RSMLUI_SYS_LIB_NAME);
    println!("cargo:rustc-link-lib=static={}", RMLUI_OUTPUT_NAME);
    println!("cargo:rustc-link-lib=static={}_debugger", RMLUI_OUTPUT_NAME);
    println!("cargo:rustc-link-lib=user32");
}
