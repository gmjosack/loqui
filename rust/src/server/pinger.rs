//! FrameWriter is a task that will continuously until it's stream has been exhausted.

use tokio::prelude::*;

use crate::protocol::codec::LoquiFrame;


pub struct Pinger<T>
where T: Sink<SinkItem=LoquiFrame>,
{
    /// A Sink that accepts LoquiFrames.
    sink: T,
}


impl<T> Pinger<T>
where T: Sink<SinkItem=LoquiFrame>,
{
    pub fn new(sink: T) -> Self {
        Self {
            sink: sink,
        }
    }
}

impl<T> Future for Pinger<T>
where T: Sink<SinkItem=LoquiFrame>,
{
    // Pinger is meant to be run as a continuous task via tokio::spawn
    // so doesn't return anything.
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        Err(())
    }
}
