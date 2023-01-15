use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LauncherSettings {
    active_mods: Vec<String>,
}

impl LauncherSettings {
    pub fn new() -> LauncherSettings {
        LauncherSettings { active_mods: Vec::new() }
    }
}