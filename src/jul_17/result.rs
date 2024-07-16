use std::{
    error::Error, fmt::Debug, fs, io::{self, stderr, Write}, path::Path, result
};

pub type Result<T> = result::Result<T, MyError>;

pub enum MyError {
    DatabaseError(Box<dyn Debug>),
    ZeroDivisionError,
    InternalServerError,
}

impl From<MyError> for std::io::Error{
    fn from(value:MyError) -> Self {
        match value {
            MyError::DatabaseError(_) => todo!(),
            MyError::ZeroDivisionError => todo!(),
            MyError::InternalServerError => todo!(),
        }
        todo!()
    }
}

fn divide(divisor: i32, dividend: i32) -> Result<i32> {
    let res = dividend
        .checked_div(divisor)
        .ok_or(MyError::ZeroDivisionError)?;
    Ok(res)
    // dividend.checked_div(divisor).ok_or(MyError::ZeroDivisionError)
}
#[test]
fn something() {
    let (divisor, dividend) = (0, 6);
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
    fn hello() {println!("hello!");}
}
mod world {
    fn world() {println!("world!");}
}
mod hello_world {
    // use super::{hello::hello, world::world};
    // fn hello_world() {hello(); world();}
}