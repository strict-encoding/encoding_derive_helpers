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

#[macro_use]
extern crate derive_test;

use std::io;

enum Error {}
trait CustomEncode {
    fn custom_encode(&self, _: &mut impl io::Write) -> Result<(), Error> {
        unreachable!()
    }
    fn custom_serialize(&self) -> Result<Vec<u8>, Error> {
        unreachable!()
    }
}
trait CustomDecode: Sized {
    fn custom_decode(_: &mut impl io::Read) -> Result<Self, Error> {
        unreachable!()
    }
    fn custom_deserialize(_: impl AsRef<[u8]>) -> Result<Self, Error> {
        unreachable!()
    }
}

impl CustomEncode for u8 {}
impl CustomDecode for u8 {}
impl CustomEncode for String {}
impl CustomDecode for String {}

#[test]
fn struct_numbered_fields() {
    #[derive(Clone, PartialEq, Eq, Debug, CustomEncode, CustomDecode)]
    #[custom_encoding(crate = crate)]
    struct NumberedFields(u8, String);
}
