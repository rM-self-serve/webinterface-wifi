use crate::constants::{CLI_ABOUT, DEF_CNFG_PATH, DEF_PASS_PATH, DEF_WEBINT_IP, DEF_WIFI_INT};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = CLI_ABOUT, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli::parse()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Edit config file
    Edit {
        /// Path to config file
        #[arg(short, long, value_names=["path"], default_value=DEF_CNFG_PATH)]
        config_path: Option<PathBuf>,
    },

    /// Validate config file
    Validate {
        /// Path to config file
        #[arg(short, long = "config", value_names=["path"], default_value=DEF_CNFG_PATH)]
        config_path: Option<PathBuf>,
    },

    /// Reload daemon to read updated config file
    #[command(name = "reload")]
    Reload {
        /// Path to config file
        #[arg(short, long = "config", value_names=["path"], default_value=DEF_CNFG_PATH)]
        config_path: Option<PathBuf>,
        /// Advanced use case, see docs
        #[arg(long = "control-port", value_names=["port"])]
        cntrl_port: Option<u16>,
    },

    /// Current network information
    #[command(name = "net-info")]
    NetInfo {
        /// Wifi interface
        #[arg(
            long,
            default_value=DEF_WIFI_INT,
            value_names = ["interface"]
        )]
        wifi: Option<String>,

        /// Webint interface
        #[arg(
            long,
            default_value=DEF_WEBINT_IP,
            value_names = ["interface"]
        )]
        webint: Option<String>,
    },

    /// Creates a user and password for use with webinterface-wifi
    CreateLogin {
        /// Path to login file
        #[arg(short = 'p', long = "path", value_names=["file"], default_value=DEF_PASS_PATH)]
        login_path: Option<PathBuf>,
    },

    /// Start webinterface-wifi in current shell
    #[command(name = "local-exec")]
    LocalExec {
        /// Path to config file
        #[arg(short, long = "config", value_names=["path"], default_value=DEF_CNFG_PATH)]
        config_path: Option<PathBuf>,

        /// <debug, info, warn, error> [default: warn]
        #[arg(short, long = "log-level", value_names=["level"])]
        log_level: Option<PathBuf>,
    },
}
