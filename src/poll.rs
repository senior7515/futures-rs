#[macro_export]
macro_rules! try_poll {
    ($e:expr) => (match $e {
        Ok($crate::Async::Ready(t)) => t,
        Ok($crate::Async::NotReady) => return Ok($crate::Async::NotReady),
        Err(e) => return Err(From::from(e)),
    })
}

/// Return type of the `Future::poll` method, indicates whether a future's value
/// is ready or not.
///
/// * `Ok(Async::Ready(t))` means that a future has successfully resolved
/// * `Ok(Async::NotReady)` means that a future is not ready to complete yet
/// * `Err(e)` means that a future has completed with the given failure
pub type Poll<T, E> = Result<Async<T>, E>;

/// Return type of future, indicating whether a value is ready or not.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Async<T> {
    /// Represents that a value is immediately ready.
    Ready(T),

    /// Represents that a value is not ready yet, but may be so later.
    NotReady,
}

impl<T> Async<T> {
    /// Change the success type of this `Async` value with the closure provided
    pub fn map<F, U>(self, f: F) -> Async<U>
        where F: FnOnce(T) -> U
    {
        match self {
            Async::NotReady => Async::NotReady,
            Async::Ready(t) => Async::Ready(f(t)),
        }
    }

    /// Returns whether this is `Async::NotReady`
    pub fn is_not_ready(&self) -> bool {
        match *self {
            Async::NotReady => true,
            Async::Ready(_) => false,
        }
    }

    /// Returns whether this is `Async::Ready`
    pub fn is_ready(&self) -> bool {
        !self.is_not_ready()
    }
}

impl<T> From<T> for Async<T> {
    fn from(t: T) -> Async<T> {
        Async::Ready(t)
    }
}
