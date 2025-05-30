use std::fs::File;
use snafu::ResultExt;
use crate::errors::anyerr;
use crate::m2::hello_m2;
use crate::errors::{error, Result};

pub fn hello_err1() -> Result<()> {
    println!("hello_err1!");
    Err(error::Code { id: 1u16 }.build())
}

pub fn hello_err2() -> Result<()> {
    let _e = anyerr!("Any error test! {}", 123);
    Err(anyerr!("Any error test!"))
}

pub fn hello_err() -> Result<()> {
    println!("hello_err!");
    hello_m2()
}

fn anyhow_fn() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("Anyhow error!"))
}

pub fn hello_anyhow() -> Result<()> {
    anyhow_fn().context(error::Anyhow)
}

pub fn hello_file() -> Result<()> {
    println!("Hello, m1 file!");
    // let _ = File::open("test.txt").context(error::IO)?;
    let _ = File::open("test.txt")
        .boxed()
        .context(error::Wrap)?;

    Ok(())
}
