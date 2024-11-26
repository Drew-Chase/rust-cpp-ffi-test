mod cpplink;

fn main() {
    hello();
}

fn hello() {
    unsafe {
        cpplink::hello();
    }
}
