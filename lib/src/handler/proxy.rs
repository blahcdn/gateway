use hyper::{Client,Request,Response,Body};
use hyper_rustls;
use crate::error;

pub async fn proxy(
  mut req: Request<Body>,
  client: Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
) -> Result<Response<Body>, error::Error> {
  let out_addr = "https://jcde.xyz";

  let uri_string = format!(
    "{}{}",
    out_addr,
    req
      .uri()
      .path_and_query()
      .map(|x| x.as_str())
      .unwrap_or("/")
  )
  .to_owned();
  *req.version_mut() = hyper::Version::HTTP_11;
  *req.uri_mut() = uri_string.parse()?;
  let forward_res = client.request(req).await.unwrap_or_else(move |_req| {
    error::send_error_res(
      "502 Bad Gateway: Origin Server Down".to_string(),
      http::StatusCode::BAD_GATEWAY,
    )
    .unwrap()
  });
  Ok(forward_res)
}

pub async fn handle(
  req: Request<Body>,
  ip: std::net::SocketAddr,
  client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
  sni_hostname: String,
) -> Result<Response<Body>, http::Error> {
  if None == req.headers().get("host") && None == req.uri().authority() {
    return error::send_error_res("Bad Request".to_string(), http::StatusCode::BAD_REQUEST);
  }
  let (mut parts, body) = req.into_parts();
  if parts.uri.authority().is_some() {
    parts.headers.insert(
      "Host",
      http::HeaderValue::from_str(&format!("{}", parts.uri.authority().unwrap())).unwrap(),
    );
  }

  parts.headers.insert(
    "x-forwarded-for",
    http::HeaderValue::from_str(&format!("{}", ip)).unwrap(),
  );
  Ok(
    proxy(Request::from_parts(parts, body), client.to_owned())
      .await
      .unwrap(),
  )
}
