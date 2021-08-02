use hyper::{Body, HeaderMap, Request, Response, Server};
use hyper::header::{HeaderValue, CACHE_CONTROL, COOKIE, SERVER, VIA};
use hyper::StatusCode;
use hyper::Version;

pub struct CachedResponse {
  status: StatusCode,
  version: Version,
  headers: HeaderMap<HeaderValue>,
  body: Vec<u8>,
}

