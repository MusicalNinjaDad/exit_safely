#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;
use std::io;
use try_v2::*;

fn main() -> Exit<()> {
    println!("Hello, world!");
    let mut args = std::env::args();
    // `ok_or(Exit::foo)?` will exit safely with ExitCode from `foo`
    if args
        .nth(2)
        .ok_or(Exit::InvocationError("Not enough args".to_string()))?
        == "FAIL"
    {
        // directly returning `Exit::foo` will exit safely with ExitCode from `foo`
        return Exit::Other;
    }
    Exit::Ok(())
}

#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[repr(u8)]
enum Exit<T: std::process::Termination> {
    Ok(T) = 0,
    FileError(io::Error) = 1,
    InvocationError(String) = 2,
    Other = 3,
}
