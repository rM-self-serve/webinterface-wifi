use std::{env, path::PathBuf};

use super::{
    cli::{Cli, Commands},
    execute,
};
use crate::server;
use crate::{
    auth::pass_hash,
    constants::{DEF_CNTRL_PORT, DEF_LOG_LEVEL, RUNTIME_LOG_LVL, _INTERNAL_LOG_NAME},
    netinfo::structs::NetInfo,
};

pub fn run() -> std::io::Result<()> {
    let cli = Cli::new();

    match &cli.command {
        Some(Commands::LocalExec {
            config_path,
            log_level,
        }) => {
            configure_log(log_level);
            let config_path = config_path.as_ref().unwrap();
            server::entry::entry(config_path)?;
        }

        Some(Commands::Edit { config_path }) => {
            configure_log(&None);
            let config_path = config_path.as_ref().unwrap();
            execute::edit(config_path)?;
        }

        Some(Commands::Validate { config_path }) => {
            configure_log(&None);
            let config_path = config_path.as_ref().unwrap();
            execute::validate(config_path)?;
        }

        Some(Commands::Reload {
            config_path,
            cntrl_port,
        }) => {
            configure_log(&None);
            let cntrl_port = cntrl_port.unwrap_or(DEF_CNTRL_PORT);
            let config_path = config_path.as_ref().unwrap();
            execute::reload(config_path, cntrl_port)?;
        }

        Some(Commands::NetInfo { wifi, webint }) => {
            configure_log(&None);
            let wifi = wifi.as_ref().unwrap();
            let webint = webint.as_ref().unwrap();
            NetInfo::from_sync(wifi, webint, true)?;
        }

        Some(Commands::CreateLogin { login_path }) => {
            configure_log(&None);
            let login_path = login_path.as_ref().unwrap();
            pass_hash::create(login_path)?;
        }

        None => {}
    }
    Ok(())
}

fn configure_log(log_level: &Option<PathBuf>) {
    if let Some(loglev) = log_level {
        let local_log = format!("{_INTERNAL_LOG_NAME}={}", loglev.display());
        env::set_var(RUNTIME_LOG_LVL, local_log);
        return log_init(RUNTIME_LOG_LVL);
    }

    if let Ok(loglev) = env::var(RUNTIME_LOG_LVL) {
        let local_log = format!("{_INTERNAL_LOG_NAME}={loglev}");
        env::set_var(RUNTIME_LOG_LVL, local_log);
        return log_init(RUNTIME_LOG_LVL);
    }

    let local_log = format!("{_INTERNAL_LOG_NAME}={DEF_LOG_LEVEL}");
    env::set_var(RUNTIME_LOG_LVL, local_log);
    return log_init(RUNTIME_LOG_LVL);
}

fn log_init(custom: &str) {
    pretty_env_logger::try_init_custom_env(custom).unwrap();
}
