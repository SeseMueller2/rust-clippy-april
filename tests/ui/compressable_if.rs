#![warn(clippy::compressable_if)]
#![allow(clippy::needless_if)]

fn main() -> Result<(), ()> {
    let test = 1;
    if test == 1 {
        println!("odd");
    }
    if test == 2 {
        println!("even");
    }
    if test == 3 {
        println!("odd");
    }

    if test == 1 {}
    if test == 3 {}

    // Negative Tests

    if test == 1 {
        println!("test is 1, so odd");
    }
    if test == 2 {
        println!("test is 2, so even");
    }
    if test == 3 {
        println!("test is 3, so odd");
    }

    Ok(())
}
