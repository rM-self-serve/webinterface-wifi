use super::{myhttp, myhttps, structs::SystemState, utils};
use crate::{
    auth,
    config::{factory::Config, structs::Network},
    constants::DEF_PASS_PATH,
    netinfo::{structs::NetInfo, with_config},
};
use async_recursion::async_recursion;
use log::{debug, error};
use std::{net::IpAddr, sync::Arc};
use tokio::{
    sync::{broadcast, Mutex, MutexGuard},
    task::JoinHandle,
};

type SrvrThread = JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;

#[async_recursion]
pub async fn run(
    system_state_arc: Arc<Mutex<SystemState>>,
    send_kill_srvr: broadcast::Sender<()>,
) -> std::io::Result<()> {
    let mut system_state = system_state_arc.lock().await;

    let Some(net_info) = NetInfo::from_config(&system_state.config).await? else {
        reset_state(system_state, send_kill_srvr);
        return Ok(());
    };
    let Some(network) = with_config::find_net(&system_state.config, &net_info, false) else {
        reset_state(system_state, send_kill_srvr);
        return Ok(());
    };

    let conn_hash = utils::conn_hash(&net_info);
    if system_state.connection_hash.is_some() && system_state.connection_hash != Some(conn_hash) {
        debug!("Network changed");
        reset_state(system_state, send_kill_srvr.clone());
        return run(system_state_arc, send_kill_srvr).await;
    }
    // conditions are correct for server to be running

    let Some(srvr_thread) = system_state.srvr_thread.as_ref() else {
        let wlanthread = spawn_server(network, net_info, &system_state.config, send_kill_srvr)?;
        system_state.srvr_thread = Some(wlanthread);
        system_state.connection_hash = Some(conn_hash);
        return Ok(());
    };
    if srvr_thread.is_finished() {
        debug!("Server thread is finished");
        reset_state(system_state, send_kill_srvr.clone());
        return run(system_state_arc, send_kill_srvr).await;
    }

    Ok(())
}

fn spawn_server(
    network: Network,
    net_info: NetInfo,
    config: &Config,
    send_kill_srvr: broadcast::Sender<()>,
) -> std::io::Result<SrvrThread> {
    let session_key = random_string::generate(16, random_string::charsets::ALPHANUMERIC);

    let webint_addr =
        utils::string2scktaddr(net_info.webint_ip.to_string(), &config.device.webint_port)?;

    let wlanip = resolve_wlanip(network.listen_ip.as_ref(), net_info.wifi_ntrfc);
    let listen_addr = utils::string2scktaddr(wlanip, network.listen_port.as_ref().unwrap())?;

    let mut pass_hash: Option<String> = None;
    if network.login_enforced.unwrap() {
        let def_login_path = DEF_PASS_PATH.to_string();
        let login_path = config.auth.login_path.as_ref().unwrap_or(&def_login_path);
        pass_hash = match auth::pass_hash::load_login(login_path) {
            Ok(cont) => Some(cont),
            Err(err) => {
                let err_str = format!("Could not load password: {}", err);
                error!("{err_str}");
                return Err(err);
            }
        }
    }

    let auth = config.auth.clone();
    let wlanthread = tokio::spawn(async move {
        if network.ssl.as_ref().unwrap() == &true {
            myhttps::start_server(
                listen_addr,
                webint_addr,
                network,
                auth,
                pass_hash,
                send_kill_srvr,
                session_key,
            )
            .await
        } else {
            myhttp::start_server(
                listen_addr,
                webint_addr,
                pass_hash,
                send_kill_srvr,
                session_key,
            )
            .await
        }
    });

    Ok(wlanthread)
}

fn resolve_wlanip(net_ip: Option<&String>, wifi_ip: IpAddr) -> String {
    let mut wlanip = net_ip.unwrap().to_owned();
    if wlanip == "auto" {
        wlanip = wifi_ip.to_string();
    }

    wlanip
}

fn reset_state(
    mut server_state: MutexGuard<'_, SystemState>,
    send_kill_srvr: broadcast::Sender<()>,
) {
    if server_state.srvr_thread.is_some() {
        let _ = send_kill_srvr.send(());
    }
    server_state.srvr_thread = None;
    server_state.connection_hash = None;
}
