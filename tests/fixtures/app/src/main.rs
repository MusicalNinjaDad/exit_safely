#![allow(stable_features)]
#![feature(never_type)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]
use exit_safely::Termination;
use try_v2::*;

use std::io;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[must_use]
#[repr(u8)]
enum Exit<T: _T> {
    Ok(T) = 0,

    // Any io errors are ExitCode 1, delegate stderr contents to io::Error's Display
    FileError(io::Error) = 1,

    // ExitCode 2 can be manually created with a String to be sent to stderr
    InvocationError(String) = 2,

    // Just exit with ExitCode 3 and nothing extra sent to stderr
    Other = 3,
}

// Allows for simple `?` propogation of `io::Error`s
impl<T: _T> From<io::Error> for Exit<T> {
    fn from(err: io::Error) -> Self {
        Exit::FileError(err)
    }
}

// And for validation of data you can impl From<foo> for Exit
struct File {
    pass_or_fail: String,
    _contents: String,
}

impl From<File> for Exit<()> {
    fn from(file: File) -> Self {
        match file.pass_or_fail.as_str() {
            "PASS" => Exit::Ok(()),
            _ => Exit::Other,
        }
    }
}

// if given too few args it will fail with ExitCode 2 (InvocationError) and a message
// if given `<FILENAME> <FAIL>` it will fail with ExitCode (Other) 3,
// if given `<FILENAME> <ANYTHING_ELSE>` it will check file exists
fn main() -> Exit<()> {
    println!("Hello, world!");

    let mut args = std::env::args();

    let filename = args
        .nth(1)
        // `ok_or(Exit::foo)?` will exit safely with ExitCode from `foo`
        .ok_or(Exit::InvocationError("REALLY Not enough args".to_string()))?;

    let pass_or_fail = args
        .next()
        .ok_or(Exit::InvocationError("Not enough args".to_string()))?;

    // Using `?` on an Error which can convert to Exit will also exit safely with the relevant ExitCode
    let contents = std::fs::read_to_string(&filename)?;
    let file = File {
        pass_or_fail,
        _contents: contents,
    };

    // If everything goes well, check the data ...
    Exit::from(file)

    // Or you could of course just `Exit::Ok(())`
}
