use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataStore {
    pub keys: HashMap<String, String>,
}
