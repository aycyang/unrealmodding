use std::fs;

use log::warn;
use unreal_modintegrator::metadata::SyncMode;

use crate::game_mod::SelectedVersion;
use crate::version::{GameBuild, Version};
use crate::AppData;

pub(crate) fn auto_pick_versions(data: &mut AppData) {
    // check that there is even a version to pick
    let mut to_remove = Vec::new();
    for (mod_id, game_mod) in data.game_mods.iter() {
        if game_mod.versions.is_empty() {
            to_remove.push(mod_id.to_owned());
        }
    }
    for mod_id in to_remove {
        data.game_mods.remove(&mod_id);
    }

    for (_, game_mod) in data.game_mods.iter_mut() {
        // if using latest indirect, find version
        if let SelectedVersion::LatestIndirect(None) = game_mod.selected_version {
            let mut versions = game_mod.versions.keys().collect::<Vec<&Version>>();
            versions.sort();
            game_mod.selected_version =
                SelectedVersion::LatestIndirect(Some(**versions.last().unwrap()));
        }
    }
}

/// Sets top-level fields from the metadata of the selected version.
/// Will panic if any versions are LatestIndirect with no version set.
pub(crate) fn set_mod_data_from_version(data: &mut AppData) {
    for (mod_id, game_mod) in data.game_mods.iter_mut() {
        let use_version = match game_mod.selected_version {
            SelectedVersion::Latest(version) => version,
            SelectedVersion::Specific(version) => version,
            SelectedVersion::LatestIndirect(version) => version.unwrap(),
        };

        let version_data = game_mod.versions.get(&use_version).unwrap();
        let metadata = version_data.metadata.as_ref().unwrap();

        game_mod.name = metadata.name.to_owned();
        game_mod.author = metadata.author.to_owned();
        game_mod.description = metadata.description.to_owned();
        game_mod.game_build = match metadata.game_build {
            Some(ref game_build) => match GameBuild::try_from(game_build) {
                Ok(game_build) => Some(game_build),
                Err(_) => {
                    warn!("Failed to parse game build for mod {:?}", mod_id);
                    None
                }
            },
            None => None,
        };
        game_mod.sync = metadata.sync.unwrap_or(SyncMode::ServerAndClient);
        game_mod.homepage = metadata.homepage.clone();
        game_mod.download = metadata.download.clone();
        let path = data
            .data_path
            .as_ref()
            .unwrap()
            .join(version_data.file_name.clone());
        game_mod.size = fs::metadata(&path).unwrap().len();
    }
}