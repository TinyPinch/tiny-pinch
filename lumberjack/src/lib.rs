pub mod offsets;
pub mod types;

use std::{fs::File, io::BufReader, path::Path};

use types::Types;
use offsets::Offsets;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dump {
    offsets: Offsets,
    types: Types,
}

impl Dump {
    pub fn new(offsets: Offsets, types: Types) -> Self {
        Self { offsets, types }
    }
    
    pub fn offsets(&self) -> &Offsets {
        &self.offsets
    }

    pub fn types(&self) -> &Types {
        &self.types
    }

    pub fn load_from(path: impl AsRef<Path>) -> Option<Self> {
        let reader = BufReader::new(File::open(path).ok()?);

        bincode::deserialize_from(reader).ok()
    }
}
