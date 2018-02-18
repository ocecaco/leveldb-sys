fn main() {
    println!("cargo:rustc-link-lib=static=leveldb");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-search=native=/home/daniel/mcpetools/leveldb-mcpe/out-static");
}
