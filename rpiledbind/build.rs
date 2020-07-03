use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/lib", manifest_dir);
    println!("cargo:rustc-link-lib=static=rgbmatrix");
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
