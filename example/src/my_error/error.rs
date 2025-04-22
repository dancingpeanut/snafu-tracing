use core::fmt::Debug;
use snafu_tracing_macro::{enrich_error, enrich_with_chain};
use crate::my_error::location::Location;

#[enrich_error]
#[enrich_with_chain]
#[derive(Debug)]
pub enum Error {
    Code { error: u16 },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
