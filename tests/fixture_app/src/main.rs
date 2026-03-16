#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;
use try_v2::*;

fn main() -> Exit<()> {
    println!("Hello, world!");
    let mut args = std::env::args();
    match args.len() - 1 {
        2 => return Exit::InvocationError(args.nth(2).unwrap()),
        3 => return Exit::Other,
        _ => (),
    }
    Exit::Ok(())
}

#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[repr(u8)]
enum Exit<T: std::process::Termination> {
    Ok(T) = 0,
    Error(String) = 1,
    InvocationError(String) = 2,
    Other = 3,
}
