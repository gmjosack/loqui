//! FrameWriter is a task that will continuously until it's stream has been exhausted.

use std::fmt::Debug;

use futures::stream::Fuse;
use tokio::prelude::*;
use log::{warn, debug};

use crate::protocol::codec::LoquiFrame;


pub struct FrameWriter<T, N, E, F>
where T: Stream<Item=LoquiFrame, Error=E>,
      N: Sink<SinkItem=LoquiFrame, SinkError=F>,
      E: Debug,
      F: Debug,
{
    /// A Stream of LoquiFrame's that the FrameWriter will attempt to consume.
    stream: Fuse<T>,

    /// A Sink of LoquiFrame's that the FrameWriter will attempt to produce to.
    sink: N,

    /// A buffer to hold the frame if we had a Frame on the Stream
    /// but were not able to push into the Sink.
    buffer: Option<LoquiFrame>,
}


impl<T, N, E, F> FrameWriter<T, N, E, F>
where T: Stream<Item=LoquiFrame, Error=E>,
      N: Sink<SinkItem=LoquiFrame, SinkError=F>,
      E: Debug,
      F: Debug,
{
    pub fn new(stream: T, sink: N) -> Self {
        Self {
            stream: stream.fuse(),
            sink: sink,
            buffer: None,
        }
    }

    /// Tries to push a LoquiFrame into the Sink. If not available shove the frame
    /// into the buffer. This function should only be called when the buffer is empty
    /// or you'll lose frames.
    fn try_start_send(&mut self, frame: T::Item) -> Result<Async<()>, N::SinkError> {
        debug!("sending frame: {:?}", frame);
        if let AsyncSink::NotReady(frame) = self.sink.start_send(frame)? {
            self.buffer = Some(frame);
            return Ok(Async::NotReady)
        }
        Ok(Async::Ready(()))
    }
}

impl<T, N, E, F> Future for FrameWriter<T, N, E, F>
where T: Stream<Item=LoquiFrame, Error=E>,
      N: Sink<SinkItem=LoquiFrame, SinkError=F>,
      E: Debug,
      F: Debug,
{
    // FrameWriter is meant to be run as a continuous task via tokio::spawn
    // so doesn't return anything.
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        // If there was anything in the buffer try to shove it into the sink
        if let Some(frame) = self.buffer.take() {
            match self.try_start_send(frame) {
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(err) => {
                    warn!("Unexpected error, crashing FrameWriter: {:?}", err);
                    return Err(());
                }
                _ => (),
            }
        }

        loop {
            match self.stream.poll() {
                // We got a frame from the Stream, try to send it.
                Ok(Async::Ready(Some(frame))) => match self.try_start_send(frame) {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(err) => {
                        warn!("Unexpected error, crashing FrameWriter {:?}", err);
                        return Err(());
                    }
                    _ => (),
                },
                // The Stream is done producing frames, time to shutdown.
                Ok(Async::Ready(None)) => match self.sink.close() {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(_)) => return Ok(Async::Ready(())),
                    Err(err) => {
                        warn!("Unexpected error, crashing FrameWriter {:?}", err);
                        return Err(());
                    }
                },
                // The Stream doesn't have anything on it
                Ok(Async::NotReady) => match self.sink.poll_complete() {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(_)) => return Ok(Async::NotReady),
                    Err(err) => {
                        warn!("Unexpected error, crashing FrameWriter {:?}", err);
                        return Err(());
                    }
                },
                Err(err) => {
                    warn!("Crashing client FrameWriter; Failed to poll: {:?}", err);
                    return Err(());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sink_never_ready() {
    }

    #[test]
    fn test_stream_never_ready() {
    }
}
