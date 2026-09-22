use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    for name in ["equipment", "monsters"] {
        let source = format!("src/databases/{name}.json");
        println!("cargo::rerun-if-changed={source}");

        let json: serde_json::Value = serde_json::from_slice(&fs::read(&source)?)?;
        let minified = serde_json::to_vec(&json)?;
        fs::write(out_dir.join(format!("{name}.json")), minified)?;
    }

    Ok(())
}
