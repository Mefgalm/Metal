#[derive(Clone, Debug)]
pub enum GameVersion {
    Retail,
    Classic,
    Bcc,
}

#[derive(Clone, Debug)]
struct AddonContent {
    game_version: GameVersion,
    zip_file_path: String,
    unzip_content: Vec<String>,
}

#[derive(Clone, Debug)]
struct AddonLoadedState {
    version: String,
    addon_contents: Vec<AddonContent>,
}

#[derive(Clone, Debug)]
enum AddonState {
    NotLoaded,
    Loaded(AddonLoadedState),
}

#[derive(Clone, Debug)]
pub struct AddonInfo {
    username: String,
    repo: String,
    state: AddonState,
}

#[derive(Debug)]
pub struct AddonsManagerState {
    addon_infos: Vec<AddonInfo>,
}

impl AddonsManagerState {
    pub fn new(addon_infos: &Vec<AddonInfo>) -> Self {
        Self { addon_infos: addon_infos.clone() }
    }
}

pub trait AddonsManagerLoader {
    fn load() -> AddonsManagerState;
}
