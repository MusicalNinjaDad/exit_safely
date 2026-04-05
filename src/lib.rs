#![feature(if_let_guard)]
#![feature(iterator_try_collect)]

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
//!
//! For use in `main()` you will probably also want to derive `Debug` and `Try`
//! (via [try_v2](https://docs.rs/try_v2/latest/try_v2/)):
//!
//! ```rust
//! #![feature(never_type)]
//! #![feature(try_trait_v2)]
//! use exit_safely::Termination;
//! use try_v2::*;
//!
//! #[derive(Debug, Termination, Try, Try_ConvertResult)]
//! #[repr(u8)]
//! enum Exit<T: std::process::Termination> {
//!     Ok(T) = 0,
//!     Error(String) = 1,
//!     InvocationError(String) = 2,
//! }
//!
//! fn main() -> Exit<()> {
//!     // Use either `?` or return `Exit::...` to exit early from your code ...
//!     Exit::Ok(())
//! }
//!
//! ```
//!
//! See the integration tests or readme for a full example
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2_diagnostic::{
    DiagnosticResult::{self, Ok},
    DiagnosticStream,
};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Meta, Variant, parse_quote, spanned::Spanned};

#[proc_macro_derive(Termination)]
/// Derives Termination.
///
/// ## Requires:
///   - `#[repr(u8)]`
///   - Discriminant assigned to each variation, will be used as the ExitCode
///   - Generic parameter as type of the "Ok" case, which must implement std::process::Termination
///   - The data stored in any variants must implement Display
pub fn termination_derive(input: TokenStream1) -> TokenStream1 {
    impl_termination(input.into()).into()
}

fn impl_termination(input: TokenStream2) -> DiagnosticStream {
    let ast: DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let Data::Enum(enum_data) = &ast.data else {
        return DiagnosticResult::error("Termination can only be derived for an enum")
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
        Some(_) => DiagnosticResult::warn_spanned(
            (),
            repr.span(),
            "use #[repr(u8)] to ensure valid exit codes",
        ),
        None => {
            let span = enum_data
                .enum_token
                .span()
                .join(enum_data.brace_token.span.open())
                .expect("opening brace");
            DiagnosticResult::warn_spanned(
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
            .ok_or_else(|| {
                DiagnosticResult::error(
                    "Termination requires explicit discriminants to specify the correct ExitCodes",
                )
                .add_help(variant.span(), "add `= n` after this")
            })
            .map(|tuple| tuple.1)
    };

    let success_variant = enum_data.variants.first().ok_or(
        DiagnosticResult::error("Termination requires at least an Ok variant")
            .add_help(enum_data.brace_token.span.span(), "add `Ok(T) = 0` here"),
    )?;

    let check_success_variant_fields = match &success_variant.fields {
        Fields::Unnamed(fields)
            if fields.unnamed.len() == 1
            => Ok(()),
        Fields::Named(fields) => DiagnosticResult::error(
            "Termination requires the Ok variant to store a single unnamed value implementing `Termination`"
            )
            .add_help(fields.span(), "change this to `(T)`"),
        Fields::Unnamed(fields) => DiagnosticResult::error(
            "Termination requires the Ok variant to store a single unnamed value implementing `Termination`"
            )
            .add_help(fields.span(), "change this to `(T)`"),
        Fields::Unit => DiagnosticResult::error(
            "Termination requires the Ok variant to store a single value implementing `Termination`"
            )
            .add_help(success_variant.ident.span(), "add `(T)` after this"),
    };
    check_success_variant_fields?;

    let success_exit_code = get_discriminant(success_variant)?;
    if  success_exit_code != parse_quote!(0) {
        let span_to_first_variant = enum_data
            .enum_token
            .span()
            .join(success_variant.span())
            .expect("same source file");
        let span_of_discriminant_value = success_variant
            .discriminant
            .as_ref()
            .expect("guaranteed discriminant")
            .1
            .span();
        return DiagnosticResult::error("Termination requires an explicit success variant")
            .add_help(
                span_to_first_variant,
                "Did you forget to add a success variant here ...",
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
        .try_collect()?;

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
        .try_collect()?;

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
}
