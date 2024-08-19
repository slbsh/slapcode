fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/stack.cpp")
        .flag("-std=c++23")
        .flag("-O3")
        .compile("stack");
}
