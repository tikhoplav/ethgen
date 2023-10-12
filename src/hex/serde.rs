use generic_array::{GenericArray, typenum::{Sum, Prod, U2}, ArrayLength};
use serde::Serializer;
use crate::Encode;
use super::encode;
use core::{str, ops::{Mul,Add}};

type HexSize<E> = Prod<<E as Encode>::ByteSize, U2>;
type HexPrefSize<E> = Sum<HexSize<E>, U2>;

/// Serialize a struct into a single hex string
pub fn serialize<E, S>(data: E, serializer: S) -> Result<S::Ok, S::Error>
where
	E: Encode,
	S: Serializer,
	<E as Encode>::ByteSize: ArrayLength,
	<E as Encode>::ByteSize: Mul<U2>,
	Prod<<E as Encode>::ByteSize, U2>: Add<U2>,
	HexSize<E>: ArrayLength,
	HexPrefSize<E>: ArrayLength,
{
	// Allocate the buffer with length to contain the byte repr of the struct
	let mut buf = GenericArray::<u8, E::ByteSize>::default();

	// Copy struct bytes to the buffer
	if data.into_bytes(&mut buf).is_err() {
		return Err(serde::ser::Error::custom("failed to bufferize data"));
	}

	// Allocate the buffer 2*n + 2 bytes to contain hex encoded structure
	let mut hex = GenericArray::<u8, HexPrefSize<E>>::default();

	// Add hex prefix `0x`
	hex[0] = 48u8;
	hex[1] = 120u8;

	if encode(&buf, &mut hex[2..]).is_err() {
		return Err(serde::ser::Error::custom("failed to hex encode data"));
	}

	// Safety: bytes returned from Encode::into_hex are valid ascii cahracters
	let s = unsafe { str::from_utf8_unchecked(&buf) };

	serializer.serialize_str(&s)
}
