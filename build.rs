use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from("/opt/homebrew/Cellar/lame/3.100/lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=mp3lame");
}
