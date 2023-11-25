use crate::{netinfo::structs::NetInfo, server::structs::SystemState};
use log::error;
use std::io::{Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;


pub fn time_now() -> Duration {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
}

pub fn conn_hash(net_info: &NetInfo) -> u64 {
    let mut s = DefaultHasher::new();
    net_info.hash(&mut s);
    s.finish()
}

pub fn string2scktaddr(ip: String, port: &u16) -> std::io::Result<SocketAddr> {
    let sstr = ip.clone() + ":" + &port.to_string();
    match sstr.parse() {
        Ok(scktddr) => return Ok(scktddr),
        Err(err) => {
            error!(
                "Error parsing ip address. ip: {} port: {} error: {}",
                ip, port, err
            );
            return Err(Error::new(ErrorKind::Other, err.to_string()));
        }
    };
}

pub async fn cleanup(
    server_state_arc: Arc<Mutex<SystemState>>,
    cancel_token: CancellationToken,
    unix: JoinHandle<std::io::Result<()>>,
    cl: Option<JoinHandle<std::io::Result<()>>>,
) {
    cancel_token.cancel();
    let a = cancel_token.clone();
    tokio::select! {
        _ = async move {
            let _ = tokio::join!(
                a.cancelled(),
                clean_srvr(server_state_arc),
                unix
            );
            if let Some(cl) = cl {
                let _ = cl.await;
            }
        } => {},
        _ = sleep(Duration::from_millis(1000)) => {},
    }
}

async fn clean_srvr(server_state_arc: Arc<Mutex<SystemState>>) {
    let server_state = server_state_arc.lock().await;
    for i in 0..50 {
        match server_state.srvr_thread.as_ref() {
            Some(srvr_thread) => {
                if srvr_thread.is_finished() {
                    break;
                }
                if i == 15 {
                    srvr_thread.abort();
                }
                sleep(Duration::from_millis(20)).await;
            }
            None => break,
        }
    }
}
