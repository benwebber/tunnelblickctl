use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/tunnelblick.ts");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tunnelblick.js");
    let status = Command::new("tsc")
        .args([
            "--lib",
            "es2015",
            "--strict",
            "--target",
            "es5",
            "--outFile",
        ])
        .arg(&out)
        .arg("src/tunnelblick.ts")
        .status()
        .expect("tsc not found");
    assert!(status.success());
}
