// the way of calling external libraries - here rust standard libraries
// this way it's possible to use methods from these traits
use std::io::Write;
use std::str::FromStr;

// fn main() {} is a function that's gonna be executed on code start
fn main() {
    // let mutable variable named numbers be a new, empty vector type
    // Vec is a dynamic length array, similar to python's list
    let mut numbers = Vec::new();

    for arg in std::env::args().skip(1) {
        // .push() -> append an element to vector
        // u64::from_str(&arg) -> parse an string argument to u64 type
        // .expect("err text") -> if an error occures return err text
        // otherwise when u64::from_str() gone ok .expect returns parsed value
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"))
    }

    if numbers.len() == 0 {
        // writeln! macro inputs given text into error stream <std::io::stderr()>
        // .unwrap is a method to check if printing writeln! has no errors
        writeln!(std::io::stderr(), "Call as: gdc NUMBER ...").unwrap();
        // std::process::exit() -> allows to force exit the program
        // normally rust treat code as properly finished when it gets to the end
        std::process::exit(1)
    }

    let mut d = numbers[0];
    // &something says that &something is refering to already existing something
    // *a says that we're going to get the data from already referenced data that *a and & point to
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("Greatest common divisor for numbers {:?} is {}", numbers, d);
}


fn gcd(mut a: u64, mut b: u64) -> u64 {
    // assert! is a built-in macro that is called with !
    // it checks at the function start if given arguments match needs
    assert!(a != 0 && b != 0);
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    // convention: no need to use return at the end of a function if the lats - to be returned -
    // element has no ; - in this case it is treated as returned one
    // as with convention return should be used inside the function if something is returned
    // before the end of the function functionality
    b
}

// rust test suite
// #[test] attribute is pointing that is a test function
// it's gonna be ommited while compiling th code
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 11), 1);

    assert_eq!(gcd(19*17*15*13, 19*13), 13*19);
}