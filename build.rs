use std::process::Command;


fn main() {
    println!("cargo:rustc-link-search=native=./clibrary");

    // println!("cargo:rustc-link-lib=dylib=rust.cpython-39-darwin");
    // Use python to acquire the path to the relevant library
    let output = Command::new("python")
        .arg("-c")
        .arg("import os;import service_module;print(os.path.dirname(service_module.__file__))")
        .output()
        .expect("failed to execute python command");

    println!("cargo:rustc-link-search=native={}", String::from_utf8(output.stdout).unwrap());
}