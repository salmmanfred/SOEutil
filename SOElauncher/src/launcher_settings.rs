use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct LauncherSettings {
    previous_mods: Vec<String>,
}
impl LauncherSettings {
    pub fn from_mod_list(mods: &Vec<String>) -> LauncherSettings {
        LauncherSettings {
            previous_mods: mods.clone(),
        }
    }
}

impl Serialize for LauncherSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("LauncherSettings", 1)?;
        state.serialize_field("previous_mods", &self.previous_mods)?;
        state.end()
    }
}