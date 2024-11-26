cmake_minimum_required(VERSION 3.30)
project(src_cpp)

set(CMAKE_CXX_STANDARD 20)

# Add the library
add_library(src_cpp STATIC library.cpp)

# Set output directories
set_target_properties(src_cpp PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/bin/src_cpp")
set_target_properties(src_cpp PROPERTIES INTERMEDIATE_DIRECTORY "${PROJECT_SOURCE_DIR}/bin/obj/src_cpp")
set_target_properties(src_cpp PROPERTIES OUTPUT_NAME "src_cpp-${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR}")


set_target_properties(src_cpp PROPERTIES PREFIX "lib" SUFFIX ".lib")

# Link against C++ standard library
target_link_libraries(src_cpp PRIVATE stdc++)