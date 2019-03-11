extern crate loqui;

use futures::{future, Future};

use loqui::server::{Server, Handler, Config};
use loqui::protocol::{Request, Response, Push};


#[derive(Default)]
struct EchoServer {}


impl Handler for EchoServer {
    fn handle_request(&self, req: Request) -> Box<dyn Future<Item=Vec<u8>, Error=()>> {

        println!("Saw {:?}", req.payload);
        Box::new(future::ok(b"hello, world!".to_vec()))
    }

    fn handle_push(&self, push: Push) {}
}


fn main() {
    env_logger::init();

    let config = Config {
        supported_encodings: vec!["json".into()],
        ..Config::default()
    };
    let mut server: Server<EchoServer> = Server::new(config);

    server.listen_and_run();
}