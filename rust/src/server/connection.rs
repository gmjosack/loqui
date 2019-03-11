use std::sync::Arc;
use std::net::SocketAddr;

use futures::stream::SplitStream;
use futures::sync::mpsc::{self, Sender};
use tokio;
use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio_codec::Framed;
use tokio_timer::Interval;
use log::{info, warn, debug};

// Internal modules
use crate::protocol::codec::{LoquiCodec, LoquiFrame};
use crate::protocol::frames;
use crate::server::config::Config;
use crate::server::frame_writer::FrameWriter;
use crate::server::handler::Handler;
use crate::server::pinger::Pinger;


pub struct Connection<T: Handler> {
    socket: SplitStream<Framed<TcpStream, LoquiCodec>>,
    peer_addr: SocketAddr,
    frame_writer: Sender<LoquiFrame>,
    config: Arc<Config>,
    handler: T,
}

impl<T: Handler> Connection<T> {
    pub fn new(socket: Framed<TcpStream, LoquiCodec>, config: Arc<Config>, ack: frames::HelloAck)
        -> Result<Connection<T>, ::failure::Error>
    {

        let peer_addr = socket.get_ref().peer_addr()?;
        info!("connection from: {:?}", peer_addr);

        let (framed_sink, framed_stream) = socket.split();
        let (mut tx, rx) = mpsc::channel(50);

        tokio::spawn(FrameWriter::new(rx, framed_sink));
        tx.try_send(LoquiFrame::HelloAck(ack))?;

        let _pinger_tx = tx.clone();
        tokio::spawn(Interval::new_interval(config.ping_interval)
            .map(|_instant| {
                LoquiFrame::Ping(frames::Ping{
                    flags: 0,
                    sequence_id: 0,
                })
            })
            .map_err(|err| {
                ::failure::Error::from(err)
            })
            .forward(tx.clone())
            .map(|_| ())
            .map_err(|_| ())
        );

        Ok(Connection {
            socket: framed_stream,
            peer_addr: peer_addr,
            frame_writer: tx,
            config: config,
            handler: T::default(),
        })
    }
}

impl<T: Handler> Future for Connection<T> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        loop {
            let frame = match self.socket.poll() {
                Err(err) => {
                    warn!("connection error ({:?}): {:?}", self.peer_addr, err);
                    return Err(());
                },
                Ok(Async::NotReady) => {
                    return Ok(Async::NotReady);
                },
                Ok(Async::Ready(frame)) => frame,
            };

            if frame.is_none() {
                info!("connection went away: {:?}", self.peer_addr);
                return Ok(Async::Ready(()));
            }

            debug!("connection ({:?}) received frame: {:?}", self.peer_addr, frame);
        }

    }
}
