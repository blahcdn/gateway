use crate::listener::{Connection, Listener};
use hyper::service::{make_service_fn, service_fn};
use hyper::{client, Body, Request};
use crate::error;
use crate::handler::proxy;
use crate::listener::Incoming;

pub async fn http_server<L>(listener: L) -> Result<(), hyper::Error>
where
  L: Listener + Send,
  <L as Listener>::Connection: Send + Unpin + 'static,
{

  let https = {
    // Build an HTTP connector which supports HTTPS too.
    let mut http = client::HttpConnector::new();
    http.enforce_http(true);
    let tls = rustls::ClientConfig::new();

    // Join the above part into an HTTPS connector.
    hyper_rustls::HttpsConnector::from((http, tls));
    // Default HTTPS connector.
    hyper_rustls::HttpsConnector::with_native_roots()
  };
  let client: client::Client<_, hyper::Body> = hyper::client::Client::builder().build(https);
  let service = make_service_fn(move |s: &L::Connection| {
    let client = client.clone();
    let ip = s.remote_addr();

    let sni_hostname = s.sni_hostname().map(|name| name.to_string()).unwrap();

    async move {
      Ok::<_, error::GenericError>(service_fn(move |req: Request<Body>| {
        let sni_hostname = sni_hostname.clone();
        proxy::handle(req, ip, client.to_owned(), sni_hostname)
       
      }))
    }
  });
  let server = hyper::Server::builder(Incoming::new(listener))
    .http1_preserve_header_case(false)
    .serve(service);
  server.await
}