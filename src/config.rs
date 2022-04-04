use std::{fs, path::Path};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::filter_list::FilterList;

/// Config contains all relevant information to start the data processing.
/// Relevant information is considered most of all data sources and destinations
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub lists: Vec<FilterList>,
    pub tmp_dir: String,
    pub out_dir: String,
}

impl Config {
    /// Populates the Config struct from a json file
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config: Config =
            serde_json::from_str(&contents).with_context(|| "invalid configuration file")?;
        let Self {
            lists,
            tmp_dir,
            out_dir,
        } = config;
        Ok(Self {
            lists,
            tmp_dir,
            out_dir,
        })
    }
}
