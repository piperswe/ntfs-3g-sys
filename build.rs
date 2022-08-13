use std::{env, fs, path::PathBuf};

extern crate autotools;
extern crate bindgen;
extern crate rm_rf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if out_path.exists() {
        let _ = rm_rf::remove(&out_path);
        fs::create_dir(&out_path).expect("Failed to create out path");
    }

    let dst = autotools::Config::new("vendor/ntfs-3g")
        .without("fuse", None)
        .disable("ntfsprogs", None)
        .disable("ntfs-3g", None)
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ntfs-3g");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}/include/ntfs-3g", dst.display()))
        .whitelist_function("ntfs_.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
