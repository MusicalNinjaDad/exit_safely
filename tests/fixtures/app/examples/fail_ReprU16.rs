use exit_safely::Termination;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination)]
#[repr(u16)]
enum NoRepr<T: _T> {
    Ok(T) = 0,
    Fail = 1,
}

fn main() {
    fail
}
