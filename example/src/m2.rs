use crate::errors::{error, Result};

pub fn hello_m2() -> Result<()> {
    error::Simple.fail()
}
