use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::cli::AddArgs;

#[derive(Serialize, Deserialize)]
pub struct DataStore {
    pub keys: HashMap<String, String>,
}

impl DataStore {
    pub fn add(&mut self, args: &AddArgs) -> Result<()> {
        if self.keys.contains_key(args.key.as_str()) {
            bail!("key already exists");
        }

        self.keys.insert(args.key.clone(), args.value.clone());

        return Ok(());
    }
}
