use crate::base::*;
use crate::error::*;
use crate::addons_manager::GameVersion;
use futures::future::join_all;
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::fs::create_dir_all;
use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
struct Asset {
    browser_download_url: String,
}

#[derive(Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>,
}

impl From<reqwest::Error> for MetalError {
    fn from(_error: reqwest::Error) -> Self {
        MetalError::new("Request failed")
    }
}

impl From<serde_json::Error> for MetalError {
    fn from(error: serde_json::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

impl From<regex::Error> for MetalError {
    fn from(error: regex::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

impl From<io::Error> for MetalError {
    fn from(error: io::Error) -> Self {
        MetalError::new(&error.to_string())
    }
}

#[derive(Debug)]
pub struct AssetInfo {
    url: String,
    repo: String,
    version: String,
    file_name: String,
    game_version: GameVersion,
}

fn get_asset_info(repo: &str, url: &str) -> MetalResult<AssetInfo> {
    let caps = Regex::new("download/([^/]+)/(.+)")?
        .captures(url)
        .ok_or(MetalError::new("Version and filename not found"))?;

    let version = caps
        .get(1)
        .ok_or(MetalError::new("Version not found"))?
        .as_str();

    let file_name = caps
        .get(2)
        .ok_or(MetalError::new("Filename not found"))?
        .as_str();

    let game_version = if file_name.contains("classic") {
        GameVersion::Classic
    } else if file_name.contains("bcc") {
        GameVersion::Bcc
    } else {
        GameVersion::Retail
    };

    Ok(AssetInfo {
        url: url.to_owned(),
        repo: repo.to_owned(),
        version: version.to_owned(),
        file_name: file_name.to_owned(),
        game_version,
    })
}

pub async fn get_assets_urls(
    http_client: &Client,
    username: &str,
    repo: &str,
) -> MetalResult<Vec<AssetInfo>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        username, repo
    );
    let body = http_client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Metal")
        .send()
        .await?
        .text()
        .await?;

    let release: Release = serde_json::from_str(&body)?;
    release
        .assets
        .into_iter()
        .map(|a| a.browser_download_url)
        .filter(|url| url.ends_with(".zip"))
        .map(|url| get_asset_info(repo, &url))
        .collect::<Result<Vec<AssetInfo>, MetalError>>()
}

fn game_version_to_folder_name(game_version: &GameVersion) -> String {
    match game_version {
        GameVersion::Bcc => "bcc",
        GameVersion::Classic => "classic",
        GameVersion::Retail => "retail",
    }
    .to_owned()
}

async fn download_file(
    http_client: &Client,
    asset_info: &AssetInfo,
    root_folder: &PathBuf,
) -> MetalResult<()> {
    let file_path = Path::new(root_folder)
        .join(&game_version_to_folder_name(&asset_info.game_version))
        .join(&asset_info.repo)
        .join(&asset_info.version)
        .join(&asset_info.file_name);

    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = File::create(file_path)?;
    let mut response = http_client.get(&asset_info.url).send().await?;
    while let Some(mut chunk) = response.chunk().await? {
        file.write_all(&mut chunk)?;
    }
    Ok(())
}

pub async fn download_assets(
    http_client: &Client,
    username: &str,
    repo: &str,
    root_folder: &PathBuf,
) -> MetalResult<()> {
    let asset_urls = get_assets_urls(http_client, username, repo).await?;

    let futures = asset_urls
        .iter()
        .map(|asset_info| download_file(http_client, &asset_info, root_folder))
        .collect::<Vec<_>>();

    combine_errors(&join_all(futures).await)
        .map(|_| ())
        .map_err(|errors| {
            MetalError::new(
                &errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
        })?;

    Ok(())
}
