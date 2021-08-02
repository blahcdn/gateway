use std::{net::SocketAddr,
  task::{Context, Poll},
  pin::Pin, io};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{server::TlsStream, Accept, TlsAcceptor};
use crate::listener::{Listener, Connection};
use futures::Future;

pub struct TlsListener {
  pub listener: TcpListener,
 pub  acceptor: TlsAcceptor,
 pub  state: TlsListenerState,
}

pub enum TlsListenerState {
  Listening,
  Accepting(Accept<TcpStream>),
}

impl Listener for TlsListener {
  type Connection = TlsStream<TcpStream>;

  fn local_addr(&self) -> Option<SocketAddr> {
    self.listener.local_addr().ok()
  }

  fn poll_accept(
    mut self: Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<io::Result<Self::Connection>> {
    loop {
      match self.state {
        TlsListenerState::Listening => match self.listener.poll_accept(cx) {
          Poll::Pending => return Poll::Pending,
          Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
          Poll::Ready(Ok((stream, _addr))) => {
            let fut = self.acceptor.accept(stream);
            self.state = TlsListenerState::Accepting(fut);
          }
        },
        TlsListenerState::Accepting(ref mut fut) => match Pin::new(fut).poll(cx) {
          Poll::Pending => return Poll::Pending,
          Poll::Ready(result) => {
            self.state = TlsListenerState::Listening;
            return Poll::Ready(result);
          }
        },
      }
    }
  }
}


impl Connection for TlsStream<TcpStream> {
  fn remote_addr(&self) -> SocketAddr {
    self.get_ref().0.peer_addr().unwrap()
  }
  fn sni_hostname(&self) -> Option<&str> {
    self.get_ref().1.get_sni_hostname()
  }
}
