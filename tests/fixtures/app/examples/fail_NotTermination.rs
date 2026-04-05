#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination)]
#[repr(u8)]
enum NotTermination<T> {
    Ok(T) = 0,
    Fail = 1,
}

// #[derive(Debug, Termination)]
// #[repr(u8)]
// enum Unit {
//     Ok = 0,
//     Fail = 1,
// }

fn main() {
    fail
}
