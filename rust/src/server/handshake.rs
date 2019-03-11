use std::sync::Arc;
use std::net::SocketAddr;

use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio_codec::Framed;
use log::{warn, debug};

// Internal modules
use crate::protocol::frames;
use crate::protocol::codec::{LoquiCodec, LoquiFrame};
use crate::server::config::Config;


pub struct HandShake {
    socket: Option<Framed<TcpStream, LoquiCodec>>,
    config: Arc<Config>,
    peer_addr: SocketAddr,
}

impl HandShake {
    pub fn new(socket: Framed<TcpStream, LoquiCodec>, config: Arc<Config>) -> HandShake {
        let peer_addr = socket.get_ref().peer_addr().expect("Only established connections should be passed.");
        HandShake {
            socket: Some(socket),
            config: config,
            peer_addr: peer_addr,
        }
    }
}


impl Future for HandShake {
    type Item = (frames::HelloAck, Framed<TcpStream, LoquiCodec>);
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let mut socket = self.socket.take().expect(
            "poll called after Async::Ready or Err..."
        );

        let frame = match socket.poll() {
            Err(err) => {
                warn!("handshake error: {:?}", err);
                return Err(());
            },
            Ok(Async::NotReady) => {
                self.socket = Some(socket);
                return Ok(Async::NotReady);
            },
            Ok(Async::Ready(frame)) => frame,
        };

        match frame {
            None => {
                warn!(
                    "client went away without handshake: {:?}",
                    self.peer_addr,
                );
                return Err(());
            },
            Some(LoquiFrame::Hello(hello)) => {
                debug!(
                    "handshake from client ({:?}) started: {:?}",
                    self.peer_addr,
                    hello,
                );
                let ack = frames::HelloAck::from_hello(
                    &hello,
                    self.config.ping_interval.subsec_millis(),
                    self.config.supported_encodings.as_ref(),
                    self.config.supported_compressions.as_ref(),
                );

                return match ack {
                    None => {
                        warn!("client failed handshake: {:?}", self.peer_addr);
                        Err(())
                    },
                    Some(ack) => Ok(Async::Ready((ack, socket))),
                }
            },
            // No other frames are expected before handshake. This is a poor behaving
            // client so don't bother trying to give an error. Just die.
            _ => {
                warn!(
                    "client ({:?}) sent unexpected frame before handshake: {:?}",
                    self.peer_addr,
                    frame,
                );
                return Err(());
            },
        };
    }
}
