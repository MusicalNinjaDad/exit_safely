//! `exit_safely` provides a simple and highly transparent option to `derive(Termination)` from
//! your own enum with a very simple API which still provides you full control over exit codes
//! and what to (safely) output to stderr.
//!
//! Minimal magic, maximum flexibilty, zero boilerplate.
//!
//! ## Why?
//!
//! [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html) warns: "Note that
//! because this function never returns, and that it terminates the process, no destructors on the
//! current stack or any other thread’s stack will be run. If a clean shutdown is needed it is
//! recommended to ... simply return a type implementing Termination ... from the main function
//! and avoid this function altogether"
//!
//! ## Example usage:
//! ```rust
//! use exit_safely::Termination;
//! use std::process::Termination as _Termination;
//!
//! #[derive(Termination)]
//! #[repr(u8)]
//! enum Exit<T: _Termination> {
//!     Ok(T) = 0,
//!     Error(String) = 1,
//!     InvocationError(String) = 2,
//! }
//! ```
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_derive(Termination)]
/// Derives Termination.
///
/// ## Requires:
///   - `#[repr(u8)]`
///   - Discrimanant assigned to each variation, will be used as the ExitCode
///   - Generic parameter as type of the "Ok" case, which must implement std::process::Termination
///   - The data stored in any variants must implement Display
pub fn termination_derive(input: TokenStream1) -> TokenStream1 {
    impl_termination(input.into()).into()
}

fn impl_termination(input: TokenStream2) -> TokenStream2 {
    TokenStream2::new()
}
