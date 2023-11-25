use super::structs::{ConfigCont, Device, Network};
use crate::config::extract;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub auth: ConfigCont,
    pub device: Device,
    pub active_nets: HashMap<String, Network>,
    pub blocked_nets: HashMap<String, Network>,
    pub undefined_networks: Option<Network>,
}

impl Config {
    pub fn init(conf_path: &PathBuf) -> std::io::Result<Config> {
        match extract::build_vaild(conf_path) {
            Ok(val) => {
                println!("{}", "Config Valid".green());
                return Ok(val);
            }
            Err(err) => {
                println!("{}", "Fix errors and retry".red());
                return Err(err);
            }
        };
    }
}

impl Config {}
