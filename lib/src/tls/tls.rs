use rustls;
use std::{sync::Arc,fs::File,io::{self,BufReader}};
use tokio::net::{TcpListener};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use tokio_rustls::{TlsAcceptor};
use super::tls_listener::{TlsListener,TlsListenerState};

use rustls::sign::{RSASigningKey, SigningKey};
use rustls::ResolvesServerCertUsingSNI;


pub async fn bind_tls(
  resolver: rustls::ResolvesServerCertUsingSNI,
  listener: TcpListener
) -> io::Result<TlsListener> {

  let tls_cfg = {
    // Load public certificate.
    // Do not use client certificate authentication.
    let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    // Select a certificate to use.
    cfg.cert_resolver = Arc::new(resolver);

    cfg.ticketer = rustls::Ticketer::new();
    let cache = rustls::ServerSessionMemoryCache::new(1024);
    cfg.set_persistence(cache);
    // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
    cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
    cfg
  };

  let acceptor = TlsAcceptor::from(Arc::new(tls_cfg));
  let state = TlsListenerState::Listening;

  Ok(TlsListener {
    listener,
    acceptor,
    state,
  })
}



pub fn add_certificate_to_resolver(hostname: &str, resolver: &mut ResolvesServerCertUsingSNI) {
  //let resolve = |filename| format!("./{filename}", filename = &filename);
  //    config_dir = env::var("XDG_CONFIG_HOME").unwrap().to_string(),

  let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
  let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

  let cert_chain = certs(cert_file).unwrap();
  let mut keys = pkcs8_private_keys(key_file).unwrap();
  let signing_key = RSASigningKey::new(&keys.remove(0)).unwrap();
  let signing_key_boxed: Arc<Box<dyn SigningKey>> = Arc::new(Box::new(signing_key));

  resolver
    .add(
      hostname,
      rustls::sign::CertifiedKey::new(cert_chain, signing_key_boxed),
    )
    .expect("Invalid certificate");
}