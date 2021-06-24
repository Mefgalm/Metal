mod base;
mod error;
mod downloader;
mod addons_manager;
mod file_system_addons_loader;

use crate::error::*;
use crate::downloader::*;
use std::env::current_dir;
use std::path::Path;

#[tokio::main]
async fn main() -> MetalResult<()> {
    let http_client = reqwest::Client::new();
    let home_dir = Path::new(&dirs::home_dir().unwrap_or(current_dir()?)).join(".metal");
    download_assets(&http_client, "WeakAuras", "WeakAuras2", &home_dir).await?;
    Ok(())
}
