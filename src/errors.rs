//! Error handling

use std;
use std::error::Error as StdError;
use std::fmt::Debug;

use futures::Future;

error_chain!{
    errors {
        /// Unable to open the cache. This may be because it doesn't exist.
        MissingCache(err: std::io::Error) {
            cause(err)
            description(err.description())
        }
    }
}

/// A trait that allows us to log a `Result` using an inline function.
pub trait ResultLoggingExt<M: Debug + Send + Sync>: Sized {
    /// Logs the current state of `Self`. In a 'good' state, it will call the logging closure
    /// provided. In a 'bad' state, it will just log the appropriate error at 'error' level.
    fn log<F>(self, message_fn: F) -> Self
    where
        F: Fn(&M) + Send + Sync + 'static;
}

impl<M: Debug + Send + Sync> ResultLoggingExt<M> for Result<M> {
    fn log<F>(self, message_fn: F) -> Self
    where
        F: Fn(&M) + Send + Sync + 'static,
    {
        match self {
            Ok(ref contents) => message_fn(contents),
            Err(ref e) => error!("{:?}", e),
        }

        self
    }
}

/// A trait that allows us to log a `Future` using an inline function.
pub trait FutureLoggingExt<M: Debug + Send + Sync>: Sized + Future {
    /// Logs the current state of `Self`. In a 'good' state, it will call the logging closure
    /// provided. In a 'bad' state, it will just log the appropriate error at 'error' level.
    fn log<F>(self, message_fn: F) -> Box<Future<Item = Self::Item, Error = Self::Error> + Send>
    where
        F: Fn(&M) + Send + Sync + 'static;
}

impl<M, Fut> FutureLoggingExt<M> for Fut
where
    M: Debug + Send + Sync + 'static,
    Fut: Future<Item = M, Error = Error> + Send + 'static,
{
    fn log<F>(self, message_fn: F) -> Box<Future<Item = Self::Item, Error = Self::Error> + Send>
    where
        F: Fn(&M) + Send + Sync + 'static,
    {
        Box::new(self.then(move |a| {
            match a {
                Ok(ref b) => message_fn(b),
                Err(ref e) => error!("{:?}", e),
            }

            a
        }))
    }
}
