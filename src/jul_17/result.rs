use std::{
    error::Error,
    fmt::Debug,
    fs,
    io::{self, stderr, Write},
    path::Path,
    result,
};

pub type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug)]
pub enum MyError {
    DatabaseError(Box<dyn Debug>),
    ZeroDivisionError,
    InternalServerError,
}

impl From<MyError> for std::io::Error {
    fn from(value: MyError) -> Self {
        match value {
            MyError::DatabaseError(_) => todo!(),
            MyError::ZeroDivisionError => todo!(),
            MyError::InternalServerError => todo!(),
        }
        todo!()
    }
}

pub fn divide(divisor: i32, dividend: i32) -> Result<i32> {
    let res = dividend
        .checked_div(divisor)
        .ok_or(MyError::ZeroDivisionError)?;
    Ok(res)
    // dividend.checked_div(divisor).ok_or(MyError::ZeroDivisionError)
}
#[test]
fn something() {
    let (divisor, dividend) = (0, 6);

    let a = divide(1, 1);
    drop(a);
    let res = match divide(divisor, dividend) {
        Ok(res) => res,
        Err(err) => match err {
            MyError::ZeroDivisionError => i32::MAX,
            _ => panic!("Impossible"),
        },
    };
}

/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or /// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        // opening dir could fail
        let entry = entry_result?; // reading dir could fail
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // renaming could fail
    }
    Ok(()) // phew!
}

mod hello {
    fn hello() {
        println!("hello!");
    }
}
mod world {
    fn world() {
        println!("world!");
    }
}
mod hello_world {
    // use super::{hello::hello, world::world};
    // fn hello_world() {hello(); world();}
}

#[test]
fn error_propagate() {
    println!("test start");
    first();
    println!("test end");
}
fn first() -> Result<()> {
    println!("first start");
    second()?;
    println!("first end");
    Ok(())
}
fn second() -> Result<()> {
    println!("second start");
    Err(MyError::InternalServerError)
    // Ok(())
}

#[test]
fn drop_trait() {
    #[derive(Debug)]
    struct MyString(&'static str);
    impl Drop for MyString {
        fn drop(&mut self) {
            println!("{:?} dropped!", self);
        }
    }

    let my_string = MyString("This is MyString!");
}

#[test]
fn pring_option() {
    let a = print_a();
    println!("{:?}", a);
}

fn print_a() -> Option<i32> {
    let a = some_a();
    println!("{:?}", a);

    let b = some_a()?;
    println!("{:?}", b);
    println!("after propagate");
    a
}

fn some_a() -> Option<i32> {
    // Some(1)
    None
}

#[test]
fn warn_error() {}

fn func_a() -> Result<()> {
    func_b();
    Ok(())
}
fn func_b() -> Result<i32> {
    Ok(1)
}
