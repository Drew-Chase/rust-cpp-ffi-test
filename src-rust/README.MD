# Rust FFI Example with C++

## Overview

This project showcases how to use Rust's Foreign Function Interface (FFI) to call functions from a C++ library. The Rust code seamlessly interacts with a simple C++ function using FFI.

## Project Structure

The project has the following structure:

```
rust-ffi-test/
├── src/
│   ├── main.rs
│   ├── cpplink.rs
├── build.rs
├── Cargo.toml
src_cpp/
    ├── CMakeLists.txt
    ├── library.h
    ├── library.cpp
```

### Rust Files

1. **src/main.rs**

    ```rust
    mod cpplink;

    fn main() {
        hello();
    }

    fn hello() {
        unsafe {
            cpplink::hello();
        }
    }
    ```

   This is the entry point of the Rust application. It declares and calls a Rust function `hello` which, through Rust's FFI, calls the C++ function `hello` defined in the linked C++ library.

2. **src/cpplink.rs**

    ```rust
    #[link(name = "libsrc_cpp-Windows-AMD64", kind = "static")]
    extern "C" {
        pub fn hello();
    }
    ```

   This file contains the FFI declaration linking the `hello` function from the C++ library.

3. **build.rs**

    ```rust
    use cmake::Config;
    use std::env;

    fn main() {
        let dst = Config::new("../src-cpp")
            .no_build_target(true)
            .configure_arg("/NODEFAULTLIB:library")
            .build();

        let out_path = if cfg!(debug_assertions) {
            dst.join("build/Debug")
        } else {
            dst.join("build/Release")
        };

        println!("Path to the compiled library: {}", out_path.display());
        println!("cargo:rustc-link-search=native={}", out_path.display());
        println!("cargo:rustc-link-lib=static=libsrc_cpp-Windows-AMD64");

        let target = env::var("TARGET").unwrap();
        if target.contains("msvc") {
            println!("cargo:rustc-link-lib=dylib=msvcrt");
        } else if target.contains("gnu") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        } else if target.contains("apple") {
            println!("cargo:rustc-link-lib=dylib=c++");
        }
    }
    ```

   This build script uses the `cmake` crate to compile the C++ library and link it with the Rust code.

4. **Cargo.toml**

    ```toml
    [package]
    name = "rust-ffi-test"
    version = "0.1.0"
    edition = "2021"

    [dependencies]

    [build-dependencies]
    cmake = { version = "0.1.52" }

    [build]
    build = "build.rs"
    ```

   The Cargo configuration file for the project.

## Building the Project

To build this project, follow these steps:

1. **Build the C++ Library**

   Navigate to the `src_cpp` directory and use CMake to build the C++ library:

    ```sh
    cmake .
    cmake --build .
    ```

2. **Build the Rust Project**

   From the root directory of the Rust project, run:

    ```sh
    cargo build
    ```

   This will trigger the `build.rs` script to compile and link the C++ library with the Rust code.

## Running the Project

After successfully building the project, you can run the Rust executable:

```sh
cargo run
```

This will call the `hello` function from the C++ library, and you should see the output `Hello, World!` printed to the console.

## C++ Library

For detailed information about the C++ library, refer to the documentation in the `src_cpp` directory. Here is a quick summary:

The C++ library consists of a header file `library.h` and an implementation file `library.cpp`:

**library.h**
```cpp
extern "C" {
    void hello();
}
```

**library.cpp**
```cpp
#include "library.h"
#include <iostream>

void hello() {
    std::cout << "Hello, World!" << std::endl;
}
```

Using this setup, the C++ library can be called from the Rust code, demonstrating cross-language interoperability using FFI.
