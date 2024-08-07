use crate::{
    config::{factory::Config, structs::Network},
    netinfo::structs::NetInfo,
};
use log::debug;

pub fn find_net(server_config: &Config, net_info: &NetInfo, is_cli: bool) -> Option<Network> {
    if let Some(val) = match_net(&server_config, &net_info.ssid, true) {
        print_or_dbug(&format!("Network active:\n{val}",), is_cli);
        return Some(val);
    };

    if let Some(blocked_net) = match_net(&server_config, &net_info.ssid, false) {
        print_or_dbug(&format!("Network is in blocklist:\n{blocked_net}"), is_cli);
        return None;
    };

    if server_config.auth.network_filter != "allowlist" {
        print_or_dbug(
            &format!(
                "Network active:\n{}",
                server_config.undefined_networks.clone().unwrap()
            ),
            is_cli,
        );
        return server_config.undefined_networks.clone();
    }
    print_or_dbug("Network not in [allowlist]", is_cli);

    None
}

fn match_net(config: &Config, router_ssid: &str, is_active: bool) -> Option<Network> {
    let search_net;
    if is_active {
        search_net = &config.active_nets
    } else {
        search_net = &config.blocked_nets
    }

    for network in search_net {
        let nrouter_ssid = network.1.router_ssid.as_ref();

        if nrouter_ssid.is_some() && nrouter_ssid.unwrap() == router_ssid {
            return Some(network.1.clone());
        }
    }

    None
}

fn print_or_dbug(output: &str, is_cli: bool) {
    if is_cli {
        println!("{}", output);
    } else {
        debug!("{}", output);
    }
}
