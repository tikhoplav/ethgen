use core::fmt;

#[derive(PartialEq)]
pub enum Error {
	InvalidLength,
	InvalidEncoding,
}

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Error::InvalidLength => {
				write!(f, "Buffer length does not correspond source length")
			},
			Error::InvalidEncoding => {
				write!(f, "Invalid hex encoding")
			}
		}
	}
}

#[inline(always)]
pub(super) const fn nybl(b: u8) -> u8 {
	let b = b as i16;
	(-1i16
		+ ((((0x2fi16 - b) & (b - 0x3a)) >> 8) & (b - 47))
		+ ((((0x40i16 - b) & (b - 0x47)) >> 8) & (b - 54))
		+ ((((0x60i16 - b) & (b - 0x67)) >> 8) & (b - 86))
	) as u8
}

/// Decode hex encoded string
///
/// Requires the `dst` length to match exactly two times of the `src` length,
/// returns `InvalidLength` error otherwise. The `0x` prefix must be stripped.
///
/// ```
/// use ethgen::hex;
///
///
/// let mut buf = [0u8; 8];
///	hex::decode("000065746867656e", &mut buf).unwrap();
///
///	assert_eq!(b"\0\0ethgen", &buf);
/// ```
pub fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<(), Error> {
	let src = src.as_ref();
	if src.len() != dst.len() << 1 {
		return Err(Error::InvalidLength);
	}

	let mut err: usize = 0;
	src.chunks(2).zip(dst.iter_mut()).for_each(|(src, dst)| {
		let (a, b) = (nybl(src[0]), nybl(src[1]));
		*dst = a << 4 | b;
		err += (a > 15 || b > 15) as usize;
	});

	match err {
		0 => Ok(()),
		_ => Err(Error::InvalidEncoding),
	}
}

/// Internal function to decode hex bytes at compile time
#[doc(hidden)]
#[inline(always)]
pub const fn const_decode<const V: usize>(src: &[u8]) -> [u8; V] {
	assert!(src.len() == V << 1, "Specified length doesn't match provided source");
	let mut dst = [0u8; V];

	let mut i = 0usize;
	while i < src.len() {
		let (a, b) = (nybl(src[i]), nybl(src[i + 1]));
		assert!(a < 16 && b < 16, "Invalid hex character");
		dst[i >> 1] = a << 4 | b;
		i += 2;
	}

	dst
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_decode() {
		let mut buf = [0u8; 8];
		decode("000065746867656e", &mut buf).unwrap();
		assert_eq!(b"\0\0ethgen", &buf);
	}

	#[test]
	#[should_panic]
	fn test_decode_overflow() {
		let mut buf = [0u8; 4];
		decode("000065746867656e", &mut buf).unwrap();
	}

	#[test]
	#[should_panic]
	fn test_decode_underflow() {
		let mut buf = [0u8; 16];
		decode("000065746867656e", &mut buf).unwrap();
	}

	#[test]
	#[should_panic]
	fn test_decode_invalid() {
		let mut buf = [0u8; 8];
		decode("non-hex", &mut buf).unwrap();
	}

	const DUMMY: [u8; 8] = const_decode(b"000065746867656e");

	#[test]
	fn test_const_decode() {
		assert_eq!(b"\0\0ethgen", &DUMMY);
	}
}
