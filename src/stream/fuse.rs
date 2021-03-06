use Poll;
use stream::Stream;

/// A stream which "fuse"s a stream once it's terminated.
///
/// Normally streams can behave unpredictably after they've terminated or
/// returned an error, but `Fuse` is always defined to return `None` from `poll`
/// after terination/errors, and afterwards all calls to `schedule` will be
/// ignored.
pub struct Fuse<S> {
    stream: Option<S>,
}

pub fn new<S: Stream>(s: S) -> Fuse<S> {
    Fuse { stream: Some(s) }
}

impl<S: Stream> Stream for Fuse<S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        let ret = self.stream.as_mut().map(|s| s.poll());
        if let Some(Poll::Ok(None)) = ret {
            self.stream = None;
        }
        ret.unwrap_or(Poll::Ok(None))
    }
}

impl<S> Fuse<S> {
    /// Returns whether the underlying stream has finished or not.
    ///
    /// If this method returns `true`, then all future calls to poll are
    /// guaranteed to return `NotReady`. If this returns `false`, then the
    /// underlying stream is still in use.
    pub fn is_done(&self) -> bool {
        self.stream.is_none()
    }
}
