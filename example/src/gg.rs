use std::fs::File;
use crate::my_error::{Context, Error, Result};
use crate::my_error::error::Wrap;

pub fn g1() -> Result<()> {
    // let _ = File::open("test.txt")?;
    let _ = File::open("test.txt").wrap()?;

    Ok(())
}

pub fn g2() -> Result<()> {
    let resp = reqwest::blocking::get("demo.haizhi.com/")?;

    Ok(())
}

pub fn g3() -> Result<()> {
    g1().context("11")
}

pub fn g4() -> Result<()> {
    let e = Error::Code {
        error: 0,
        location: Default::default(),
        chain: None,
    };
    Err(e)
}

pub fn g5() -> Result<()> {
    g4().context("11")
}
