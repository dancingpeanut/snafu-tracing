
use core::error::Error as StdError;
use crate::location::Location;

#[derive(Debug)]
pub enum Error {
    Any { error: String, location: Location },
    Wrap { error: Box<dyn StdError + Send + Sync + 'static>, location: Location }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn from(error: E) -> Self {
        Error::Wrap {
            error: Box::new(error),
            location: Default::default(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;