# Exit rust apps safely

[`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html) warns:
> "Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread’s stack will be run. If a clean shutdown is needed it is recommended to ... simply return a type implementing Termination ... from the main function and avoid this function altogether"

`exit_safely` provides a simple and highly transparent option to `derive(Termination)` from your own enum with a very simple API which still provides you full control over exit codes and what to (safely) output to stderr.

Minimal magic, maximum flexibility, zero boilerplate.

## Example

Here's the full example from the integration tests, showing how this plays really nicely with `Try` (derived via [try_v2](https://docs.rs/try_v2/latest/try_v2/))

```rust
#![feature(never_type)]
#![feature(try_trait_v2)]
use exit_safely::Termination;
use try_v2::*;

use std::io;

use std::process::Termination as _T; // Needed as trait bound for Exit

#[derive(Debug, Termination, Try, Try_ConvertResult)]
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
    contents: String,
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
        contents,
    };

    // If everything goes well, check the data ...
    Exit::from(file)

    // Or you could of course just `Exit::Ok(())`
}
```

## Stability

> 🔬 **Experimental Features**
>
> This crate makes use of the following experimental features:
>
> - `#![feature(if_let_guard)]` ([tracking issue](https://github.com/rust-lang/rust/issues/51114))
> - `#![feature(iterator_try_collect)]` ([tracking issue](https://github.com/rust-lang/rust/issues/94047))
> - `#![feature(never_type)]` ([tracking issue](https://github.com/rust-lang/rust/issues/35121))
> - `#![feature(proc_macro_diagnostic)]` ([tracking issue](https://github.com/rust-lang/rust/issues/54140))
> - `#![feature(try_trait_v2)]` ([tracking issue](https://github.com/rust-lang/rust/issues/84277))
>
> Since `Termination` works best for types which also implement the experimental `Try`, we hope this is acceptable to you.
>
> The authors consider all of the above features to be reliable and already well advanced in the stabilisation process. Nevertheless, we run automated tests **every month** to ensure no fundamental changes affect this crate.
