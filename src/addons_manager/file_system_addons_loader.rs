use std::path::PathBuf;
use std::{fs::File, path::Path};

use crate::base::*;
use crate::{addons_manager::*, error::MetalResult, MetalError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

const ADDONS_JSON: &str = "addons.json";
const ADDONS_LOCK: &str = "addons.lock";
const DEFAULT_JSON: &str = "{
    \"wow_root_path\": null,
    \"addons\": [

    ]
}";

pub struct FileSystemAddonsLoader {
    root_folder: PathBuf,
    json_path: PathBuf,
}

impl FileSystemAddonsLoader {
    pub fn new(root_folder: &PathBuf) -> Self {
        Self {
            root_folder: root_folder.clone(),
            json_path: Path::new(root_folder).join(ADDONS_JSON),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AddonJson {
    url: String,
    game_versions: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Configs {
    wow_root_path: Option<String>,
    addons: Vec<AddonJson>,
}

fn parse_json_github_url(
    game_versions: &Vec<GameVersion>,
    url: &str,
) -> MetalResult<(String, AddonInfo)> {
    let caps = Regex::new("github.com/([\\d\\w]+)/([\\d\\w]+)")?
        .captures(url)
        .ok_or(MetalError::new(
            "Url should be in format 'github.com/{username}/{repo}'",
        ))?;

    let username = caps.get(1).unwrap().as_str();
    let repo = caps.get(2).unwrap().as_str();

    Ok((
        repo.to_owned(),
        AddonInfo::new(url, username, game_versions),
    ))
}

fn parse_configs(configs: &Configs) -> MetalResult<AddonsManagerState> {
    let mut addons = vec![];
    let mut game_versions = vec![];
    for a in &configs.addons {
        game_versions.clear();
        for gv in &a.game_versions {
            game_versions.push(gv.parse::<GameVersion>()?);
        }
        addons.push(parse_json_github_url(&game_versions, &a.url)?);
    }
    Ok(AddonsManagerState::new(&configs.wow_root_path, &addons))
}

fn create_default_json_and_state(addons_json_path: &PathBuf) -> MetalResult<AddonsManagerState> {
    fs::write(&addons_json_path, DEFAULT_JSON)?;
    Ok(AddonsManagerState::new(&None, &vec![]))
}

fn update_json_file(
    filer_loader: &FileSystemAddonsLoader,
    addons_manager_state: &AddonsManagerState,
) -> MetalResult<()> {
    let mut addon_jsons = vec![];
    for (_repo, addon_info) in &addons_manager_state.addon_infos {
        addon_jsons.push(AddonJson {
            url: addon_info.url.clone(),
            game_versions: addon_info
                .game_versions
                .iter()
                .map(|gv| gv.to_string())
                .collect::<Vec<String>>(),
        });
    }
    let configs = Configs {
        wow_root_path: addons_manager_state.wow_root_path.clone(),
        addons: addon_jsons,
    };

    let str_data = serde_json::to_string(&configs)?;
    fs::write(&filer_loader.json_path, str_data)?;
    Ok(())
}

impl AddonsManagerLoader for FileSystemAddonsLoader {
    fn load(&self) -> MetalResult<AddonsManagerState> {
        match fs::read_to_string(&self.json_path) {
            Ok(file_data) => parse_configs(&serde_json::from_str(&file_data)?),
            Err(_err) => create_default_json_and_state(&self.json_path),
        }
    }

    fn set_wow_root_path(&self, addons_manager_state: &mut AddonsManagerState, wow_root_path: &str) -> MetalResult<()> {
        addons_manager_state.wow_root_path = Some(wow_root_path.to_owned());
        update_json_file(self, addons_manager_state)?;
        Ok(())
    }
}
