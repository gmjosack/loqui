use std::default::Default;
use crate::protocol::{Request, Response, Push};

pub trait Handler: Default + Send + Sync + 'static {
    fn handle_request(&self, req: Request) -> Response;
    fn handle_push(&self, push: Push);
}
