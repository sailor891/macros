use anyhow::Result;

#[macro_export]
macro_rules! my_try {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
}

fn main() -> Result<()> {
    let elem = my_try!(Ok(24));
    println!("{:?}", elem);
    Ok(())
}
