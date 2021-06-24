use crate::addons_manager::*;

const ADDONS_JSON: &str = "addons.json";
const ADDONS_LOCK: &str = "addons.lock";

pub struct FileSystemAddonsLoader {
}

impl AddonsManagerLoader for FileSystemAddonsLoader {
    fn load() -> AddonsManagerState {
        AddonsManagerState::new(&vec![])
    }
}
