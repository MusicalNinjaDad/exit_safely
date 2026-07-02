//! `exit_safely` provides a simple and highly transparent option to `derive(Termination)` from
//! your own enum with a very simple API which still provides you full control over exit codes
//! and what to (safely) output to stderr.
//!
//! Minimal magic, maximum flexibility, zero boilerplate.
//!
//! ## Why?
//!
//! [`std::process::exit`](https://doc.rust-lang.org/std/process/fn.exit.html) warns: "Note that
//! because this function never returns, and that it terminates the process, no destructors on the
//! current stack or any other thread’s stack will be run. If a clean shutdown is needed it is
//! recommended to ... simply return a type implementing Termination ... from the main function
//! and avoid this function altogether"
//!
//! ## Example on nightly
//!
//! For the best use in `main()` you will probably also want to derive `Debug` and `Try`
//! (via [try_v2](https://docs.rs/try_v2/latest/try_v2/)):
//!
//! ```rust
//! #![cfg_attr(unstable_never_type, feature(never_type))]
//! #![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]
//! #![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]
//! use std::process::Termination as _T;
//! use exit_safely::Termination;
//! # #[cfg(has_try_trait_v2)]
//! use try_v2::Try;
//!
//! # #[cfg(has_try_trait_v2)]
//! /// First define your exit codes:
//! #[derive(Debug, Termination, Try)]
//! #[FromResidual(Result<_, Self::Residual>)]
//! #[must_use]
//! #[repr(u8)]
//! enum Exit<T: _T> {
//!     Ok(T) = 0,
//!     Error(String) = 1,
//!     InvocationError(String) = 2,
//! }
//!
//! # #[cfg(has_try_trait_v2)]
//! /// Then any conversion:
//! /// clap errors return exit_code 2 & output the details
//! /// to stderr, letting clap handle formatting
//! impl<T: _T> From<clap::Error> for Exit<T> {
//!     fn from(err: clap::Error) -> Self {
//!         Self::InvocationError(err.to_string())
//!     }
//! }
//! # struct Cli {}
//! # impl Cli {
//! #     fn try_parse() -> Result<Self, clap::Error> {
//! #         Ok(Cli {})
//! #     }
//! # }
//! # #[cfg(has_try_trait_v2)]
//! # impl From<i32> for Exit<()> {
//! #     fn from(value: i32) -> Self {
//! #         Exit::Ok(())
//! #     }
//! # }
//! # fn process<I: IntoIterator>(v: I) -> Result<i32, clap::Error> {
//! #    Ok(5)
//! # }
//! #
//! # #[cfg(has_try_trait_v2)]
//! fn main() -> Exit<()> {
//!     // Use `?` to return the right exit code for the error type
//!     let cli = Cli::try_parse()?;
//!
//!     # let mut inputs = [4].into_iter();
//!     // ok_or()? converts a missing value to an exit
//!     let value = inputs
//!         .next()
//!         .ok_or(Exit::Error("Not enough input, need more cheese".to_string()))?;
//!
//!     // if your central processing returns a value which might be invalid
//!     Exit::from(process(inputs)?)
//!
//!     // or simply return `Exit::...`
//!     // Exit::Ok(())
//! }
//! # #[cfg(not(has_try_trait_v2))]
//! # fn main() {}
//! ```
//!
//! ## Example on stable
//!
//! If you prefer not to use a nightly toolchain then `exit_safely` still works fine, although
//! you cannot leverage the power of `?` just yet. The same example looks more like the pattern
//! if you used std::process::exit (or go)
//!
//! ```rust
//! use std::process::Termination as _T;
//! use exit_safely::Termination;
//!
//! /// First define your exit codes:
//! #[derive(Debug, Termination)]
//! #[must_use]
//! #[repr(u8)]
//! enum Exit<T: _T> {
//!     Ok(T) = 0,
//!     Error(String) = 1,
//!     InvocationError(String) = 2,
//! }
//!
//! /// Then any conversion:
//! /// clap errors return exit_code 2 & output the details
//! /// to stderr, letting clap handle formatting
//! impl<T: _T> From<clap::Error> for Exit<T> {
//!     fn from(err: clap::Error) -> Self {
//!         Self::InvocationError(err.to_string())
//!     }
//! }
//!
//! # struct Cli {}
//! # impl Cli {
//! #     fn try_parse() -> Result<Self, clap::Error> {
//! #         Ok(Cli {})
//! #     }
//! # }
//! # fn process<I: IntoIterator>(v: I) {}
//! #
//! fn main() -> Exit<()> {
//!     // Match on `Results` and use `into` to return the right exit code for the error type
//!     let cli = match Cli::try_parse() {
//!         Ok(cli) => cli,
//!         Err(e) => return e.into(), // or `Exit::from(e)` if you prefer
//!     };
//!
//!     # let mut inputs = [4].into_iter();
//!     // `let ... else` converts a missing value to an exit
//!     let Some(value) = inputs.next() else {
//!         return Exit::Error("Not enough input, need more cheese".to_string())
//!     };
//!
//!     process(inputs);
//!     Exit::Ok(())
//! }
//! ```
//!
//! > 🔬 **Stability**
//! >
//! > This crate makes use of the following experimental features if they are available but
//! > **does not require** any experimental features to work.
//! >
//! > ### For improved compiler errors
//! >
//! > - [`#![feature(proc_macro_diagnostic)]`](https://github.com/rust-lang/rust/issues/54140)
//! >
//! > To provide nicer compiler errors with "help", "notes" and a warning if you forget the repr(u8)
//! >
//! > On current stable help & notes are output as extra errors, but warnings are not possible
//! >
//! > ### Recommended: try_trait_v2, try_trait_v2_residual & never_type
//! >
//! > - [`#![feature(never_type)]`](https://github.com/rust-lang/rust/issues/35121)
//! > - [`#![feature(try_trait_v2)]`](https://github.com/rust-lang/rust/issues/84277)
//! > - [`#![feature(try_trait_v2_residual)]`](https://github.com/rust-lang/rust/issues/91285)
//! >
//! > I find the ergonomics work best for types which also implement the experimental `Try` and
//! > created the crate [`try_v2`](https://crates.io/crates/try_v2) to make it easy for you to
//! > take advantage.
//! >
//! > I consider all of the above features to be reliable and already well advanced in the
//! > stabilisation process. Nevertheless, I run automated tests **every month** and on every PR
//! > against stable, beta & nightly to ensure no fundamental changes affect this crate. I get a
//! > direct notification on my phone for any failures.
//! >
//! > Feel free to check out the build script, workflows and source to see how I do this.

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostic::{ToDiagnostic, ToTokens, prelude::*};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Meta, Variant, parse_quote, spanned::Spanned};
#[cfg(has_try_trait_v2)]
use try_v2::Transform;

#[proc_macro_derive(Termination)]
/// Derives Termination.
///
/// ## Requires:
///   - `#[repr(u8)]`
///   - Discriminant assigned to each variation, will be used as the ExitCode
///   - Generic parameter as type of the "Ok" case, which must implement std::process::Termination
///   - The data stored in any variants must implement Display
pub fn termination_derive(input: TokenStream1) -> TokenStream1 {
    impl_termination(input.into()).to_tokens()
}

fn impl_termination(input: TokenStream2) -> DiagnosticStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let Data::Enum(enum_data) = &ast.data else {
        return error("Termination can only be derived for an enum")
            .add_help(name.span(), "not an enum");
    };

    let repr = ast
        .attrs
        .iter()
        .find(|attr| attr.meta.path().is_ident(&format_ident!("repr")));
    let check_valid_repr = match repr {
        Some(repr)
            if let Meta::List(ml) = &repr.meta
                && ml
                    .parse_args::<Ident>()
                    .is_ok_and(|repr| repr == format_ident!("u8")) =>
        {
            Ok(())
        }
        Some(_) => warn_spanned(
            (),
            repr.span(),
            "use #[repr(u8)] to ensure valid exit codes",
        ),
        None => {
            let span = enum_data.enum_token.span();
            warn_spanned(
                (),
                span,
                "add #[repr(u8)] above this to allow for valid error codes",
            )
        }
    };
    check_valid_repr?;

    let get_discriminant = |variant: &Variant| {
        variant
            .discriminant
            .clone()
            .or_error(
                "Termination requires explicit discriminants to specify the correct ExitCodes",
            )
            .add_help(variant.span(), "add `= n` after this")
            .map(|tuple| tuple.1)
    };

    let success_variant = enum_data
        .variants
        .first()
        .or_error("Termination requires at least an Ok variant")
        .add_help(enum_data.brace_token.span.span(), "add `Ok(T) = 0` here")?;

    let check_success_variant_fields = match &success_variant.fields {
        Fields::Unnamed(fields)
            if fields.unnamed.len() == 1
            => Ok(()),
        Fields::Named(fields) => error(
            "Termination requires the Ok variant to store a single unnamed value implementing `Termination`"
            )
            .add_help(fields.span(), "change this to `(T)`"),
        Fields::Unnamed(fields) => error(
            "Termination requires the Ok variant to store a single unnamed value implementing `Termination`"
            )
            .add_help(fields.span(), "change this to `(T)`"),
        Fields::Unit => error(
            "Termination requires the Ok variant to store a single value implementing `Termination`"
            )
            .add_help(success_variant.ident.span(), "add `(T)` after this"),
    };
    check_success_variant_fields?;

    let success_exit_code = get_discriminant(success_variant)?;
    if success_exit_code != parse_quote!(0) {
        let span_of_discriminant_value = success_variant
            .discriminant
            .as_ref()
            .expect("guaranteed discriminant")
            .1
            .span();
        return error("Termination requires an explicit success variant")
            .add_help(
                success_variant.span(),
                "Did you forget to add a success variant before this ...",
            )
            .add_help(span_of_discriminant_value, "...or should this be 0?");
    };

    let success_variant = &success_variant.ident;

    let silent_fail_variants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| variant.fields.is_empty())
        .map(|variant| variant.ident.clone());
    let silent_fail_discriminants: Vec<_> = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| variant.fields.is_empty())
        .map(get_discriminant)
        // TODO: #63 Make this try_collect() when it gets stable
        .collect::<DiagnosticResult<Vec<_>>>()?;

    let fail_message_variants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| !variant.fields.is_empty())
        .map(|variant| variant.ident.clone());
    let fail_message_discriminants: Vec<_> = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| !variant.fields.is_empty())
        .map(get_discriminant)
        // TODO: #63 Make this try_collect() when it gets stable
        .collect::<DiagnosticResult<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics std::process::Termination for #name #ty_generics #where_clause {
            fn report(self) -> std::process::ExitCode {
                match self {
                    #name::#success_variant(v) => v.report(),
                    #(#name::#silent_fail_variants => std::process::ExitCode::from(#silent_fail_discriminants),)*
                    #(#name::#fail_message_variants(msg) => {
                        let mut stderr = std::io::stderr();
                        _ = std::io::Write::write_fmt(&mut stderr, std::format_args!("{}\n", msg));
                        std::process::ExitCode::from(#fail_message_discriminants)
                    })*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive() {
        let original = quote! {
            #[derive(Termination)]
            #[repr(u8)]
            enum Exit<T: _Termination> {
                Ok(T) = 0,
                Error(String) = 1,
                InvocationError(String) = 2,
                Other = 3,
            }
        };
        let expected_impl = quote! {
            impl<T: _Termination> std::process::Termination for Exit<T> {
                fn report(self) -> std::process::ExitCode {
                    match self {
                        Exit::Ok(v) => v.report(),
                        Exit::Other => std::process::ExitCode::from(3),
                        Exit::Error(msg) => {
                            let mut stderr = std::io::stderr();
                            _ = std::io::Write::write_fmt(&mut stderr, std::format_args!("{}\n", msg));
                            std::process::ExitCode::from(1)
                        }
                        Exit::InvocationError(msg) => {
                            let mut stderr = std::io::stderr();
                            _ = std::io::Write::write_fmt(&mut stderr, std::format_args!("{}\n", msg));
                            std::process::ExitCode::from(2)
                        }
                    }
                }
            }
        };
        assert_eq!(
            expected_impl.to_string(),
            impl_termination(original).unwrap().to_string()
        );
    }

    #[test]
    fn has_try() {
        assert!(cfg!(has_try_trait_v2));
    }
}
