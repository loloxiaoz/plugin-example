use crate::error::RResult;
use crate::plugin::PluginToLoad;
use crate::utils::vec_from_map::VecFromMap;

use abi_stable::std_types::RVec;
use common::PluginId;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::value::RawValue;

lazy_static! {
    pub static ref CONFIG: Config = load().unwrap();
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub plugins: RVec<PluginToLoad>,
    pub commands: VecFromMap<PluginId, Box<RawValue>>,
}

pub fn load() -> RResult<Config> {
    let path = { "./data/app_config.json".to_string() };
    let file_contents = std::fs::read_to_string(&path)?;
    let config = serde_json::from_str(&file_contents)?;
    Ok(config)
}
