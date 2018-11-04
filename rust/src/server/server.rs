use std::net::SocketAddr;
use std::sync::Arc;
use std::marker::PhantomData;

use tokio;
use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio_codec::Framed;

// Internal modules
use crate::protocol::codec::LoquiCodec;
use crate::server::config::Config;
use crate::server::handler::Handler;
use crate::server::handshake::HandShake;
use crate::server::connection::Connection;


pub struct Server<T: Handler> {
    config: Arc<Config>,
    // The Connection will actually construct T saw we need to stash
    // it here to apease the compiler.
    _handler: PhantomData<T>,
}


impl<T: Handler> Server<T> {
    pub fn new(config: Config) -> Server<T> {
        Server {
            config: Arc::new(config),
            _handler: PhantomData,
        }
    }

    pub fn listen(&mut self) -> Result<TcpListener, ::failure::Error> {
        let addr: SocketAddr = self.config.address.parse()?;
        Ok(TcpListener::bind(&addr)?)
    }

    pub fn run(&mut self, listener: TcpListener) {
        // Grab the local addr before the listener is consumed.
        let local_addr = listener.local_addr().unwrap();
        let config = self.config.clone();

		let server = listener.incoming().for_each(move |socket| {
            info!("socket accepted: addr={:?}", socket.peer_addr());
            let socket = Framed::new(socket, LoquiCodec::new(config.max_payload_bytes));
            let conn_config = config.clone();
            let connection = HandShake::new(socket, config.clone())
                .and_then(move |(ack, socket)| {
                    match Connection::<T>::new(socket, conn_config.clone(), ack) {
                        Ok(conn) => future::Either::A(conn),
                        Err(err) => {
                            warn!("connection failure: {:?}", err);
                            future::Either::B(future::err(()))
                        },
                    }
                });

            tokio::spawn(connection);
            Ok(())
        })
        .map_err(|err| {
            warn!("accept error: {:?}", err);
        });

		info!("server started on {:?}", local_addr);
		tokio::run(server);
    }

    pub fn listen_and_run(&mut self) {
        let listener = self.listen();
        self.run(listener.unwrap());
    }
}
