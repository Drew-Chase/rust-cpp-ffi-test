cmake_minimum_required(VERSION 3.30)
project(src_cpp)
set(CMAKE_CXX_STANDARD 20)
add_library(src_cpp STATIC library.cpp)

set_target_properties(src_cpp PROPERTIES RUNTIME_OUTPUT_DIRECTORY "${PROJECT_SOURCE_DIR}/bin/src_cpp")
set_target_properties(src_cpp PROPERTIES INTERMEDIATE_DIRECTORY "${PROJECT_SOURCE_DIR}/bin/obj/src_cpp")
set_target_properties(src_cpp PROPERTIES DEBUG_POSTFIX "-debug")
set_target_properties(src_cpp PROPERTIES OUTPUT_NAME "src_cpp-${CMAKE_SYSTEM_NAME}-${CMAKE_SYSTEM_PROCESSOR}")
set(CMAKE_CACHEFILE_DIR ${PROJECT_SOURCE_DIR}/bin/obj/cmake/src_cpp)

target_compile_options(src_cpp PRIVATE -Oz)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -static-libgcc -static-libstdc++ -static -lpthread")