use std::fs::File;
use snafu::ResultExt;
use crate::errors::{anyerr, Error, WrapResultExt};
use crate::m2::hello_m2;
use crate::errors::{error, Result};

pub fn hello_err1() -> Result<()> {
    let e = error::Code::new(12).build();
    Err(e)
}

pub fn hello_err2() -> Result<()> {
    let _e = anyerr!("Any error test! {}", 123);
    // Err(anyerr!("Any error test!"))
    Ok(())
}

pub fn hello_err() -> Result<()> {
    hello_m2()
}

fn anyhow_fn() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("Anyhow error!"))
}

pub fn hello_anyhow() -> Result<()> {
    anyhow_fn().context(error::Anyhow)
}

pub fn hello_file() -> Result<()> {
    // let _ = File::open("test.txt").context(error::IO)?;
    // let _ = File::open("test.txt")
    //     .boxed()
    //     .context(error::Wrap)?;
    let _ = File::open("test.txt")?;

    Ok(())
}
