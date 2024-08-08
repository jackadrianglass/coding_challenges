fn main() {
    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .file("cxx/calculator.cc")
        .std("c++20")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cxx/calculator.cc");
    println!("cargo:rerun-if-changed=cxx/calculator.h");
}
