use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let target_dir = match env::var("CARGO_TARGET_DIR") {
        Ok(val) => PathBuf::from(val),
        Err(_) => PathBuf::from("target"),
    };

    let source_path = Path::new("src\\server.json");

    let target_path = target_dir.join("debug").join("server.json");

    fs::copy(source_path, target_path).expect("Failed to copy debug server.json");
}
