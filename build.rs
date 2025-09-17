fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++23")
        .opt_level(3)
        .flag("-march=x86-64-v2")
        .file("src/max.cxx")
        .compile("cppmax");
}
