use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LauncherSettings {
    active_mods: Vec<String>,
}

impl LauncherSettings {
    pub fn new() -> LauncherSettings {
        LauncherSettings { active_mods: Vec::new() }
    }

    pub fn from_mod_list(mods: &Vec<String>) -> LauncherSettings {
        LauncherSettings {
            active_mods: mods.clone(),
        }
    }
}