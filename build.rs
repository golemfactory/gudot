use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn main() {
    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set"));

    let mut cmd = Command::new("cargo");
    cmd.args(&[
        "build",
        "--release",
        "--target-dir",
        &out_dir.to_str().unwrap(),
    ])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .current_dir("gudot-gwasm");

    let output = cmd.output().expect("Building gudot-gwasm failed");
    let status = output.status;
    if !status.success() {
        panic!(
            "Building gudot-gwasm failed: exit code: {}",
            status.code().unwrap()
        );
    }

    // NB this will *not* be necessary when --out-dir option of cargo build
    // makes stable
    let source_dir = &out_dir.join("wasm32-unknown-emscripten").join("release");
    let profile =
        PathBuf::from(env::var("PROFILE").expect("The PROFILE environment variable must be set"));
    let target_dir = Path::new("target").join(profile);
    const JS: &str = "gudot.js";
    const WASM: &str = "gudot.wasm";
    fs::copy(source_dir.join(JS), target_dir.join(JS)).unwrap_or_else(|_| {
        panic!(
            "Failed copying {} from {} to {}",
            JS,
            source_dir.display(),
            target_dir.display()
        )
    });
    fs::copy(source_dir.join(WASM), target_dir.join(WASM)).unwrap_or_else(|_| {
        panic!(
            "Failed copying {} from {} to {}",
            WASM,
            source_dir.display(),
            target_dir.display()
        )
    });
}
