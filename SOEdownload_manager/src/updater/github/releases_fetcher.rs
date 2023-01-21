use super::release::Releases;
use log::info;
use reqwest::header::USER_AGENT;
use reqwest::Client;

pub async fn fetch_releases(url: &str) -> Result<Releases, Box<dyn std::error::Error>> {
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36";

    info!("Downloading latest release from {url}");
    info!("User agent set to {user_agent}");

    let client = Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;
    
    let releases = response.json::<Releases>().await?;
    
    Ok(releases)
}
