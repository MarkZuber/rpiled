fn main() {
    println!("cargo:rustc-link-search=./lib");
    println!("rustc-link-lib:librgbmatrix.a");
}
