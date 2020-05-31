fn main() {
    println!("cargo:rustc-link-search=native=/home/pi/repos/rpiled/rpiledbind/lib");
    println!("cargo:rustc-link-lib=static=rgbmatrix");
    println!("cargo:rustc-flags=-l dylib=stdc++");
}
