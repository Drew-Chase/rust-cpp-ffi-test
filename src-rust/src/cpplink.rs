//! Library File relative path:
//! ../src-cpp/bin/obj/release/libsrc_cpp-Windows-AMD64.lib
//! The `link` macro will automatically add the .lib to the end of the name


#[link(name = "libsrc_cpp-Windows-AMD64", kind = "static")]
extern "C" {
    pub fn hello();
}
