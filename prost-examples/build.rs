use std::{fs, io::Result, path::PathBuf};
fn main() -> Result<()> {
    let generated_dir = PathBuf::from("src/generated");
    fs::create_dir_all(&generated_dir)?;

    prost_build::Config::new()
        .out_dir(&generated_dir)
        .type_attribute(".", "#[proto2arrow::derive_builder]")
        .compile_protos(&["protos/example.proto"], &["protos/"])?;
    Ok(())
}
