fn main() {
    println!("cargo:rustc-link-lib=dylib=clibrary_test");
    println!("cargo:rustc-link-search=native=./clibrary");
}