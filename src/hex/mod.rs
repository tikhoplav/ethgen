//! Hex encoding and serialization

mod encode;
#[doc(inline)]
pub use encode::{
	Encoder,
	IterEncoder,
	IterEncoderUpper,
};

 mod decode;
 #[doc(inline)]
 pub use decode::{Error, decode};
 #[doc(hidden)]
 pub use decode::const_decode;

mod serde;
#[doc(inline)]
pub use self::serde::{serialize};

mod macros;

/// Hex serializable type
pub trait Encode {
	type IntoEncoder: Encoder;

	fn into_encoder(self) -> Self::IntoEncoder;
}

// /// Hex deserializable type
// trait Decode: Sized {
// 	fn from_hex(src: impl AsRef<[u8]>) -> Result<Self, Error>;
// }
