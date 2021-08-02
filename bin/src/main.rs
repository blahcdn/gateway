use blahcdn_proxy_lib::{error, server, tls};
use rustls;
use std::net::TcpListener;

fn main() {
  // Serve an echo service over HTTPS, with proper error handling.
  if let Err(e) = run_server() {
    println!("FAILED: {}", e);
  }
}
#[tokio::main]
async fn run_server() -> Result<(), error::Error> {
  let l = TcpListener::bind("127.0.0.1:8080")?;
  let l = tokio::net::TcpListener::from_std(l).unwrap();

  let mut resolver = rustls::ResolvesServerCertUsingSNI::new();
  tls::add_certificate_to_resolver("localhost", &mut resolver);

  let listener = tls::bind_tls(resolver, l).await?;
  let server = server::http_server(listener);
  Ok(server.await?)
}
