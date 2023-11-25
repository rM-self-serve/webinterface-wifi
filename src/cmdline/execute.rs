use crate::config::factory::Config;
use crate::constants::{CMD_ENV, DEF_EDITOR, TCP_BUFFER_SIZE};
use crate::netinfo;
use colored::Colorize;
use log::error;
use std::env;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

pub fn validate(config_path: &PathBuf) -> std::io::Result<()> {
    let server_config = Config::init(config_path)?;
    let wifi = &server_config.device.wifi_interface;
    let webint = &server_config.device.webint_ip;
    let sep = "----".yellow();
    println!("{}{}{}", sep, "MOCK RUN".yellow(), sep);
    let net_info_opt = match netinfo::structs::NetInfo::from_sync(wifi, webint, true) {
        Ok(val) => val,
        Err(err) => {
            error!("{} {err}", "While retreiving network info:".red());
            return Err(err);
        }
    };
    if let Some(net_info) = net_info_opt {
        netinfo::with_config::find_net(&server_config, &net_info, true);
        return Ok(());
    }

    Ok(())
}

pub fn edit(config_path: &PathBuf) -> std::io::Result<()> {
    println!("Config file at: {}", config_path.display());
    let mut editor = env::var("EDITOR").unwrap_or(DEF_EDITOR.to_string());
    editor = format!("{editor}");

    if !Path::new(config_path).exists() {
        let err_str = "Config file does not exist";
        error!("{err_str}");
        return Err(Error::new(ErrorKind::Other, err_str));
    }

    let bytes = std::fs::read(config_path).unwrap();
    let og_hash = sha256::digest(&bytes);

    let Ok(mut cmd) = std::process::Command::new(CMD_ENV)
        .arg(editor.clone())
        .arg(config_path.display().to_string())
        .spawn()
    else {
        let err_str = format!("Could not find editor: {editor}");
        error!("{err_str}");
        return Err(Error::new(ErrorKind::Other, err_str));
    };

    if let Err(err) = cmd.wait() {
        let err_str = format!("Editor returned a non-zero status: {err}");
        error!("{err_str}");
        return Err(Error::new(ErrorKind::Other, err_str));
    };

    let bytes = std::fs::read(config_path).unwrap();
    let new_hash = sha256::digest(&bytes);

    if og_hash != new_hash {
        println!("Changes applied to config")
    } else {
        println!("No changes to config")
    }

    // Vailidate config
    Config::init(config_path)?;
    Ok(())
}

pub fn reload(config_path: &PathBuf, cntrl_port: u16) -> std::io::Result<()> {
    if let Err(err) = Config::init(config_path) {
        println!("{}", "Daemon not updated".red());
        return Err(err);
    };
    let address = format!("127.0.0.1:{}", cntrl_port);
    let mut stream = match TcpStream::connect(address) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", "Could not connect to webinterface-wifi daemon.".red());
            return Err(err);
        }
    };
    stream.write("reload".as_bytes())?;

    let mut buffer = [0; TCP_BUFFER_SIZE];
    let _ = stream.read(&mut buffer)?;

    let result = match std::str::from_utf8(&buffer) {
        Ok(result) => result.replace('\0', ""),
        Err(err) => {
            let err_str = format!("Empty response: {err}");
            error!("{err_str}");
            return Err(Error::new(ErrorKind::Other, err_str));
        }
    };

    if result == "" {
        let err_str = format!("Empty response");
        error!("{err_str}");
        return Err(Error::new(ErrorKind::Other, err_str));
    }

    println!("{}", result);

    Ok(())
}
