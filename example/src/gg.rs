use std::fs::File;
use crate::my_error::error::{Context, Error, Result};

pub fn g1() -> Result<()> {
    let _ = File::open("test.txt")?;

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
    Err(Error::Code {
        error: 0,
        location: Default::default(),
        chain: None,
    })
}

pub fn g5() -> Result<()> {
    g4().context("11")
}
