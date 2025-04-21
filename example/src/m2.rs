use crate::errors::{error, Result};

pub fn hello_m2() -> Result<()> {
    println!("Hello, m2!");
    error::Simple.fail()
}
