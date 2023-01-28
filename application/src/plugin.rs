use abi_stable::{
    library::{lib_header_from_path, LibraryError, LibrarySuffix, RawLibrary},
    std_types::{RErr, ROk, RVec},
};
use common::{Error as AppError, PluginFactory_Ref, PluginId, PluginType};
use core_extensions::SelfOps;
use serde::Deserialize;
use std::{
    collections::HashMap,
    io,
    path::{Path, PathBuf},
};

use crate::app::ApplicationState;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PluginToLoad {
    Named(String),
    WithRename {
        #[serde(alias = "name")]
        named: String,
        #[serde(alias = "renamed")]
        rename: Option<String>,
    },
}

/// Returns the path the plugin will be loaded from.
fn compute_plugin_path(base_name: &str) -> io::Result<PathBuf> {
    let debug_dir = "../target/debug/".as_ref_::<Path>().into_::<PathBuf>();
    let release_dir = "../target/release/".as_ref_::<Path>().into_::<PathBuf>();

    let debug_path = RawLibrary::path_in_directory(&debug_dir, base_name, LibrarySuffix::NoSuffix);
    let release_path =
        RawLibrary::path_in_directory(&release_dir, base_name, LibrarySuffix::NoSuffix);

    match (debug_path.exists(), release_path.exists()) {
        (false, false) => debug_path,
        (true, false) => debug_path,
        (false, true) => release_path,
        (true, true) => {
            if debug_path.metadata()?.modified()? < release_path.metadata()?.modified()? {
                release_path
            } else {
                debug_path
            }
        }
    }
    .piped(Ok)
}

pub fn check(plugins: &RVec<PluginToLoad>, state: &mut ApplicationState) -> Vec<PluginId> {
    let mut nonexistent_files = Vec::<(String, io::Error)>::new();
    let mut library_errs = Vec::<(String, LibraryError)>::new();
    let mut loaded_libraries = Vec::<PluginId>::new();

    for plug in plugins {
        let (named, rename) = match plug {
            PluginToLoad::Named(named) => ((*named).clone(), None),
            PluginToLoad::WithRename { named, rename } => ((*named).clone(), rename.clone()),
        };
        let library_path: PathBuf = match compute_plugin_path(named.as_ref()) {
            Ok(x) => x,
            Err(e) => {
                nonexistent_files.push((named.clone(), e));
                continue;
            }
        };

        let res = (|| {
            let header = lib_header_from_path(&library_path)?;
            header.init_root_module::<PluginFactory_Ref>()
        })();

        let root_module = match res {
            Ok(x) => x,
            Err(e) => {
                library_errs.push((named.clone(), e));
                continue;
            }
        };

        let name_key = rename.unwrap_or_else(|| named.clone());

        let plugin_id = PluginId::from(name_key);

        loaded_libraries.push(plugin_id.clone());
        state.id_map.insert(plugin_id, root_module);
    }

    if !nonexistent_files.is_empty() {
        for (name, e) in nonexistent_files {
            eprintln!(
                "Could not load librarr: {}, because of this error: {}",
                name, e
            )
        }
    }

    if !library_errs.is_empty() {
        for (name, e) in library_errs {
            eprintln!(
                "Could not load librarr: {}, because of this error: {}",
                name, e
            )
        }
    }
    loaded_libraries
}

pub fn load(
    plugins: &mut HashMap<PluginId, PluginType>,
    state: &mut ApplicationState,
    loaded_libraries: Vec<PluginId>,
) {
    let mut plugin_new_errs = Vec::<(PluginId, AppError)>::new();
    for plugin_id in loaded_libraries {
        let mod_ref = state.id_map.get_mut(&plugin_id).unwrap();
        let plugin_constructor = mod_ref.new();

        let plugin = match plugin_constructor(state.sender.clone(), plugin_id.clone()) {
            ROk(x) => x,
            RErr(e) => {
                plugin_new_errs.push((plugin_id.clone(), e));
                continue;
            }
        };

        plugins.insert(plugin_id.clone(), plugin);
        println!("load {:?} success", plugin_id);
    }

    if !plugin_new_errs.is_empty() {
        for (plugin_id, e) in plugin_new_errs {
            println!(
                "Could not instantiate plugin: {:?}, because of this error: {}",
                plugin_id, e
            )
        }
        std::process::exit(1);
    }
}
