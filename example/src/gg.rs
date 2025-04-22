use std::fs::File;
use crate::kk::{MyError, KResult};

pub fn g1() -> KResult<()> {
    let _ = File::open("test.txt")?;

    Ok(())
}

pub fn g2() -> KResult<()> {
    let resp = reqwest::blocking::get("demo.haizhi.com/")?;

    Ok(())
}
