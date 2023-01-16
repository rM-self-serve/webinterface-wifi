use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use std::net::IpAddr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::{convert::Infallible, net::SocketAddr};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

use crate::net_interface;

const WEB_INTERFACE: &str = "http://10.11.99.1:80";

struct ServerState {
    handle_wlan0: Option<JoinHandle<()>>,
    current_ip: Option<IpAddr>,
}
impl ServerState {
    fn init() -> Arc<Mutex<ServerState>> {
        let server_state = ServerState {
            handle_wlan0: None,
            current_ip: None,
        };
        Arc::new(Mutex::new(server_state))
    }
}

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    Ok(Response::new(Body::from(body_str)))
}

async fn handle(client_ip: IpAddr, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.uri().path().starts_with("/") {
        match hyper_reverse_proxy::call(client_ip, WEB_INTERFACE, req).await {
            Ok(response) => Ok(response),
            Err(_error) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap()),
        }
    } else {
        debug_request(req)
    }
}

async fn start_server(addr: &str, expose_port: &str) {
    let bind_addr = format!("{}:{}", addr, expose_port);
    let addr: SocketAddr = bind_addr.parse().expect("Could not parse ip:port.");
    let make_svc = make_service_fn(|conn: &AddrStream| {
        let remote_addr = conn.remote_addr().ip();
        async move { Ok::<_, Infallible>(service_fn(move |req| handle(remote_addr, req))) }
    });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Running server on {:?}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn abort_thread(mut server_state: MutexGuard<ServerState>) {
    server_state.handle_wlan0.as_mut().unwrap().abort();
    server_state.handle_wlan0 = None;
    server_state.current_ip = None;
}

#[allow(while_true)]
#[tokio::main]
pub async fn run_loop(expose_portstr: &str) {
    let server_state_arc = ServerState::init();
    let expose_port = expose_portstr.to_string();
    while true {
        let mut server_state = server_state_arc.lock().unwrap();
        let curr_net = net_interface::RMipv4s::new();

        if (curr_net.wlan0.is_none() || curr_net.usb0.is_none())
            || (server_state.current_ip.is_some() && server_state.current_ip != curr_net.wlan0)
        {
            if server_state.handle_wlan0.is_some() {
                abort_thread(server_state);
            }
        } else {
            if server_state.handle_wlan0.is_none() {
                let wlan_ipaddr = curr_net.wlan0.unwrap();
                server_state.current_ip = Some(wlan_ipaddr.clone());
                let wlanip = wlan_ipaddr.to_string();
                let expose_port_clone = expose_port.clone();
                let wlanthread =
                    tokio::spawn(async move { start_server(&wlanip, &expose_port_clone).await });
                server_state.handle_wlan0 = Some(wlanthread)
            } else {
                if server_state.handle_wlan0.as_ref().unwrap().is_finished() {
                    abort_thread(server_state);
                }
            }
        }
        sleep(Duration::from_millis(3000)).await;
    }
}
