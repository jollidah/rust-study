use std::mem::size_of;

#[derive(Debug,PartialEq, PartialOrd)]
enum HttpStatus {
    Ok,
    NotModified,
    NotFound,
}

pub enum RoughTime {
    InThePast(u8, u32),
    JustNow,
    InTheFuture(u8, u32),
}

impl HttpStatus{
    fn show(&self){
        println!("{:?}", self);
    }
}

#[test]
fn test_partial_eq_enum() {
    assert!(HttpStatus::NotModified < HttpStatus::NotFound);
    let a = HttpStatus::Ok;
    a.show();
    println!("{:?}", HttpStatus::NotModified as i32);
    println!("{:?}", size_of::<RoughTime>());
    println!("{:?}", size_of::<(u8, u32, u8, u8, u16)>());
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units),
        // RoughTime::InThePast(units, 1) => format!("{} {} ago", 1, units),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units),
    }
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

fn greet_people(names: &[&str]) {
    match names {
        [] => {
            println!("Hello, nobody.")
        }
        ["Seung Woo"] => {
            println!("Hello, Seung Woo.")
        }
        [a] => {
            println!("Hello, {}.", a)
        }
        [a, b] => {
            println!("Hello, {} and {}.", a, b)
        }
        [a, .., b] => {
            println!("Hello, everyone from {} to {}.", a, b)
        }
    }
}

fn binding_pattern(arr: &[i32]) {
    match &arr{
        target @ [1, ..] => println!("First value is one!: {:?}", target),
        _ => println!("First value isn't one!")
    }
}

#[test]
fn test_binding_pattern() {
    binding_pattern(&[1, 2]);
    binding_pattern(&[2, 2]);
}
