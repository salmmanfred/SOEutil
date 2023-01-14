use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LauncherSettings {
    previous_mods: Vec<String>,
}

impl LauncherSettings {
    pub fn new() -> LauncherSettings {
        LauncherSettings { previous_mods: Vec::new() }
    }

    pub fn from_mod_list(mods: &Vec<String>) -> LauncherSettings {
        LauncherSettings {
            previous_mods: mods.clone(),
        }
    }
}