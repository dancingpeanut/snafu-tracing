use snafu::Snafu;
use snafu_tracing::{DebugTrace, trace_error, quick_tracing};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[trace_error]
#[derive(Snafu, DebugTrace)]
#[snafu(module, context(suffix(false)), visibility(pub))]
pub enum Error {
    #[snafu(display("{_error}"))]
    Any { _error: String },
    #[snafu(display("{error}"))]
    Wrap {
        error: Box<dyn std::error::Error + Send + Sync>,
    },

    #[snafu(display("Error code: {id}"))]
    Code { id: u16 },
    #[snafu(display("IO error"))]
    IO { error: std::io::Error },
    #[snafu(display("Simple error"))]
    Simple,
    #[snafu(display("{error}"))]
    Anyhow { error: anyhow::Error },
}

quick_tracing!(anyerr, crate::errors::error::Any);
pub use anyerr;

pub trait MyResultExt<T>: Sized {
    fn wrap(self) -> std::result::Result<T, Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> MyResultExt<T> for std::result::Result<T, E> {
    #[track_caller]
    fn wrap(self) -> std::result::Result<T, Error> {
        match self {
            Ok(v) => Ok(v),
            Err(error) =>{
                let error = Error::Wrap {
                    error: Box::new(error),
                    _location: Default::default(),
                };
                Err(error)
            },
        }
    }
}
