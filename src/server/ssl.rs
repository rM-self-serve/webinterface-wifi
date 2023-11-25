use hyper::server::conn::AddrIncoming;
use hyper_rustls::TlsAcceptor;
use std::{
    fs,
    io::{BufReader, Error, ErrorKind, Result},
    vec::Vec,
};

pub async fn acceptor(
    pub_path: &str,
    priv_path: &str,
    incoming: AddrIncoming,
) -> Result<TlsAcceptor> {
    let certs = load_certs(pub_path)?;
    let key = load_private_key(priv_path)?;

    let a = TlsAcceptor::builder()
        .with_single_cert(certs, key)
        .map_err(|e| error(format!("{}", e)))?
        .with_all_versions_alpn()
        .with_incoming(incoming);
    Ok(a)
}

pub fn load_certs(filename: &str) -> Result<Vec<rustls::Certificate>> {
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = BufReader::new(certfile);
    let certs = rustls_pemfile::certs(&mut reader)
        .map_err(|_| error("failed to load certificate".into()))?;
    if certs.len() < 1 {
        return Err(error("expected at least one certificate".into()));
    }

    Ok(certs.into_iter().map(rustls::Certificate).collect())
}

pub fn load_private_key(filename: &str) -> Result<rustls::PrivateKey> {
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = BufReader::new(keyfile);
    let keys = rustls_pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;
    if keys.len() != 1 {
        return Err(error("expected a single private key".into()));
    }

    Ok(rustls::PrivateKey(keys[0].clone()))
}

fn error(err: String) -> Error {
    Error::new(ErrorKind::Other, err)
}
