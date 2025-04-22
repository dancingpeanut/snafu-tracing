use core::error::Error as StdError;
use std::fmt::Display;
use crate::my_error::error::Error;
use crate::my_error::location::Location;

/// 支持链式 context 的 trait
pub trait Context<T, E> {
    fn context<M>(self, msg: M) -> crate::my_error::error::Result<T, Error>
    where
        M: Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn context<M>(self, msg: M) -> crate::my_error::error::Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let w = Error::Wrap {
                error: Box::new(e),
                location: Location::default(),
                chain: None,
            };
            w
        })
    }
}

impl<T> Context<T, Error> for std::result::Result<T, Error> {
    #[track_caller]
    fn context<M>(self, msg: M) -> crate::my_error::error::Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let m = Error::Message {
                msg: msg.to_string(),
                location: Location::default(),
                chain: None,
            };
            m.with_chain(e)
        })
    }
}
