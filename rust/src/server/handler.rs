use std::default::Default;

use futures::Future;

use crate::protocol::{Request, Response, Push};

pub trait Handler: Default + Send + Sync + 'static {
    fn handle_request(&self, req: Request) -> Box<dyn Future<Item=Vec<u8>, Error=()>>;
    fn handle_push(&self, push: Push);
}
