use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let mut target_dir = PathBuf::from(&out_dir);

    for _ in 0..3 {
        target_dir.pop();
    }

    let source_path = Path::new("src/server.json");
    let target_path = target_dir.join("server.json");

    fs::copy(source_path, &target_path).unwrap_or_else(|_| panic!("Failed to copy {:?} to {:?}",
        source_path, target_path));
}
