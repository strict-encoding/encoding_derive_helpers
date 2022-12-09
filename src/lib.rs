// Helper functions for creating encoding derivation macros.
//
// Written in 2019-2023 by
//     Dr. Maxim Orlovsky <orlovsky@protonmail.ch>
//
// This software is distributed according to the principle AS IS without any
// warranty.
//
// You should have received a copy of the Apache 2.0 License along with this
// software. If not, see <https://opensource.org/licenses/Apache-2.0>.

// Coding conventions
#![recursion_limit = "256"]
#![deny(dead_code, missing_docs, warnings)]

//! Helper functions for creating encoding derive crates, like
//! `strict_encoding_derive` or `lightning_encoding_derive`.
//!
//! To create a derive crate, just use the following sample:
//!
//! ```ignore
//! extern crate proc_macro;
//! #[macro_use]
//! extern crate amplify;
//!
//! use encoding_derive_helpers::{decode_derive, encode_derive, TlvEncoding};
//! use proc_macro::TokenStream;
//! use syn::DeriveInput;
//!
//! #[proc_macro_derive(StrictEncode, attributes(strict_encoding))]
//! pub fn derive_strict_encode(input: TokenStream) -> TokenStream {
//!     let derive_input = parse_macro_input!(input as DeriveInput);
//!     encode_derive(
//!         "strict_encoding",
//!         ident!(strict_encoding),
//!         ident!(StrictEncode),
//!         ident!(strict_encode),
//!         ident!(strict_serialize),
//!         derive_input,
//!         TlvEncoding::Denied,
//!     )
//!     .unwrap_or_else(|e| e.to_compile_error())
//!     .into()
//! }
//! ```

extern crate proc_macro;
#[macro_use]
extern crate amplify;
#[macro_use]
extern crate quote;

mod decode;
mod encode;
mod param;

pub use decode::decode_derive;
pub use encode::encode_derive;
