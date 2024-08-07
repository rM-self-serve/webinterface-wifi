use crate::{auth::validate_req, constants::FAVICON_PATH};
use http::Version;
use hyper::{Body, Request, Response, StatusCode};
use log::{debug, error};
use std::{
    convert::Infallible,
    net::{IpAddr, SocketAddr},
};

pub async fn handle(
    client_ip: IpAddr,
    webint_addr: SocketAddr,
    pass_hash: Option<String>,
    session_key: String,
    mut req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    debug!("Request: {:#?}", req);

    if let Some(pass_hash) = pass_hash {
        if let Some(error_response) = validate_req::run(&req, pass_hash, client_ip, session_key) {
            return Ok(error_response);
        }
    }

    if req.uri().path().starts_with("/favicon.ico") {
        let Ok(data) = std::fs::read(FAVICON_PATH) else {
            debug!("Cound not find favicon.ico");
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        };
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "image/jpeg")
            .body(Body::from(data))
            .unwrap());
    }

    if req.uri().path().starts_with("/") {
        let webint_addr = format!("http://{}", webint_addr);
        *req.version_mut() = Version::default();
        match hyper_reverse_proxy::call(client_ip, &webint_addr, req).await {
            Ok(response) => {
                debug!("Response: {:#?}", response);
                return Ok(response);
            }
            Err(err) => {
                error!("Error: {:#?}", err);
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error: {:#?}", err)))
                    .unwrap());
            }
        }
    }

    debug_request(req)
}

fn debug_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body_str = format!("{:?}", req);
    debug!("Debug req: {body_str}");
    Ok(Response::new(Body::from(body_str)))
}
