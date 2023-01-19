use std::fs;
use std::path::{Path, PathBuf};
use std::env::consts::OS;

use super::github::release::Releases;
use log::{error, info};

pub async fn download_latest_release(mut releases: Releases) -> Result<PathBuf, ()> {
    releases.retain(|release| !release.is_prerelease());

    if releases.is_empty() {
        error!("There is no stable release in the specified repository");
        return Err(());
    }

    for release in releases {
        for asset in release.get_assets() {
            if asset.get_name().contains(OS) {
                let result = reqwest::get(asset.get_download_url()).await;

                if let Ok(response) = result {
                    // TODO: better error handling 
                    let content = response.bytes().await.unwrap();

                    let updater_cache = Path::new(".cache/updater");
                    let file_path = updater_cache.join(asset.get_name());

                    let _ = fs::create_dir_all(updater_cache);
                    let _ = fs::write(&file_path, content);

                    return Ok(file_path);
                } else {
                    error!(
                        "Unable to retreive {} from {}",
                        asset.get_name(),
                        asset.get_download_url()
                    );
                    info!("This program will continue it's execution");
                    return Err(());
                }
            }
        }
    }
    Err(())
}
