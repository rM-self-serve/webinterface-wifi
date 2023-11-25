use super::request;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Server,
};
use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::broadcast;
use log::error;

pub async fn start_server(
    listen_addr: SocketAddr,
    webint_addr: SocketAddr,
    pass_hash: Option<String>,
    send_kill_srvr: broadcast::Sender<()>,
    session_key: String
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        let pass_hash_clone = pass_hash.clone();
        let session_key_clone = session_key.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                request::handle(
                    remote_addr.clone(),
                    webint_addr.clone(),
                    pass_hash_clone.clone(),
                    session_key_clone.clone(),
                    req,
                )
            }))
        }
    });

    let server = match Server::try_bind(&listen_addr) {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("Error starting server on {}\n{}", &listen_addr, err);
            error!("{err_str}");
            return Err(Box::new(err));
        }
    };
    println!("Running server on http://{:?}", listen_addr);
    let graceful = server.serve(service).with_graceful_shutdown(async {
        let mut rec_kill_srvr = broadcast::Sender::subscribe(&send_kill_srvr);
        let _ = rec_kill_srvr.recv().await;
        println!("Gracefully shutting down server");
    });
    if let Err(err) = graceful.await {
        error!("Server error: {}", err);
        return Err(Box::new(err));
    }
    Ok(())
}
