// generated through https://app.quicktype.io/?l=rs

use serde::{Deserialize, Serialize};

pub type Releases = Vec<Release>;

#[derive(Serialize, Deserialize)]
pub struct Release {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: i64,
    author: Author,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    assets: Vec<Asset>,
    tarball_url: String,
    zipball_url: String,
    body: String,
}

impl Release {
    pub fn is_prerelease(&self) -> bool {
        self.prerelease
    }
    pub fn get_assets(&self) -> &Vec<Asset> {
        &self.assets
    }
}

#[derive(Serialize, Deserialize)]
pub struct Asset {
    url: String,
    id: i64,
    node_id: String,
    name: String,
    label: Option<serde_json::Value>,
    uploader: Author,
    content_type: String,
    state: String,
    size: i64,
    download_count: i64,
    created_at: String,
    updated_at: String,
    browser_download_url: String,
}

impl Asset {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_download_url(&self) -> &String {
        &self.browser_download_url
    }
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    author_type: String,
    site_admin: bool,
}
