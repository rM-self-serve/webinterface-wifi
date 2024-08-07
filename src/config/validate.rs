use super::structs::{ABList, ConfigCont, ConfigContOPT, DeviceOPT, Network};
use crate::{
    auth,
    constants::{DEF_PASS_PATH, DEF_SSL_CERT_PATH, DEF_SSL_PRIV_PATH},
    server::ssl,
};
use log::{error, info, warn};
use std::{
    collections::{HashMap, HashSet},
    net::IpAddr,
    path::Path,
    vec,
};

pub fn config(conf_opt: &Option<ConfigContOPT>) -> bool {
    let Some(conf) = conf_opt else {
        error!("Missing [conf] field.");
        return false;
    };

    let Some(network_filter) = conf.network_filter.as_ref() else {
        error!("Missing 'network_filter' in [conf] field.");
        return false;
    };

    if network_filter != "off" && network_filter != "allowlist" && network_filter != "blocklist" {
        error!("[conf].network_filter must be either 'off', 'allowlist', or 'blocklist'.");
        return false;
    }

    if Path::new(DEF_SSL_CERT_PATH).exists() {
        if let Err(err) = ssl::load_certs(&DEF_SSL_CERT_PATH) {
            error!("{err}");
            return false;
        };
    }
    if Path::new(DEF_SSL_PRIV_PATH).exists() {
        if let Err(err) = ssl::load_private_key(&DEF_SSL_PRIV_PATH) {
            error!("{err}");
            return false;
        };
    }

    if let Some(ssl_cert_path) = conf.ssl_cert_path.as_ref() {
        if let Err(err) = ssl::load_certs(&ssl_cert_path) {
            error!("{err}");
            return false;
        };
    }

    if let Some(ssl_priv_path) = conf.ssl_priv_path.as_ref() {
        if let Err(err) = ssl::load_private_key(&ssl_priv_path) {
            error!("{err}");
            return false;
        };
    }

    if let Some(login_path) = conf.login_path.as_ref() {
        if let Err(err) = auth::pass_hash::load_login(login_path) {
            error!("{err}");
            return false;
        };
    }

    true
}

pub fn device(deviceopt_opt: &Option<DeviceOPT>) -> bool {
    let Some(device_opt) = deviceopt_opt.as_ref() else {
        info!("Missing [device] field, assuming default.");
        return true;
    };

    if device_opt.webint_port.is_none() {
        info!("Missing 'webint_port' in [device] field, assuming default.");
    }

    if device_opt.webint_ip.is_none() {
        info!("Missing 'webint_ip' in [device] field, assuming default.");
    }

    if device_opt.wifi_interface.is_none() {
        info!("Missing 'wifi_interface' in [device] field, assuming default.");
    }

    true
}

pub fn networks(
    network_opt: &Option<HashMap<String, Network>>,
    conf: &ConfigCont,
    network_filter: &str,
) -> bool {
    let Some(networks) = network_opt.as_ref() else {
        if network_filter == "allowlist" {
            error!("Missing networks since [conf].network_filter == {network_filter}.");
            return false;
        }
        return true;
    };

    let mut success = true;
    for name_network in networks.iter() {
        let name = format!("[networks.{}]", name_network.0.to_string());

        success &= network(&name_network.1, conf, &name, false, false);
    }

    success
}

pub fn undefined_net(undefined_net_opt: &Option<Network>, conf: &ConfigCont) -> bool {
    let Some(unl_net) = undefined_net_opt.as_ref() else {
        if conf.network_filter != "allowlist" {
            error!("If [conf].network_filter == '{}', the [undefined_networks] field must be populated.", conf.network_filter);
            return false;
        }
        return true;
    };

    if conf.network_filter == "allowlist" {
        warn!(
            "[undefined_network] will be ignored when [conf].network_filter == '{}'.",
            conf.network_filter
        );
    }

    network(&unl_net, conf, "[undefined_networks]", false, true)
}

pub fn network(
    network: &Network,
    auth: &ConfigCont,
    name: &str,
    is_blocked: bool,
    is_undefined: bool,
) -> bool {
    let mut success = true;

    if network.router_ssid.is_none() && !is_undefined {
        error!("Missing 'router_ssid' in {name} field.");
        success = false;
    }

    if is_blocked {
        return success;
    }

    let login_enforced = match network.login_enforced {
        None => {
            error!("Missing 'login_enforced' in {name} field.");
            success = false;
            false
        }
        Some(val) => val,
    };

    if auth.login_path.as_ref().is_none() && !Path::new(DEF_PASS_PATH).exists() {
        if login_enforced == true {
            error!(
                "Missing login at {DEF_PASS_PATH} or 'login_path' field in [conf] since {name}.login_enforced == true."
            );
            success = false;
        }
    }

    let ssl = match network.ssl {
        None => {
            error!("Missing 'ssl' in {name} field.");
            success = false;
            false
        }
        Some(val) => val,
    };

    if auth.ssl_cert_path.as_ref().is_none() && !Path::new(DEF_SSL_CERT_PATH).exists() {
        if ssl == true {
            error!("Missing ssl cert at {DEF_SSL_CERT_PATH} or 'ssl_cert_path' field in [conf] since {name}.ssl == true.");
            success = false;
        }
    }

    if auth.ssl_priv_path.as_ref().is_none() && !Path::new(DEF_SSL_PRIV_PATH).exists() {
        if ssl == true {
            error!("Missing ssl private key at {DEF_SSL_PRIV_PATH} or 'ssl_priv_path' field in [conf] since {name}.ssl == true.");
            success = false;
        }
    }

    if network.http_redirect_port.as_ref().is_some() && ssl == false {
        warn!("The field 'ssl' in {name} == false but 'http_redirect_port' is set.");
    }

    if network.__name.is_some() {
        error!("The field '__name' in {name} is private.");
        success = false;
    };

    let Some(listen_port) = network.listen_port.as_ref() else {
        error!("Missing 'listen_port' in {name} field.");
        return false;
    };

    let Some(listen_ip) = network.listen_ip.as_ref() else {
        error!("Missing 'listen_ip' in {name} field.");
        return false;
    };

    if listen_ip != "auto" && listen_ip.parse::<IpAddr>().is_err() {
        error!("Invalid 'listen_ip' in {name} field.");
        return false;
    }

    if listen_ip == "0.0.0.0" && *listen_port == 80 {
        error!("Can not run server on 0.0.0.0:80 in {name} field.");
        success = false;
    };

    success
}

pub fn ablist(a_or_b: &str, ablist_opt: &Option<ABList>, network_filter: &str) -> bool {
    if network_filter != a_or_b {
        if ablist_opt.is_some() {
            warn!("the [{a_or_b}] field is present though [conf].network_filter == {network_filter}. \
            The {a_or_b} will be ignored.");
        }
        return true;
    }

    let Some(ablist) = ablist_opt.as_ref() else {
        error!("Missing [{a_or_b}] field.");
        return false;
    };

    let Some(networks) = ablist.networks.as_ref() else {
        error!("Missing 'networks' in [{a_or_b}] field.");
        return false;
    };

    if networks.is_empty() {
        error!("'networks' is empty in [{a_or_b}] field.");
        return false;
    }

    true
}

pub fn net_ablist(
    networks_opt: &Option<HashMap<String, Network>>,
    active_list: &Option<Vec<String>>,
    auth: &ConfigCont,
) -> bool {
    let mut success = true;
    let mut net_names = vec![];
    let Some(networks) = networks_opt.as_ref() else {
        error!(
            "No defined networks while [conf].network_filter == {}",
            auth.network_filter
        );
        return false;
    };

    for net_name in networks.iter() {
        net_names.push(net_name.0);
    }

    let all_nets: HashSet<String> = net_names.iter().cloned().map(ToString::to_string).collect();
    let ablist_nets: HashSet<String> = active_list.as_ref().unwrap().iter().cloned().collect();

    let unused_nets: HashSet<_> = all_nets.difference(&ablist_nets).collect();
    let unused_ablist: HashSet<_> = ablist_nets.difference(&all_nets).collect();
    let ntrsctn_nets: HashSet<_> = all_nets.intersection(&ablist_nets).collect();

    if !unused_nets.is_empty() && auth.network_filter == "allowlist" {
        warn!(
            "Defined networks not used in {}: {:#?}",
            auth.network_filter, unused_nets
        );
    }
    if !unused_ablist.is_empty() {
        warn!(
            "Networks in [{}] not defined: {:#?}",
            auth.network_filter, unused_ablist
        );
    }

    if ntrsctn_nets.is_empty() {
        error!("No networks found from {}.", auth.network_filter);
        success = false;
    }

    let mut is_blocked = false;
    if auth.network_filter == "blocklist" {
        is_blocked = true;
    }

    for net in ntrsctn_nets.iter() {
        let net_body = networks.get(*net).unwrap();
        let name = format!("[networks.{}]", *net);
        success &= network(&net_body, &auth, &name, is_blocked, false);
    }

    success
}
