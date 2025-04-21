use std::error::Error;
use std::fmt;

pub use snafu_tracing_macro::{DebugTrace, trace_error};

pub trait DebugTrace: Error {
    fn debug_trace(&self, f: &mut fmt::Formatter) -> Result<u32, fmt::Error>;
}

impl Error for Box<dyn DebugTrace + Send + Sync + 'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(Box::as_ref(self))
    }
}
