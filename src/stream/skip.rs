use {Poll, Async};
use stream::Stream;

/// A stream combinator which skips a number of elements before continuing.
///
/// This structure is produced by the `Stream::skip` method.
pub struct Skip<S> {
    stream: S,
    remaining: u64,
}

pub fn new<S>(s: S, amt: u64) -> Skip<S>
    where S: Stream,
{
    Skip {
        stream: s,
        remaining: amt,
    }
}

impl<S> Stream for Skip<S>
    where S: Stream,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        while self.remaining > 0 {
            match try_poll!(self.stream.poll()) {
                Some(_) => self.remaining -= 1,
                None => return Ok(Async::Ready(None)),
            }
        }

        self.stream.poll()
    }
}
