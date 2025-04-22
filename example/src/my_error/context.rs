use core::error::Error as StdError;
use std::result::Result as StdResult;
use std::fmt::Display;
use crate::my_error::error::Error;
use crate::my_error::location::Location;

/// 支持链式 context 的 trait
pub trait Context<T, E> {
    fn context<M>(self, msg: M) -> StdResult<T, Error>
    where
        M: Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for StdResult<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn context<M>(self, msg: M) -> StdResult<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let a = Error::Any {
                error: Box::new(e),
                location: Location::default(),
                chain: None,
            };
            let m = Error::Context {
                msg: msg.to_string(),
                location: Location::default(),
                chain: None,
            };
            m.with_chain(a)
        })
    }
}

impl<T> Context<T, Error> for StdResult<T, Error> {
    #[track_caller]
    fn context<M>(self, msg: M) -> crate::my_error::error::Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| {
            let m = Error::Context {
                msg: msg.to_string(),
                location: Location::default(),
                chain: None,
            };
            m.with_chain(e)
        })
    }
}
