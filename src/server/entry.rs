use super::{catch_signals, server_loop, structs::SystemState, utils};
use crate::{config::factory::Config, constants::SIGUSR1};
use std::io::Result;
use std::{path::PathBuf, sync::Arc};
use tokio::{
    sync::{broadcast, Mutex},
    time::{sleep, Duration},
};
use tokio_util::sync::CancellationToken;

type BrdCst = (broadcast::Sender<()>, broadcast::Receiver<()>);
type BrdCsti32 = (broadcast::Sender<i32>, broadcast::Receiver<i32>);

#[tokio::main]
pub async fn entry(config_path: &PathBuf) -> Result<()> {
    let config = Config::init(config_path)?;
    let daemon_port = config.auth.daemon_port.clone();
    let server_state_arc = SystemState::init(config);
    let cancel_token = CancellationToken::new();
    let (send_kill_sleep, rec_kill_sleep): BrdCsti32 = broadcast::channel(16);
    let unix = catch_signals::unix(send_kill_sleep.clone(), cancel_token.clone())?;
    let cl = catch_signals::cntrl_lstnr(send_kill_sleep.clone(), cancel_token.clone(), daemon_port);
    let (send_kill_srvr, _): BrdCst = broadcast::channel(17);

    let deployed = deploy(
        config_path,
        rec_kill_sleep,
        send_kill_srvr,
        server_state_arc.clone(),
    )
    .await;

    utils::cleanup(server_state_arc, cancel_token, unix, cl).await;
    deployed?;

    Ok(())
}

async fn deploy(
    config_path: &PathBuf,
    mut rec_kill_sleep: broadcast::Receiver<i32>,
    send_kill_srvr: broadcast::Sender<()>,
    mut server_state_arc: Arc<Mutex<SystemState>>,
) -> Result<()> {
    loop {
        if let Err(err) = server_loop::run(server_state_arc.clone(), send_kill_srvr.clone()).await {
            let _ = send_kill_srvr.clone().send(());
            return Err(err);
        };

        tokio::select! {
            _ = sleep(Duration::from_secs(10)) => {},
            Ok(sig) = rec_kill_sleep.recv() => {
                let _ = send_kill_srvr.clone().send(());
                if sig == SIGUSR1 {
                    let config_reload = Config::init(config_path)?;
                    server_state_arc = SystemState::init(config_reload);
                    continue;
                }
                return Ok(());
            }
        }
    }
}
