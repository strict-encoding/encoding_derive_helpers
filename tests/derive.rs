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

extern crate proc_macro;
#[macro_use]
extern crate amplify;
#[macro_use]
extern crate syn;

use encoding_derive_helpers::{decode_derive, encode_derive};
use proc_macro::TokenStream;
use syn::DeriveInput;

#[test]
fn test_custom_derivation() {
    #![allow(unused)]
    #[proc_macro_derive(CustomEncode, attributes(custom_encoding))]
    fn derive_strict_encode(input: TokenStream) -> TokenStream {
        let derive_input = parse_macro_input!(input as DeriveInput);
        encode_derive(
            "custom_encoding",
            ident!(custom_encoding),
            ident!(CustomEncode),
            ident!(custom_encode),
            ident!(custom_serialize),
            derive_input,
            true,
        )
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
    }

    #[proc_macro_derive(CustomDecode, attributes(custom_encoding))]
    fn derive_strict_decode(input: TokenStream) -> TokenStream {
        let derive_input = parse_macro_input!(input as DeriveInput);
        decode_derive(
            "custom_encoding",
            ident!(custom_encoding),
            ident!(CustomDecode),
            ident!(custom_decode),
            ident!(custom_deserialize),
            derive_input,
            true,
        )
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
    }
}
