use super::{request, ssl};
use crate::{config::structs::{ConfigCont, Network}, constants::{DEF_SSL_CERT_PATH, DEF_SSL_PRIV_PATH}};
use http::Request;
use hyper::{
    server::conn::AddrIncoming,
    service::{make_service_fn, service_fn},
    Server,
};
use hyper::{Body, Response, StatusCode};
use hyper_rustls::acceptor::TlsStream;
use log::{debug, error};
use std::{convert::Infallible, net::SocketAddr};
use tokio::{sync::broadcast, task::JoinHandle};

pub async fn start_server(
    listen_addr: SocketAddr,
    webint_addr: SocketAddr,
    net: Network,
    auth: ConfigCont,
    pass_hash: Option<String>,
    send_kill_srvr: broadcast::Sender<()>,
    session_key: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bind = match AddrIncoming::bind(&listen_addr) {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("Error starting server on {}\n{}", &listen_addr, err);
            error!("{err_str}");
            return Err(Box::new(err));
        }
    };

    let def_ssl_cert = DEF_SSL_CERT_PATH.to_string();
    let def_ssl_priv = DEF_SSL_PRIV_PATH.to_string();
    let ssl_cert = auth.ssl_cert_path.as_ref().unwrap_or(&def_ssl_cert);
    let ssl_priv = auth.ssl_priv_path.as_ref().unwrap_or(&def_ssl_priv);

    let acceptor = match ssl::acceptor(
        ssl_cert,
        ssl_priv,
        bind,
    )
    .await
    {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("Error parsing SSL certificates: {}", err);
            error!("{err_str}");
            return Err(Box::new(err));
        }
    };

    let mut rdrct_opt: Option<JoinHandle<Result<_, Box<dyn std::error::Error + Send + Sync>>>> = None;
    if let Some(http_redirect_port) = net.http_redirect_port {
        let send_kill_srvr_clone = send_kill_srvr.clone();
        let listen_addr_clone = listen_addr.clone();
        rdrct_opt = Some(tokio::spawn(async move {
            http_redirect(listen_addr_clone, http_redirect_port, send_kill_srvr_clone).await
        }));
    }

    let service = make_service_fn(|conn: &TlsStream| {
        let remote_addr = conn.io().unwrap().remote_addr().ip();
        let pass_hash_clone = pass_hash.clone();
        let session_key_clone = session_key.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                request::handle(
                    remote_addr,
                    webint_addr,
                    pass_hash_clone.clone(),
                    session_key_clone.clone(),
                    req,
                )
            }))
        }
    });

    let server = Server::builder(acceptor).serve(service);
    println!("Running server on https://{:?}", listen_addr);
    let graceful = server.with_graceful_shutdown(async {
        let mut rec_kill_srvr = broadcast::Sender::subscribe(&send_kill_srvr);
        let _ = rec_kill_srvr.recv().await;
        println!(
            "Gracefully shutting down server on https://{:?}",
            listen_addr
        );
    });
    let srvr_res = graceful.await;

    if let Some(rdrct) = rdrct_opt {
        let _ = send_kill_srvr.send(());
        let _ = rdrct.await;
    }

    if let Err(err) = srvr_res {
        let err_str = format!("Https server error: {}", err);
        error!("{err_str}");
        return Err(Box::new(err));
    }


    Ok(())
}

async fn http_redirect(
    mut listen_addr: SocketAddr,
    http_port: u16,
    send_kill_srvr: broadcast::Sender<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tls_port = listen_addr.port();

    let service = make_service_fn(|_| async move {
        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
            debug!("request: {:#?}", req);
            let tls_str_res = replace_domain_tls(req, tls_port);
            async move {
                Ok::<_, Infallible>(match tls_str_res {
                    Ok(tls_str) => Response::builder()
                        .header("Location", tls_str)
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .body(Body::empty())
                        .unwrap(),
                    Err(err) => Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(err))
                        .unwrap(),
                })
            }
        }))
    });

    listen_addr.set_port(http_port);
    let server = match Server::try_bind(&listen_addr) {
        Ok(val) => val,
        Err(err) => {
            let err_str = format!("Error starting server on {}\n{}", &listen_addr, err);
            error!("{err_str}");
            return Err(Box::new(err));
        }
    };
    println!("Running redirect server on http://{:?}", listen_addr);
    let graceful = server.serve(service).with_graceful_shutdown(async {
        let mut rec_kill_srvr = broadcast::Sender::subscribe(&send_kill_srvr);
        let _ = rec_kill_srvr.recv().await;
        println!(
            "Gracefully shutting down http redirect server on http://{:?}",
            listen_addr
        );
    });
    if let Err(err) = graceful.await {
        let err_str = format!("Http redirect server error: {}", err);
        error!("{err_str}");
        return Err(Box::new(err));
    }

    Ok(())
}

fn replace_domain_tls(req: Request<Body>, tls_port: u16) -> Result<String, String> {
    let uri = req.uri().to_string();

    // uri format such as "/xx/xx/" and header contains host
    if let Some(host) = req.headers().get("host") {
        let domain_str = match host.to_str() {
            Ok(val) => val,
            Err(err) => {
                let err_str = format!(
                    "Error turning host header to str: {:#?}\nError: {}",
                    host, err
                );
                error!("{err_str}");
                return Err(err_str);
            }
        };
        let Some(host_str) = domain_str.split(":").nth(0) else {
            let err_str = format!("Error parsing domain_str: {:#?}", domain_str);
            error!("{err_str}");
            return Err(err_str);
        };
        let mut domain = host_str.to_string();
        if tls_port != 443 {
            domain += ":";
            domain += &tls_port.to_string();
        }
        let final_str = format!("https://{}{}", domain, uri);

        return Ok(final_str);
    }

    // uri format such as "/192.168.1.11:80/xx/xx"
    if &uri[..1] == "/" {
        let Some(full_host_str) = uri.split("/").nth(1) else {
            let err_str = format!("Error parsing uri to full_host_str: {:#?}", uri);
            error!("{err_str}");
            return Err(err_str);
        };
        let Some(host_str) = full_host_str.split(":").nth(0) else {
            let err_str = format!("Error parsing full_host_str: {:#?}", uri);
            error!("{err_str}");
            return Err(err_str);
        };
        let mut domain = host_str.to_string();
        if tls_port != 443 {
            domain += ":";
            domain += &tls_port.to_string();
        }
        let mut final_str = format!("https://{}/", domain);
        for (i, val) in uri.split("/").enumerate() {
            if i >= 3 {
                if val != "" {
                    final_str += val;
                    final_str += "/";
                }
            }
        }

        return Ok(final_str);
    }

    // uri format such as "http://192.168.1.11:8080/xx/xx"
    let Some(full_host_str) = uri.split("/").nth(1) else {
        let err_str = format!("Error parsing uri to full_host_str: {:#?}", uri);
        error!("{err_str}");
        return Err(err_str);
    };
    let Some(host_str) = full_host_str.split(":").nth(0) else {
        let err_str = format!("Error parsing full_host_str: {:#?}", uri);
        error!("{err_str}");
        return Err(err_str);
    };
    let mut domain = host_str.to_string();
    if tls_port != 443 {
        domain += ":";
        domain += &tls_port.to_string();
    }
    let mut final_str = format!("https://{}/", domain);
    for (i, val) in uri.split("/").enumerate() {
        if i >= 2 {
            if val != "" {
                final_str += val;
                final_str += "/";
            }
        }
    }

    Ok(final_str)
}
