# C++ Library

## Overview

This library provides a simple C++ function that can be called from other programming languages, such as Rust, using FFI (Foreign Function Interface).

## Directory Structure

The library has the following structure:

```
src_cpp/
    ├── CMakeLists.txt
    ├── library.h
    ├── library.cpp
```

## Building with CMake

The `CMakeLists.txt` file contains the build instructions for the C++ project. It specifies how the compiler should build the project and generate the necessary binaries.

## Header File (`library.h`)

The header file declares the functions provided by the library. It is not strictly necessary but helps in separating function declarations from their implementations.

This library includes a single function declaration: `void hello();`.

The `extern` keyword is used to ensure the function can be accessed from Rust. The `"C"` specifier indicates to the Rust compiler that the function uses C linkage.

### Declaration Example:
```cpp
extern "C"
{
    void hello();
}
```

## Implementation File (`library.cpp`)

The implementation file defines the logic for the functions declared in the header file.

### Implementation Example:
```cpp
#include "library.h"
#include <iostream>

void hello()
{
    std::cout << "Hello, World!" << std::endl;
}
```

With this setup, you can build the C++ library and use it in a Rust project, enabling cross-language functionality through FFI.
