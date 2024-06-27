fn gcd_test() {
    use std::env;
    use std::str::FromStr;
    let mut numbers: Vec<u64> = Vec::new(); // (dynamically) growable vector type, but should define as a mutable
    for arg in env::args().skip(1) {
        // * Rust does not have exceptions: all errors are handled using either Result or panic
        numbers.push(u64::from_str(&arg).expect("error parsing argument")); // Type of vector can be infered by this line & expect => if Result is okay then return value
                                                                            // let test_type = u64::from_str(&arg); // Result value that indicates whether the parse succeeded or failed
    }

    if numbers.len() == 0 {
        eprintln!(
            r#"Usage: gcd 
        NUMBER ..."#
        ); // r#""# raw string syntax
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        // Rust iterators are very efficient -> almost same performance as handwritten loop
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0); // for test & ! for the macro
    while m != 0 {
        if m < n {
            let t = m; // As Rust can infer t's type, we don't need to write out type (only within func bodies)
            m = n;
            n = t;
        }
        m = m % n;
    }
    n // without semicolon return value
}
// assert! checks that its argument is true, and if it is not, terminates the program(panic! -> abrupt termination)

#[test] // inclue compilation only when run with `cargo test` command
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(
        gcd(
            2 * 3 * 5 * 11 * 17, // assert_eq!
            3 * 7 * 11 * 13 * 19
        ),
        3 * 11
    );
}

// * Cautious!
// TODO cargo test run test async job
