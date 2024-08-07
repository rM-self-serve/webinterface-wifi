use super::{factory::Config, parse, structs::ConfigCore, validate};
use log::{debug, error};
use std::io::{Error, ErrorKind};
use std::{fs::read_to_string, path::PathBuf};

pub fn build_vaild(conf_path: &PathBuf) -> std::io::Result<Config> {
    let conf_str = match read_to_string(conf_path) {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("Unable to read {}\nError: {err}", conf_path.display());
            error!("{err_str}");
            return Err(Error::new(ErrorKind::Other, err_str));
        }
    };
    let conf_core: ConfigCore = match toml::from_str(&conf_str) {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("In {}\n{err}", conf_path.display());
            error!("{err_str}");
            return Err(Error::new(ErrorKind::Other, err_str));
        }
    };
    debug!("{:#?}", conf_core);

    let Some(conf) = process(conf_core) else {
        let err_str = format!("Parse/Validation Error");
        return Err(Error::new(ErrorKind::Other, err_str));
    };

    Ok(conf)
}

fn process(mut conf_core: ConfigCore) -> Option<Config> {
    let mut success = true;

    if !validate::config(&conf_core.conf) {
        return None;
    }
    let conf = parse::config(conf_core.conf.unwrap());

    success &= validate::device(&conf_core.device);
    let device = parse::device(conf_core.device);

    success &= validate::undefined_net(&conf_core.undefined_networks, &conf);

    if !validate::ablist("allowlist", &conf_core.allowlist, &conf.network_filter) {
        return None;
    }
    let allowlist = parse::allowlist(conf_core.allowlist);

    if !validate::ablist("blocklist", &conf_core.blocklist, &conf.network_filter) {
        return None;
    }
    let blocklist = parse::blocklist(conf_core.blocklist);

    let networks_opt = conf_core.networks;
    let mut active_list: Option<Vec<String>> = Some(vec![]);

    if conf.network_filter == "off" {
        success &= validate::networks(&networks_opt, &conf, &conf.network_filter);
    } else {
        if conf.network_filter == "allowlist" {
            active_list = allowlist;
        } else {
            active_list = blocklist;
        }
        success &= validate::net_ablist(&networks_opt, &active_list, &conf);
    }

    if !success {
        return None;
    }

    let (active_nets, blocked_nets) =
        parse::active_blocked(networks_opt, active_list, &conf.network_filter);

    if let Some(net) = conf_core.undefined_networks.as_mut() {
        net.__name = Some("[undefined_networks]".to_string());
    }

    Some(Config {
        auth: conf,
        device: device,
        active_nets: active_nets,
        blocked_nets: blocked_nets,
        undefined_networks: conf_core.undefined_networks,
    })
}
