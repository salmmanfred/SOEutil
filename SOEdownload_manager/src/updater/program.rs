use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Program {
    Game,
    Launcher,
    Downloader,
}
