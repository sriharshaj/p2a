use std::{error, fs, io::Write, path};

#[derive(Default)]
pub enum EnumRepr {
    #[default]
    Int32,
    String,
}

#[derive(Default)]
pub struct Config {
    enum_repr: EnumRepr,
    proto_root: String,
    out_dir: Option<path::PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enum_repr(mut self, repr: EnumRepr) -> Self {
        self.enum_repr = repr;
        self
    }

    pub fn out_dir(mut self, path: &path::Path) -> Self {
        self.out_dir = Some(path.to_path_buf());
        self
    }

    pub fn with_protos_namespace(mut self, proto_root: String) -> Self {
        self.proto_root = proto_root;
        self
    }

    pub fn generate_builders(
        &self,
        source_files: &[&path::Path],
    ) -> Result<(), Box<dyn error::Error>> {
        for source_file in source_files {
            let proto_file_content = fs::read_to_string(source_file)?;
            let proto_code = syn::parse_file(&proto_file_content)?;

            let items =
                crate::builder::generate_arrow_builders(proto_code.items, &self.proto_root)?;
            let builders_code = syn::File {
                shebang: None,
                attrs: Vec::new(),
                items,
            };

            let source_stem = source_file
                .file_stem()
                .ok_or("Source file has no filename")?
                .to_str()
                .ok_or("Source filename is not valid UTF-8")?;

            let output_dir = source_file.parent().unwrap_or_else(|| path::Path::new("."));

            let output_filename = format!("{}_builder.rs", source_stem);
            let output_path = output_dir.join(output_filename);

            let mut builders_file = fs::File::create(&output_path)?;
            builders_file.write_all(&prettyplease::unparse(&builders_code).into_bytes())?;
        }

        Ok(())
    }
}
