use {Async, Poll, IntoFuture, Future};
use stream::Stream;

/// A stream combinator which skips elements of a stream while a predicate
/// holds.
///
/// This structure is produced by the `Stream::skip_while` method.
pub struct SkipWhile<S, P, R> where S: Stream, R: IntoFuture {
    stream: S,
    pred: P,
    pending: Option<(R::Future, S::Item)>,
    done_skipping: bool,
}

pub fn new<S, P, R>(s: S, p: P) -> SkipWhile<S, P, R>
    where S: Stream,
          P: FnMut(&S::Item) -> R,
          R: IntoFuture<Item=bool, Error=S::Error>,
{
    SkipWhile {
        stream: s,
        pred: p,
        pending: None,
        done_skipping: false,
    }
}

impl<S, P, R> Stream for SkipWhile<S, P, R>
    where S: Stream,
          P: FnMut(&S::Item) -> R,
          R: IntoFuture<Item=bool, Error=S::Error>,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        if self.done_skipping {
            return self.stream.poll();
        }

        loop {
            if self.pending.is_none() {
                let item = match try_poll!(self.stream.poll()) {
                    Some(e) => e,
                    None => return Ok(Async::Ready(None)),
                };
                self.pending = Some(((self.pred)(&item).into_future(), item));
            }

            assert!(self.pending.is_some());
            match self.pending.as_mut().unwrap().0.poll() {
                Ok(Async::Ready(true)) => self.pending = None,
                Ok(Async::Ready(false)) => {
                    let (_, item) = self.pending.take().unwrap();
                    self.done_skipping = true;
                    return Ok(Async::Ready(Some(item)))
                }
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => {
                    self.pending = None;
                    return Err(e)
                }
            }
        }
    }
}
