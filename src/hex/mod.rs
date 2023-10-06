//! Hex (base16) serialization / deserialization
//!
//! Provides two traits `Encode` and `Decode` that enables use of serde macros
//! for automatic serialization / deserialization of a structure from / into a
//! hex encoded string.
//!
//! ```
//!	use ethgen::{hex, typenum};
//! use serde::Serialize;
//!
//! struct Value([u8; 8]);
//!
//! impl AsRef<[u8]> for Value {
//!		fn as_ref(&self) -> &[u8] {
//!			self.0.as_ref()
//! 	}
//! }
//!
//! impl hex::Encode for Value {
//!		// Should match the length of underlying data
//!		type BytesLength = typenum::U8;
//! }
//!
//! #[derive(Serialize)]
//! struct Data {
//!		#[serde(with = "hex")]
//!		value: Value,
//! }
//!
//!
//!
//! let data = Data {
//!		value: Value(*b"\0\0ethers"),
//! };
//!
//! let mut buf = [0u8; 256];
//! // serde_json can be used if `std` is supported
//! let n = serde_json_core::to_slice(&data, &mut buf).unwrap();
//! let json = &buf[..n];
//!
//! let expected = br#"{"value":"0x0000657468657273"}"#;
//!
//!	assert_eq!(expected, json);
//! ```
use generic_array::ArrayLength;

mod hex_full;
#[doc(inline)]
pub use hex_full::{serialize};

/// Hex serializable type
pub trait Encode: AsRef<[u8]> {
	type BytesLength: ArrayLength;
}
