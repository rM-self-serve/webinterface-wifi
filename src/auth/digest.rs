use super::validate_req::nonce_hash;
use http::HeaderValue;
use log::debug;
use sha256::digest;
use std::{collections::HashMap, net::IpAddr};

pub struct AuthDigest {
    nonce: String,
    uri: String,
    response: String,
    qop: String,
    nc: String,
    cnonce: String,
    opaque: String,
    method: String,
}

impl AuthDigest {
    pub fn is_valid(self, user_hash: String, client_ip: IpAddr, session_key: &str) -> bool {
        let request_hash = digest(format!("{}:{}", self.method, self.uri));

        let total_hash = digest(format!(
            "{}:{}:{}:{}:{}:{}",
            user_hash, self.nonce, self.nc, self.cnonce, self.qop, request_hash
        ));

        if total_hash != self.response {
            debug!("Password does not match");
            return false;
        }

        if self.nonce != nonce_hash(client_ip, session_key, &self.opaque) {
            debug!("Invalid nonce for connection");
            return false;
        }

        true
    }
}

impl AuthDigest {
    pub fn parse_header(val: &HeaderValue, method: String) -> Option<AuthDigest> {
        let Ok(to_parse) = val.to_str() else {
            return None;
        };
        let Some(keyval) = parse_raw(to_parse) else {
            debug!("Could not parse authorization header");
            return None;
        };
        let Some(nonce) = keyval.get("nonce") else {
            debug!("Could not find nonce in authorization header");
            return None;
        };
        let Some(uri) = keyval.get("uri") else {
            debug!("Could not find uri in authorization header");
            return None;
        };
        let Some(response) = keyval.get("response") else {
            debug!("Could not find response in authorization header");
            return None;
        };
        let Some(qop) = keyval.get("qop") else {
            debug!("Could not find qop in authorization header");
            return None;
        };
        let Some(nc) = keyval.get("nc") else {
            debug!("Could not find nc in authorization header");
            return None;
        };
        let Some(cnonce) = keyval.get("cnonce") else {
            debug!("Could not find cnonce in authorization header");
            return None;
        };
        let Some(opaque) = keyval.get("opaque") else {
            debug!("Could not find opaque in authorization header");
            return None;
        };

        Some(AuthDigest {
            nonce: nonce.to_owned(),
            uri: uri.to_owned(),
            response: response.to_owned(),
            qop: qop.to_owned(),
            nc: nc.to_owned(),
            cnonce: cnonce.to_owned(),
            opaque: opaque.to_owned(),
            method: method,
        })
    }
}

// username can only contain letters + numbers
fn parse_raw(to_parse: &str) -> Option<HashMap<String, String>> {
    let mut keyval: HashMap<String, String> = HashMap::default();
    let header = to_parse.replace("Digest", "").replace(" ", "");
    for h in header.split(',') {
        let mut key: Option<&str> = None;
        let mut val: Option<&str> = None;
        let h_chars = h.chars();
        for (i, c) in h_chars.enumerate() {
            if c == '=' {
                key = Some(&h[..i]);
                let hold_val = &h[i + 1..];
                if &hold_val[..1] == r#"""# {
                    val = Some(&hold_val[1..hold_val.len() - 1]);
                } else {
                    val = Some(hold_val);
                }
                break;
            }
        }
        let Some(key_str) = key else {
            println!("Could not parse key: {}", h);
            return None;
        };
        let Some(val_str) = val else {
            println!("Could not val: {}", h);
            return None;
        };
        keyval.insert(key_str.to_string(), val_str.to_string());
    }
    Some(keyval)
}
