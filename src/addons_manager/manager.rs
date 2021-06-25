use crate::error::{MetalError, MetalResult};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum GameVersion {
    Retail,
    Classic,
    Bcc,
}

impl FromStr for GameVersion {
    fn from_str(input: &str) -> MetalResult<Self> {
        match input {
            "retail" => Ok(GameVersion::Retail),
            "classic" => Ok(GameVersion::Classic),
            "bcc" => Ok(GameVersion::Bcc),
            _ => Err(MetalError::new(&format!(
                "Unknown game version '{}'",
                input
            ))),
        }
    }

    type Err = MetalError;
}

impl Display for GameVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            GameVersion::Retail => "retail",
            GameVersion::Classic => "classic",
            GameVersion::Bcc => "bcc",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
pub struct AddonInfo {
    pub username: String,
    pub url: String,
    pub game_versions: Vec<GameVersion>,
}

impl AddonInfo {
    pub fn new(url: &str, username: &str, game_version: &Vec<GameVersion>) -> Self {
        Self {
            url: url.to_owned(),
            username: username.to_owned(),
            game_versions: game_version.clone(),
        }
    }
}

#[derive(Debug)]
pub struct AddonsManagerState {
    pub addon_infos: HashMap<String, AddonInfo>,
    pub wow_root_path: Option<String>,
}

impl AddonsManagerState {
    pub fn new(wow_root_path: &Option<String>, addon_infos: &Vec<(String, AddonInfo)>) -> Self {
        let mut addons = HashMap::new();
        for (k, v) in addon_infos {
            addons.insert(k.clone(), v.clone());
        }
        Self {
            addon_infos: addons,
            wow_root_path: wow_root_path.clone(),
        }
    }
}

pub trait AddonsManagerLoader {
    fn load(&self) -> MetalResult<AddonsManagerState>;
    fn set_wow_root_path(&self, addons_manager_sgate: &mut AddonsManagerState, wow_root_path: &str) -> MetalResult<()>;
}
