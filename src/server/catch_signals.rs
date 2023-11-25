use super::utils::time_now;
use crate::constants::{SIGINT, SIGTERM, SIGUSR1, TCP_BUFFER_SIZE};
use log::{debug, error, info, warn};
use tokio::time::sleep;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{
    net::TcpStream,
    signal::unix::{signal, SignalKind},
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;

const FIVE_MINUTES: u64 = 5 * 60;

pub fn unix(
    send_kill_sleep: broadcast::Sender<i32>,
    cancel_token: CancellationToken,
) -> Result<JoinHandle<Result<()>>> {
    let mut sig_int = signal(SignalKind::interrupt())?;
    let mut sig_term = signal(SignalKind::terminate())?;

    let a = tokio::spawn(async move {
        tokio::select! {
            _ = cancel_token.cancelled() => {},
            _ = sig_int.recv() => {
                let _ = send_kill_sleep.send(SIGINT);
            },
            _ = sig_term.recv() =>  {
                let _ = send_kill_sleep.send(SIGTERM);
            },
        };
        debug!("Signal listener shutdown");
        Ok(())
    });
    Ok(a)
}

pub fn cntrl_lstnr(
    send_kill_sleep: broadcast::Sender<i32>,
    cancel_token: CancellationToken,
    cntrl_port_opt: Option<u16>,
) -> Option<JoinHandle<Result<()>>> {
    let Some(cntrl_port) = cntrl_port_opt.clone() else {
        return None;
    };

    let res = tokio::spawn(async move {
        // create new listener if stream read error
        // exit if >= 5 error in 5 minutes
        let mut last_err_time = time_now();
        let mut error_count: u8 = 0;
        loop {
            if let Err(err) =
                connect(send_kill_sleep.clone(), cancel_token.clone(), cntrl_port).await
            {
                warn!("Error: {}", err);
                let time_now = time_now();
                if time_now - last_err_time > Duration::from_secs(FIVE_MINUTES) {
                    last_err_time = time_now;
                    error_count = 0;
                }

                error_count += 1;

                if error_count >= 5 {
                    return Err(err);
                }
                
                sleep(Duration::from_secs(10)).await;
                continue;
            };

            return Ok(());
        }
    });

    Some(res)
}

async fn connect(
    send_kill_sleep: broadcast::Sender<i32>,
    cancel_token: CancellationToken,
    cntrl_port: u16,
) -> Result<()> {
    let addr_str = format!("127.0.0.1:{cntrl_port}");
    let listener = match tokio::net::TcpListener::bind(&addr_str).await {
        Ok(val) => val,
        Err(err) => {
            error!("Error starting control listener on {}\n{}", addr_str, err);
            return Err(err);
        }
    };
    debug!("Running control listener on {cntrl_port}");
    loop {
        tokio::select! {
            _ = cancel_token.cancelled() => {
                debug!("Control listener graceful shutdown");
                return Ok(())
            },
            res = listener.accept() => {
                let res = match res {
                    Ok(val) => val,
                    Err(err) => {
                        error!("{err}");
                        break
                    }
                };
                if let Err(err) = handle_tcp(res.0, send_kill_sleep.clone()).await {
                    error!("{err}");
                    break
                };
            }
        }
    }
    let err_str = "Control listener error, shut down";
    debug!("{err_str}");

    return Err(Error::new(ErrorKind::Other, err_str));
}

async fn handle_tcp(stream: TcpStream, send_kill_sleep: broadcast::Sender<i32>) -> Result<()> {
    stream.readable().await?;
    let mut buffer = [0; TCP_BUFFER_SIZE];
    match stream.try_read(&mut buffer) {
        Ok(0) => return Err(Error::new(ErrorKind::Other, "buffer len 0")),
        Ok(_) => {}
        Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    let result = match std::str::from_utf8(&buffer) {
        Ok(result) => result.replace('\0', ""),
        Err(err) => {
            error!("{}", err);
            return Ok(());
        }
    };

    if result == "reload" {
        info!("Recieved reload signal");
        if let Err(err) = stream.try_write("Config Reloaded".as_bytes()) {
            error!("Responding to TCP stream: {}", err);
            return Err(err);
        }
        let _ = send_kill_sleep.send(SIGUSR1);
    } else {
        error!("Recieved unrecognized signal: {:#?}", result);
        std::io::stdout().flush().unwrap();
    }

    Ok(())
}
