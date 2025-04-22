use core::error::Error as StdError;
use core::fmt::{Display, Debug};
use crate::my_error::location::Location;

#[derive(Debug)]
pub enum Error {
    Message {
        msg: String,
        location: Location,
        chain: Option<Box<Self>>
    },
    Wrap {
        error: Box<dyn StdError + Send + Sync + 'static>,
        location: Location,
        chain: Option<Box<Self>>
    },
    
    Code {
        error: u16,
        location: Location,
        chain: Option<Box<Self>>
    },
}

impl Error {
    pub fn with_chain(self, mut chain: Error) -> Self {
        match &mut chain {
            Error::Message { chain: c, .. } => *c = Some(Box::new(self)),
            Error::Wrap { chain: c, .. } => *c = Some(Box::new(self)),
            Error::Code { chain: c, .. } => *c = Some(Box::new(self)),
        }
        chain
    }
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
