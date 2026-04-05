use exit_safely::Termination;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination)]
#[repr(u8)]
enum NotTermination<T> {
    Ok(T) = 0,
    Fail = 1,
}

#[derive(Debug, Termination)]
#[repr(u8)]
enum Unit {
    Ok = 0,
    Fail = 1,
}

#[derive(Debug, Termination)]
#[repr(u8)]
enum NamedFields<T: _T> {
    Ok { outcome: T } = 0,
    Fail = 1,
}

#[derive(Debug, Termination)]
#[repr(u8)]
enum TooManyUnnamedFields<T: _T> {
    Ok(T, u8) = 0,
    Fail = 1,
}

#[derive(Debug, Termination)]
#[repr(u8)]
enum NotExitCodeZero<T: _T> {
    Ok(T) = 1,
    Fail = 2,
}

fn main() {
    fail
}
