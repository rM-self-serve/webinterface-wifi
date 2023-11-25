use super::structs::{ABList, ConfigCont, ConfigContOPT, Device, DeviceOPT, Network};
use crate::constants::{DEF_CNTRL_PORT, RUNTIME_ENV, RUNTIME_ENV_DAEMON};
use std::{
    collections::{HashMap, HashSet},
    env,
};

pub fn config(mut conf_opt: ConfigContOPT) -> ConfigCont {
    if conf_opt.daemon_port.is_none() {
        if let Ok(key) = env::var(RUNTIME_ENV) {
            if key == RUNTIME_ENV_DAEMON {
                conf_opt.daemon_port = Some(DEF_CNTRL_PORT);
            }
        }
    }

    ConfigCont {
        network_filter: conf_opt.network_filter.unwrap(),
        ssl_cert_path: conf_opt.ssl_cert_path,
        ssl_priv_path: conf_opt.ssl_priv_path,
        login_path: conf_opt.login_path,
        daemon_port: conf_opt.daemon_port,
    }
}

pub fn device(device_opt: Option<DeviceOPT>) -> Device {
    let default_device = Device::default();
    let Some(device_cont_opt) = device_opt else {
        return default_device;
    };
    let webint_port = match device_cont_opt.webint_port {
        Some(val) => val,
        None => default_device.webint_port,
    };
    let webint_ip = match device_cont_opt.webint_ip {
        Some(val) => val,
        None => default_device.webint_ip,
    };
    let wifi_interface = match device_cont_opt.wifi_interface {
        Some(val) => val,
        None => default_device.wifi_interface,
    };

    Device {
        webint_port: webint_port,
        webint_ip: webint_ip,
        wifi_interface: wifi_interface,
    }
}

pub fn allowlist(allowlist_opt: Option<ABList>) -> Option<Vec<String>> {
    match allowlist_opt {
        None => None,
        Some(val) => val.networks,
    }
}

pub fn blocklist(blocklist_opt: Option<ABList>) -> Option<Vec<String>> {
    match blocklist_opt {
        None => None,
        Some(val) => val.networks,
    }
}

pub fn active_blocked(
    networks_opt: Option<HashMap<String, Network>>,
    ab_list: Option<Vec<String>>,
    network_filter: &str,
) -> (HashMap<String, Network>, HashMap<String, Network>) {
    let Some(mut networks) = networks_opt else {
        return (HashMap::new(), HashMap::new());
    };
    for net in networks.iter_mut() {
        net.1.__name = Some(format!("[networks.{}]", net.0.to_string()));
    }

    if network_filter == "off" {
        return (networks, HashMap::new());
    }

    let mut net_names = vec![];
    for net_name in networks.iter() {
        net_names.push(net_name.0);
    }
    let mut active_nets = HashMap::new();
    let mut blocked_nets = HashMap::new();
    let all_nets: HashSet<String> = net_names.iter().cloned().map(ToString::to_string).collect();
    let ablist_nets: HashSet<String> = ab_list.unwrap().iter().cloned().collect();
    let ntrsctn_nets: HashSet<_> = all_nets.intersection(&ablist_nets).collect();
    let remaining_nets: HashSet<_> = all_nets.difference(&ablist_nets).collect();

    if network_filter == "blocklist" {
        for net_name in remaining_nets.iter() {
            let net_body = networks.get(*net_name).unwrap().clone();
            active_nets.insert(net_name.to_string(), net_body);
        }
        for net_name in ntrsctn_nets.iter() {
            let net_body = networks.get(*net_name).unwrap().clone();
            blocked_nets.insert(net_name.to_string(), net_body);
        }
        return (active_nets, blocked_nets);
    }

    for net_name in ntrsctn_nets.iter() {
        let net_body = networks.get(*net_name).unwrap().clone();
        active_nets.insert(net_name.to_string(), net_body);
    }

    (active_nets, blocked_nets)
}
