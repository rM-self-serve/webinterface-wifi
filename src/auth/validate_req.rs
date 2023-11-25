use super::{
    digest::AuthDigest,
    response::{not_auth_resp, unreq_auth_header},
};
use crate::constants::AUTH_REALM;
use http::{Request, Response};
use hyper::Body;
use log::debug;
use sha256::digest;
use std::net::IpAddr;

pub fn run(
    req: &Request<Body>,
    user_hash: String,
    client_ip: IpAddr,
    session_key: String,
) -> Option<Response<Body>> {
    match req.headers().get("authorization") {
        Some(val) => {
            let Some(authdig) = AuthDigest::parse_header(val, req.method().to_string()) else {
                let response = unreq_auth_header();
                debug!("Unrecogonized auth header:");
                return Some(response);
            };
            if !authdig.is_valid(user_hash, client_ip, &session_key) {
                let response = register_client(client_ip, session_key);
                debug!("Auth header not valid:");
                return Some(response);
            }
        }
        None => {
            let response = register_client(client_ip, session_key);
            debug!("Auth header missing:");
            return Some(response);
        }
    }
    None
}

fn register_client(client_ip: IpAddr, session_key: String) -> Response<Body> {
    let new_opaque = random_string::generate(16, random_string::charsets::ALPHANUMERIC);
    let new_nonce = nonce_hash(client_ip, &session_key, &new_opaque);

    let auth_header_content = vec![
        &*format!(r#"Digest realm="{}""#, AUTH_REALM),
        r#"qop="auth, auth-int""#,
        "algorithm=SHA-256",
        &*format!(r#"nonce="{}""#, new_nonce),
        &*format!(r#"opaque="{}""#, new_opaque),
    ]
    .join(",");

    not_auth_resp(auth_header_content)
}

pub fn nonce_hash(client_ip: IpAddr, session_key: &str, opaque: &str) -> String {
    digest(format!("{}:{}:{}", client_ip, session_key, opaque))
}
