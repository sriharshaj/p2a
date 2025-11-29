use std::{fs, io::Result, path::PathBuf};
fn main() -> Result<()> {
    let generated_dir = PathBuf::from("src");
    fs::create_dir_all(&generated_dir)?;

    prost_build::Config::new()
        .out_dir(&generated_dir)
        .compile_protos(&["protos/example.proto"], &["protos/"])?;

    let example = generated_dir.join("example.rs");
    let example_builders = generated_dir.join("example_builders.rs");
    let protos_namespace = String::from("crate::example");
    let arrow_namespace = String::from("crate::example_builders");

    if let Err(e) = p2a::Config::new()
        .with_protos_namespace(protos_namespace)
        .with_arrow_namespace(arrow_namespace)
        .generate_builders(&example, &example_builders)
    {
        eprintln!("Error converting to arrow builders: {}", e);
        return Err(std::io::Error::other(e.to_string()));
    }
    Ok(())
}
