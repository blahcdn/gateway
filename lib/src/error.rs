use hyper::{Body, Response};

use log::error;
pub type Error = Box<dyn std::error::Error + Send + Sync>;


pub fn send_error_res(msg: String, code: http::StatusCode) -> Result<Response<Body>, http::Error> {
  error!("code {}", msg);
  Response::builder().status(code).body(Body::from(code.to_string() + &msg))
}
