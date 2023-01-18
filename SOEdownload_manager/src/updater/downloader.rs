use super::github::release::{Releases};

pub async fn download_latest_release(releases:&Releases, file_name_ending:&String) {
    for release in releases {
        if release.is_prerelease() {
            continue;
        }
        for artifact in release.get_assets(){
            
        }
        
    } 
}