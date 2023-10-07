use serde::Serializer;
use super::Encode;
use core::str;

/// Serialize a struct into a single hex string
pub fn serialize<E, S>(data: E, serializer: S) -> Result<S::Ok, S::Error>
where
	E: Encode,
	S: Serializer,
{
	// TODO: Use buffer with compile time determined length
	// let mut buf = GenericArray::<u8, U<Encode::BYTES>>::default();
	let mut buf = [0u8; 256];
	// TODO: transfer prefix creation login into Encoder trait and impl
	buf[0] = 48u8;
	buf[1] = 120u8;

	let encoder = data.into_encoder();

	// TODO: use encoder.len() instead as it provided with the trait
	let mut n: usize = 0;
	encoder.zip(buf.iter_mut().skip(2)).for_each(|(src, dst)| {
		*dst = src;
		n += 1;
	});

	// Safety: bytes returned from Encode::into_hex are valid ascii cahracters
	let s = unsafe { str::from_utf8_unchecked(&buf[..n + 2]) };
	serializer.serialize_str(&s)
}

#[cfg(test)]
mod test {
	use super::*;
	use super::super::IterEncoder;
	use core::{slice, iter};
	use generic_array::{GenericArray, typenum::U18};
	use bytes::Bytes;

	#[derive(Debug)]
	struct Foo (
		[u8; 12],
		GenericArray<u8, U18>,
		Bytes,
	);

	// impl<'a> IntoIterator for &'a Foo {
	// 	type Item = &'a u8;
	// 	type IntoIter = iter::Chain<
	// 		iter::Chain<
	// 			slice::Iter<'a, u8>, slice::Iter<'a, u8>
	// 		>, slice::Iter<'a, u8>
	// 	>;

	// 	fn into_iter(self) -> Self::IntoIter {
	// 		self.0.iter().chain(self.1.iter()).chain(self.2.iter())
	// 	}
	// }

	impl<'a> Encode for &'a Foo {
		type IntoEncoder = IterEncoder<'a, iter::Chain<
			iter::Chain<
				slice::Iter<'a, u8>, slice::Iter<'a, u8>
			>, slice::Iter<'a, u8>
		>>;

		fn into_encoder(self) -> Self::IntoEncoder {
			IterEncoder::new(
				self.0.iter().chain(self.1.iter()).chain(self.2.iter())
			)
		}
	}

	#[test]
	fn test_encoder() {
		let foo = Foo(
			*b"ethgen does ",
			(*b"no heap allocation").into(),
			Bytes::from(" to make it fast"),
		);

		let mut buf = [0u8; 256];
		let mut serializer = serde_json_core::ser::Serializer::new(&mut buf);
		serialize(&foo, &mut serializer).unwrap();
		let n = serializer.end();
		let result = unsafe { core::str::from_utf8_unchecked(&buf[..n]) };

		let expected = r#""0x65746867656e20646f6573206e6f206865617020616c6c6f636174696f6e20746f206d616b652069742066617374""#;

		assert_eq!(expected, result);
	}
}
