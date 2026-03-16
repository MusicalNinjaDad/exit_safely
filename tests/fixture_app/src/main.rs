#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;
use std::process::Termination as _T;
use std::io;
use try_v2::*;

fn main() -> Exit<()> {
    println!("Hello, world!");
    let mut args = std::env::args();
    let filename = args
        .nth(1)
        .ok_or(Exit::InvocationError("REALLY Not enough args".to_string()))?;

    if args
        .next()
        // `ok_or(Exit::foo)?` will exit safely with ExitCode from `foo`
        .ok_or(Exit::InvocationError("Not enough args".to_string()))?
        == "FAIL"
    {
        // directly returning `Exit::foo` will exit safely with ExitCode from `foo`
        return Exit::Other;
    }
    
    
    // Using `?` on an Error which can convert to Exit will also exit safely with the relevant ExitCode
    let _file_contents = std::fs::read_to_string(&filename)?;

    // If everything goes well ...
    Exit::Ok(())
}

#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[repr(u8)]
enum Exit<T: _T> {
    Ok(T) = 0,
    FileError(io::Error) = 1,
    InvocationError(String) = 2,
    Other = 3,
}

impl<T: _T> From<io::Error> for Exit<T> {
    fn from(err: io::Error) -> Self {
        Exit::FileError(err)
    }
}