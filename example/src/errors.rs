use snafu::Snafu;
use snafu_tracing::{DebugTrace, trace_error, wrap_result_ext, drive_anyerr};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[trace_error]
#[wrap_result_ext]
#[drive_anyerr]
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

// quick_tracing!(anyerr, crate::errors::error::Any);
pub use anyerr;
