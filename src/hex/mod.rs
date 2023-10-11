//! Hex encoding and serialization

mod error;
#[doc(inline)]
pub use error::Error;

mod encode;
#[doc(inline)]
pub use encode::{encode, encode_upper};

mod decode;
#[doc(hidden)]
pub use decode::const_decode;
#[doc(inline)]
pub use decode::decode;

// mod serde;
// #[doc(inline)]
// pub use self::serde::{serialize};

mod macros;

// /// Hex serializable type
// pub trait Encode {
// 	type IntoEncoder: Encoder;

// 	fn into_encoder(self) -> Self::IntoEncoder;
// }

// // /// Hex deserializable type
// // trait Decode: Sized {
// // 	fn from_hex(src: impl AsRef<[u8]>) -> Result<Self, Error>;
// // }

//
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crypto_bigint::{CheckedAdd, Encoding, U256};
//
//     use generic_array::{
//         typenum::{U, U8},
//         ArrayLength, GenericArray,
//     };
//
//     pub trait Encode {
//         const BYTES: usize;
//         // type BytesLength: generic_array::ArrayLength;
//         type BytesLength: ArrayLength;
//
//         fn get_buffer() -> GenericArray<u8, U<Self::BYTES>>;
//     }
//
//     #[derive(Debug)]
//     struct Reserves {
//         pub reserve0: U256,
//         pub reserve1: U256,
//         pub block_timestamp_last: u32,
//     }
//
//     // TODO::
//     // - implement encode and decode with `faster_hex`;
//     //   - add feature flag for `faster_hex`;
//     //   - add the existing functions as a fallback;
//     // - implement `encode_compressed` and `decode_compressed` (for short hex);
//     // - finalize FromHex and IntoHex traits;
//     //   - add buffer size;
//     // - return unhex! macro
//     // - implement serde:
//     //   - should deal with `0x` prefix;
//     //   - should allocate proper buffer;
//     //   - should call into_hex / from_hex;
//     impl Reserves {
//         fn into_hex(self, dst: &mut [u8]) {
//             let a = self.reserve0.to_be_bytes();
//             encode(a, &mut dst[0..64]);
//
//             let b = self.reserve1.to_be_bytes();
//             encode(b, &mut dst[64..128]);
//
//             let c = self.block_timestamp_last.to_be_bytes();
//             encode(c, &mut dst[184..192]);
//         }
//
//         fn from_hex(hex: &[u8]) -> Self {
//             let reserve0 = U256::from_be_slice(&hex[0..32]);
//             let reserve1 = U256::from_be_slice(&hex[32..64]);
//             let block_timestamp_last = u32::from_be_bytes(hex[92..96].try_into().unwrap());
//             Reserves {
//                 reserve0,
//                 reserve1,
//                 block_timestamp_last,
//             }
//         }
//     }
//
//     #[test]
//     fn test_something() {
//         panic!("{}", "some panic here");
//     }
//
//     #[test]
//     fn uint_hex_test() {
//         let a = "0x0000000000000000000000000000000000000000000000000000000034073387000000000000000000000000000000000000000000000000078cfccdc5353a9100000000000000000000000000000000000000000000000000000000652244f6";
//         let mut buf = [0u8; 96];
//         decode(&a[2..], &mut buf).unwrap();
//
//         let mut reserves = Reserves::from_hex(&buf);
//
//         reserves.reserve0 = reserves.reserve0.checked_add(&reserves.reserve1).unwrap();
//
//         let mut buf = [48u8; 194];
//         buf[0] = 48u8;
//         buf[1] = 120u8;
//
//         reserves.into_hex(&mut buf[2..]);
//
//         let b = unsafe { core::str::from_utf8_unchecked(&buf) };
//         panic!("\n{}\n{}\n", a, b);
//
//         // let a = "0000000000000000000000000000000000000000000000000000000000000001";
//         // let val = U256::from_be_hex(a);
//         // let val = val.checked_add(&U256::ONE).unwrap();
//
//         // let mut buf = [0u8; 66];
//         // buf[0] = 48u8;
//         // buf[1] = 120u8;
//
//         // let src = val.to_be_bytes();
//         // encode(src, &mut buf[2..]);
//         // let b = unsafe { core::str::from_utf8_unchecked(&buf) };

// assert_eq!(
// 	"0x0000000000000000000000000000000000000000000000000000000000000002",
// 	b
// );
//     }
// }
