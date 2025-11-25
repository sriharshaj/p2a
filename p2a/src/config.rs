use std::{error, fs, io::Write, path};

use crate::builder::{generate_arrow_builders, parse_oneof};

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
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enum_repr(mut self, repr: EnumRepr) -> Self {
        self.enum_repr = repr;
        self
    }

    pub fn with_protos_namespace(mut self, proto_root: String) -> Self {
        self.proto_root = proto_root;
        self
    }

    pub fn generate_builders(
        &self,
        source: &path::Path,
        dest: &path::Path,
    ) -> Result<(), Box<dyn error::Error>> {
        let proto_file_content = fs::read_to_string(source)?;
        let proto_code = syn::parse_file(&proto_file_content)?;

        let oneof_enums = parse_oneof(&proto_code.items, &self.proto_root, "self")?;
        let items =
            generate_arrow_builders(proto_code.items, &self.proto_root, "self", &oneof_enums)?;
        let builders_code = syn::File {
            shebang: None,
            attrs: Vec::new(),
            items,
        };

        let mut builders_file = fs::File::create(dest)?;
        builders_file.write_all(&prettyplease::unparse(&builders_code).into_bytes())?;

        Ok(())
    }
}
