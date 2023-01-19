mod updater;
use updater::downloader::download_latest_release;
use updater::github::releases_fetcher::fetch_releases;
use clap::Parser;
use log::warn;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value_t = false)]
    update_game:bool,

    #[arg(default_value_t = false)]
    update_launcher:bool,

    #[arg(default_value_t = String::from("https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases"))]
    game_releases_url:String,

    #[arg(default_value_t = String::from("https://api.github.com/repos/symphony-of-empires/SOEutil/releases"))]
    launcher_releases_url:String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let releases = fetch_releases("https://api.github.com/repos/yrenum/symphony-of-empires/releases").await?;

    let result = download_latest_release(releases).await;

    if let Err(_) = result {
        warn!("Something went wrong during files download");
    }

    Ok(())
}
