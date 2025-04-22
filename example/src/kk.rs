use core::error::Error as StdError;
use core::fmt::{Display, Debug};
use crate::location::Location;

#[derive(Debug)]
pub enum Error {
    Message {
        msg: String,
        source: Option<Box<Self>>,
        location: Location,
    },
    Wrap {
        msg: String,
        source: Box<dyn StdError + Send + Sync + 'static>,
        location: Location,
    },
}

impl Error {
    pub fn location(&self) -> &Location {
        match self {
            Error::Message { location, .. } => location,
            Error::Wrap { location, .. } => location,
        }
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn from(e: E) -> Self {
        Error::Wrap {
            msg: "".to_string(),
            source: Box::new(e),
            location: Location::default(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// 支持链式 context 的 trait
pub trait Context<T, E> {
    fn context<M>(self, msg: M) -> Result<T, Error>
    where
        M: Display + Send + Sync + 'static;
}

impl<T, E> Context<T, E> for std::result::Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    #[track_caller]
    fn context<M>(self, msg: M) -> Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| Error::Wrap {
            msg: msg.to_string(),
            source: Box::new(e),
            location: Location::default(),
        })
    }
}

impl<T> Context<T, Error> for std::result::Result<T, Error> {
    #[track_caller]
    fn context<M>(self, msg: M) -> Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|e| Error::Message {
            msg: msg.to_string(),
            source: Some(Box::new(e)),
            location: Location::default(),
        })
    }
}
