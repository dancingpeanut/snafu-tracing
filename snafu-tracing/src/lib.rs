//! Snafu tracing
//!

//! # Example
//!
//! ```rust
//! use snafu::Snafu;
//! use snafu_tracing::{DebugTrace, trace_error, quick_tracing};
//! 
//! pub type Result<T, E = Error> = std::result::Result<T, E>;
//! 
//! #[trace_error]
//! #[derive(Snafu, DebugTrace)]
//! #[snafu(module, context(suffix(false)), visibility(pub))]
//! pub enum Error {
//!     #[snafu(display("{_error}"))]
//!     Any { _error: String },
//!     #[snafu(display("{error}"))]
//!     Wrap {
//!         error: Box<dyn std::error::Error + Send + Sync>,
//!     },
//! }
//! 
//! quick_tracing!(anyerr, crate::error::Any);
//! 
//! pub fn hello_err() -> Result<()> {
//!     let _e = anyerr!("Any error test! {}", 123);
//!     Err(anyerr!("Any error test!"))
//! }
//! 
//! fn main() {
//!     let e = hello_err();
//!     println!("{:?}", e);
//! }
//! ```

use std::error::Error;
use std::fmt;

pub use snafu_tracing_macro::{DebugTrace, quick_tracing, trace_error};

pub trait DebugTrace: Error {
    fn debug_trace(&self, f: &mut fmt::Formatter) -> Result<u32, fmt::Error>;
}

impl Error for Box<dyn DebugTrace + Send + Sync + 'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Error::source(Box::as_ref(self))
    }
}
