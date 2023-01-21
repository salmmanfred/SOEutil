use std::fs;
use std::path::{Path, PathBuf};

use super::github::release::Releases;
use log::{error, info};

pub async fn download_latest_release(mut releases: Releases, file_name_contains:&str, allow_prereleases:bool) -> Result<PathBuf, ()> {
    
    if !allow_prereleases {
        releases.retain(|release| !release.is_prerelease());
    }
    

    if releases.is_empty() {
        error!("There is no stable release in the specified repository");
        return Err(());
    }

    for release in releases {
        for asset in release.get_assets() {
            if asset.get_name().contains(file_name_contains) {
                let result = reqwest::get(asset.get_download_url()).await;

                if let Ok(response) = result {
                    // TODO: better error handling 
                    let content = response.bytes().await.unwrap();

                    let updater_cache = Path::new(".cache/updater");
                    let file_path = updater_cache.join(asset.get_name());
                    
                    if let Err(_) = fs::create_dir_all(updater_cache){
                        error!("Error creating the file directroy {:?}", updater_cache);
                    }
                    
                    if let Err(_) = fs::write(&file_path, content){
                        error!("Error writing to a file {:?}",file_path);
                    }

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
