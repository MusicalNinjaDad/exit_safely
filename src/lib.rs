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
use syn::{Data, DeriveInput};

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
    let ast: DeriveInput = syn::parse2(input).unwrap();

    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();

    let Data::Enum(enum_data) = ast.data else {
        todo!()
    };

    let success_variant = &enum_data.variants[0].ident; //TODO: validate field type & discriminant
    let silent_fail_variants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| variant.fields.is_empty())
        .map(|variant| variant.ident.clone());
    let silent_fail_discriminants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| variant.fields.is_empty())
        .map(|variant| variant.discriminant.clone().unwrap().1);
    let fail_message_variants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| !variant.fields.is_empty())
        .map(|variant| variant.ident.clone());
    let fail_message_discriminants = enum_data
        .variants
        .iter()
        .skip(1)
        .filter(|variant| !variant.fields.is_empty())
        .map(|variant| variant.discriminant.clone().unwrap().1);

    quote! {
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
    }
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
            impl_termination(original).to_string()
        );
    }
}
