use exit_safely::Termination;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination)]
enum NoRepr<T: _T> {
    Ok(T),
    Fail,
}

fn main() {}
