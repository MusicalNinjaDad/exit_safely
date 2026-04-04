#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;
use try_v2::*;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination, Try, Try_ConvertResult)]
enum NoRepr<T: _T> {
    Ok(T),
    Fail,
}

fn main() {}
