mod errors;
mod m1;
mod m2;
mod kk;
mod gg;

fn main() -> errors::Result<()> {
    if let Err(e) = m1::hello_err() {
        println!("--1: {e}");
        println!("--2: {e:?}");
        println!("--3: {:?}", e)
    }
    if let Err(e) = m1::hello_file() {
        println!("--4: {e:?}");
    }
    if let Err(e) = m1::hello_err1() {
        println!("--5: {e}");
        println!("--6: {e:?}");
    }
    if let Err(e) = m1::hello_err2() {
        println!("--8: {e:?}");
    }
    if let Err(e) = m1::hello_anyhow() {
        println!("--10: {e:?}");
    }
    
    if let Err(e) = gg::g1() {
        println!("--11: {e:?}");
    }
    if let Err(e) = gg::g2() {
        println!("--12: {e:?}");
    }

    Ok(())
}
