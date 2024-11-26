use cmake::Config;
use std::env;

fn main() {
    // Use CMake to compile the C++ library
    let dst = Config::new("../src-cpp")
        .no_build_target(true)
        .configure_arg("/NODEFAULTLIB:library")
        .build();

    // The path to the actual directory containing the compiled library
    let out_path = if cfg!(debug_assertions) {
        dst.join("build/Debug")
    } else {
        dst.join("build/Release")
    };

    println!("Path to the compiled library: {}", out_path.display());

    // Output directory containing the compiled library
    println!("cargo:rustc-link-search=native={}", out_path.display());

    // Link the static library with the exact name
    println!("cargo:rustc-link-lib=static=libsrc_cpp-Windows-AMD64");

    // Link the C++ standard library
    let target = env::var("TARGET").unwrap();
    if target.contains("msvc") {
        // Windows Microsoft Visual C/C++ Compiler (MSVC)
        println!("cargo:rustc-link-lib=dylib=msvcrt");
    } else if target.contains("gnu") {
        // Unix/Linux GNU C/C++ Compiler (GCC)
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("apple") {
        // Darwin/MacOS C/C++ Compiler
        // This uses XCode's CLANG Compiler
        println!("cargo:rustc-link-lib=dylib=c++");
    }
}
