use crate::my_error::Error;

mod errors;
mod m1;
mod m2;
mod gg;
mod my_error;

fn main() -> errors::Result<()> {
    // if let Err(e) = m1::hello_err() {
    //     println!("--1: {e}");
    //     println!("--2: {e:?}");
    //     println!("--3: {:?}", e)
    // }
    // if let Err(e) = m1::hello_file() {
    //     println!("--4: {e:?}");
    // }
    // if let Err(e) = m1::hello_err1() {
    //     println!("--5: {e}");
    //     println!("--6: {e:?}");
    // }
    // if let Err(e) = m1::hello_err2() {
    //     println!("--8: {e:?}");
    // }
    // if let Err(e) = m1::hello_anyhow() {
    //     println!("--10: {e:?}");
    // }

    if let Err(e) = gg::g1() {
        println!("--11: {e:?}");
        match e {
            Error::Wrap { error, .. } => {
                if let Some(e) = error.downcast_ref::<std::io::Error>() {
                    println!("IO error: {:?}", e);
                } else {
                    println!("Unknown error {error:?}");
                }
            }
            _ => {}
        }
    }
    if let Err(e) = gg::g2() {
        println!("--12: {e:?}");
    }
    if let Err(e) = gg::g3() {
        println!("--13: {e:?}");
    }
    if let Err(e) = gg::g5() {
        println!("--15: {e:?}");
    }

    Ok(())
}
