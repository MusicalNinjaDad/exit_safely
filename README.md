# exit_safely - with Drop & meaningful exit codes

`exit_safely` provides a simple and highly transparent option to `derive(Termination)` from
your own enum with a very simple API which still provides you full control over exit codes
and what to (safely) output to stderr.

Minimal magic, maximum flexibilty, zero boilerplate.

## Why?

[`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html) warns: "Note that
because this function never returns, and that it terminates the process, no destructors on the
current stack or any other thread’s stack will be run. If a clean shutdown is needed it is
recommended to ... simply return a type implementing Termination ... from the main function
and avoid this function altogether"

## Example on nightly

For the best use in `main()` you will probably also want to derive `Debug` and `Try`
(via [try_v2](https://docs.rs/try_v2/latest/try_v2/)):

```rust
#![cfg_attr(unstable_never_type, feature(never_type))]
#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]
use std::process::Termination as _T;
use exit_safely::Termination;
use try_v2::{Try, Try_ConvertResult};

/// First define your exit codes:
#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[must_use]
#[repr(u8)]
enum Exit<T: _T> {
    Ok(T) = 0,
    Error(String) = 1,
    InvocationError(String) = 2,
}

/// Then any conversion:
/// clap errors return exit_code 2 & output the details
/// to stderr, letting clap handle formatting
impl<T: _T> From<clap::Error> for Exit<T> {
    fn from(err: clap::Error) -> Self {
        Self::InvocationError(err.to_string())
    }
}

fn main() -> Exit<()> {
    // Use `?` to return the right exit code for the error type
    let cli = Cli::try_parse()?;

    // ok_or()? converts a missing value to an exit
    let value = inputs
        .next()
        .ok_or(Exit::Error("Not enough input, need more cheese".to_string()))?;

    // if your central processing returns a value which might be invalid
    Exit::from(process(inputs)?)

    // or simply return `Exit::...`
    // Exit::Ok(())
}
```

## Example on stable

If you prefer not to use a nightly toolchain then `exit_safely` still works fine, although
you cannot leverage the power of `?` just yet. The same example looks more like the pattern
if you used std::process::exit (or go)

```rust
use std::process::Termination as _T;
use exit_safely::Termination;

/// First define your exit codes:
#[derive(Debug, Termination)]
#[must_use]
#[repr(u8)]
enum Exit<T: _T> {
    Ok(T) = 0,
    Error(String) = 1,
    InvocationError(String) = 2,
}

/// Then any conversion:
/// clap errors return exit_code 2 & output the details
/// to stderr, letting clap handle formatting
impl<T: _T> From<clap::Error> for Exit<T> {
    fn from(err: clap::Error) -> Self {
        Self::InvocationError(err.to_string())
    }
}

fn main() -> Exit<()> {
    // Match on results and use into to return the right exit code for the error type
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => return e.into(), // or `Exit::from(e)` if you prefer
    };

    // let else converts a missing value to an exit
    let Some(value) = inputs.next() else {
        return Exit::Error("Not enough input, need more cheese".to_string())
    };

    process(inputs);
    Exit::Ok(())
}
```

> 🔬 **Stability**
>
> This crate makes use of the following experimental features if they are available but
> **does not require** any experimental features to work.
>
> ### For improved compiler errors
>
> - [`#![feature(proc_macro_diagnostic)]`](https://github.com/rust-lang/rust/issues/54140)
>
> To provide nicer compiler errors with "help", "notes" and a warning if you forget the repr(u8)
>
> On current stable help & notes are output as extra errors, but warnings are not possible
>
> ### Recommended: try_trait_v2, try_trait_v2_residual & never_type
>
> - [`#![feature(never_type)]`](https://github.com/rust-lang/rust/issues/35121)
> - [`#![feature(try_trait_v2)]`](https://github.com/rust-lang/rust/issues/84277)
> - [`#![feature(try_trait_v2_residual)]`](https://github.com/rust-lang/rust/issues/91285)
>
> I find the ergonomics work best for types which also implement the experimental `Try` and
> created the crate [`try_v2`](https://crates.io/crates/try_v2) to make it easy for you to
> take advantage.
>
> I consider all of the above features to be reliable and already well advanced in the
> stabilisation process. Nevertheless, I run automated tests **every month** and on every PR
> against stable, beta & nightly to ensure no fundamental changes affect this crate. I get a
> direct notification on my phone for any failures.
>
> Feel free to check out the build script, workflows and source to see how I do this.
