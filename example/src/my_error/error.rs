use core::error::Error as StdError;
use core::fmt::{Display, Debug};
use snafu_tracing_macro::{enrich_error, enrich_with_chain};
use crate::my_error::location::Location;

#[enrich_error]
#[enrich_with_chain]
#[derive(Debug)]
pub enum Error {
    Code { error: u16 },
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn from(e: E) -> Self {
        Error::Wrap {
            error: Box::new(e),
            location: Location::default(),
            chain: None,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
