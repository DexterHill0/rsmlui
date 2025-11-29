use std::path::PathBuf;

use cxx_build::CFG;

#[cfg(all(feature = "freetype", feature = "cosmic-text"))]
compile_error!("Features `freetype` and `cosmic-text` are mutually exclusive. Enable only one.");

const RSMLUI_SYS_LIB_NAME: &str = "rsmlui_dummy";
const RMLUI_OUTPUT_NAME: &str = "rmlui";

#[rustfmt::skip]
const DEFINTIONS: &[(&str, &str)] = &[
    #[cfg(feature = "freetype")]
    ("RMLUI_FONT_ENGINE", "freetype"),

    #[cfg(not(feature = "freetype"))]
    ("RMLUI_FONT_ENGINE", "none"),

    #[cfg(not(feature = "backend-win32-gl2"))]
    ("RMLUI_DISABLE_INCLUDE_WINDOWS", "ON"),

    ("BUILD_SHARED_LIBS", "OFF"),
    ("RMLUI_STATIC_LIB", "ON"),
    ("RMLUI_DEBUG", "ON"),
    ("RMLUI_NO_THIRDPARTY_CONTAINERS", "ON"),
    ("UNICODE", "ON"),
    ("_UNICODE", "ON"),
    ("WIN32_LEAN_AND_MEAN", "ON"),
];

fn build_rmlui_renderer(bridge: &mut cc::Build) {
    #[cfg(feature = "backend-win32-gl2")]
    bridge.file("RmlUi/Backends/RmlUi_Backend_Win32_GL2.cpp");

    #[cfg(feature = "renderer-gl2")]
    bridge.file("RmlUi/Backends/RmlUi_Renderer_GL2.cpp");

    #[cfg(feature = "platform-win32")]
    bridge.file("RmlUi/Backends/RmlUi_Platform_Win32.cpp");

    // required for vfuncs
    bridge.file("RmlUi/Source/Core/RenderInterface.cpp");

    // TODO: custom backend
    // #[cfg(not(feature = "backend_glfw_gl3"))]
}

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
        .clang_arg("-I./RmlUi/Backends")
        .allowlist_type("Rml::Input::KeyIdentifier")
        .allowlist_type("Rml::ClipMaskOperation")
        .allowlist_type("Rml::BlendMode")
        .allowlist_type("Rml::Vertex")
        .allowlist_var("")
        .allowlist_function("")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .translate_enum_integer_types(true)
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
    let mut bridge = cxx_build::bridges(&[
        "src/ffi/core.rs",
        "src/ffi/renderer.rs",
        "src/ffi/backend.rs",
        "src/ffi/context.rs",
        "src/ffi/element_document.rs",
    ]);
    let bridge = bridge
        // .file("src/cxx/Core.cpp")
        // .file("src/cxx/Backend.cpp")
        // .file("src/cxx/Renderer.cpp")
        // .file("src/cxx/Context.cpp")
        // .file("src/cxx/ElementDocument.cpp")
        .flag_if_supported("/std:c++20")
        .include(&cxx_include_dir)
        .include("./include")
        .include("./RmlUi/Include")
        .include("./RmlUi/Backends");

    for (key, value) in DEFINTIONS {
        bridge.define(key, *value);
    }

    build_rmlui_renderer(bridge);

    bridge.compile("rsmlui-cxx");

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
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");

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

    println!("cargo:rustc-link-lib=static={}", RSMLUI_SYS_LIB_NAME);
    println!("cargo:rustc-link-lib=static={}", RMLUI_OUTPUT_NAME);
    println!("cargo:rustc-link-lib=static={}_debugger", RMLUI_OUTPUT_NAME);

    #[cfg(feature = "freetype")]
    {
        println!("cargo:rustc-link-lib=static=bz2_static");
        println!("cargo:rustc-link-lib=static=brotlicommon");
        println!("cargo:rustc-link-lib=static=brotlienc");
        println!("cargo:rustc-link-lib=static=brotlidec");
        println!("cargo:rustc-link-lib=static=harfbuzz");
        println!("cargo:rustc-link-lib=static=zlibstatic");
        println!("cargo:rustc-link-lib=static=libpng16_static");
        println!("cargo:rustc-link-lib=static=freetype");
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=opengl32");
        println!("cargo:rustc-link-lib=gdi32");
    }
}
