mod updater;
use clap::Parser;
use log::{error, info, warn};
use simplelog::*;
use std::env::consts::OS;
use std::fs::{self, File};
use updater::downloader::download_latest_release;
use updater::github::releases_fetcher::fetch_releases;
use updater::installer::install_archive;

#[derive(Parser)]
struct Args {
    #[arg(long, action)]
    update_game: bool,
    #[arg(long, action)]
    update_launcher: bool,

    #[arg(long, action)]
    allow_prereleases: bool,

    //"https://api.github.com/repos/yrenum/symphony-of-empires/releases"
    #[arg(default_value_t = String::from("https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases"))]
    game_releases_url: String,

    #[arg(default_value_t = String::from("https://api.github.com/repos/symphony-of-empires/SOEutil/releases"))]
    launcher_releases_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("download_manager.log").unwrap(),
    );

    let args = Args::parse();

    if args.update_game {
        info!("Starting game update...");

        let releases = fetch_releases(&args.game_releases_url).await?;
        let result = download_latest_release(releases, OS, args.allow_prereleases).await;

        if let Ok(path) = result {
            install_archive(path);
        } else {
            warn!("Something went wrong during the game update");
        }
    }
    if args.update_launcher {
        info!("Starting launcher update...");

        let releases = fetch_releases(&args.launcher_releases_url).await?;
        let result = download_latest_release(releases, OS, args.allow_prereleases).await;

        if let Ok(path) = result {
            install_archive(path);
        } else {
            warn!("Something went wrong during the launcher update");
        }
    }


    if let Err(_) = fs::remove_dir_all(".cache") {
        error!("Couldn't delete the cache directory");
    }


    Ok(())
}
