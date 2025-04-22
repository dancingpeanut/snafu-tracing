use core::error::Error as StdError;
use core::fmt::Debug;
use snafu_tracing_macro::{derive_wrap, enrich_error, enrich_with_chain};
use crate::my_error::location::Location;

#[enrich_error]
#[enrich_with_chain]
#[derive_wrap]
#[derive(Debug)]
pub enum Error {
    Code { error: u16 },
    IO { error: std::io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
