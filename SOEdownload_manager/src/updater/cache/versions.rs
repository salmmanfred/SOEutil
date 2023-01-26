use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::updater::program::Program;

pub type Versions = Vec<Version>;

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub program:Program,
    pub version:String,
    pub file_hashes: HashMap<String, String>,
    pub prerelease:bool,
}
