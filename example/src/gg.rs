use std::fs::File;
use crate::kk::{Result};

pub fn g1() -> Result<()> {
    let _ = File::open("test.txt")?;

    Ok(())
}

pub fn g2() -> Result<()> {
    let resp = reqwest::blocking::get("demo.haizhi.com/")?;

    Ok(())
}
