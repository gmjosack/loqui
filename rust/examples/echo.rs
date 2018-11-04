extern crate loqui;
extern crate env_logger;

use loqui::server::{Server, Handler, Config};
use loqui::protocol::{Request, Response, Push};


#[derive(Default)]
struct EchoServer {}


impl Handler for EchoServer {
    fn handle_request(&self, req: Request) -> Response {
        Response{
           flags: 0,
           sequence_id: 0,
           payload: b"hello, world!".to_vec(),
        }
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
