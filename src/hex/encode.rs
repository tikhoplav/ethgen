use super::Error;
#[cfg(feature = "faster-hex")]
use faster_hex;

#[inline(always)]
#[cfg(not(feature = "faster-hex"))]
fn nybl_lower(b: u8) -> u8 {
    let b = b as i16;
    (b + 0x30 + (((0x9 - b) >> 8) & (0x61i16 - 0x3a))) as u8
}

#[inline(always)]
#[cfg(not(feature = "faster-hex"))]
fn nybl_upper(b: u8) -> u8 {
    let b = b as i16;
    (b + 0x30 + (((0x9 - b) >> 8) & (0x41i16 - 0x3a))) as u8
}

macro_rules! impl_encode_unchecked {
    (	$(#[$meta:meta])*
	$name:ident => $fn:ident, $fallback:ident
    ) => {
	$(#[$meta])*
	pub fn $name(src: &[u8], dst: &mut [u8]) -> usize {
	    // Panic if underflow
	    let pad = dst.len() - (src.len() << 1);

	    #[cfg(not(feature = "faster-hex"))]
	    {
		// Panic if index >= length
		for (i, byte) in src.iter().enumerate() {
		    dst[pad + (i << 1)]     = $fallback(byte >>   4);
		    dst[pad + (i << 1) + 1] = $fallback(byte & 0x0f);
		}
	    }

	    #[cfg(feature = "faster-hex")]
	    {
		// Panic if dst length < src length * 2
		faster_hex::$fn(src, &mut dst[pad..]).unwrap();
	    }

	    pad
	}
    }
}

impl_encode_unchecked! {
    /// Encode to hex
    encode_unchecked => hex_encode, nybl_lower
}

impl_encode_unchecked! {
    /// Encode to uppercase hex
    encode_upper_unchecked => hex_encode_upper, nybl_upper
}

macro_rules! impl_encode {
    (	$(#[$meta:meta])*
	$name:ident => $fn:ident
    ) => {
	$(#[$meta])*
	pub fn $name(src: &[u8], dst: &mut [u8]) -> Result<usize, Error> {
	    if src.len() % 2 != 0 {
		return Err(Error::InvalidLength);
	    }

	    if src.len() << 1 > dst.len() {
		return Err(Error::BufferOverflow);
	    }

	    Ok($fn(src, dst))
	}
    }
}

impl_encode! {
    /// Encode to hex
    ///
    /// Writes encoded hex bytes to the end of the provided buffer returning
    /// the number of leading zeroes.
    ///
    /// ```
    /// use ethgen::hex;
    ///
    ///
    ///
    /// let msg = b"ethgen rules";
    ///
    /// let mut buf = [0u8; 32];
    /// let pad = hex::encode(msg, &mut buf).unwrap();
    ///
    /// assert_eq!(
    ///	    b"65746867656e2072756c6573",
    ///	    &buf[pad..]
    /// );
    /// ```
    #[inline]
    encode => encode_unchecked
}

impl_encode! {
    /// Encode to upper hex
    ///
    /// Writes encoded upper hex bytes to the end of the buffer returning the
    /// number of leading zeroes.
    ///
    /// ```
    /// use ethgen::hex;
    ///
    ///
    /// let msg = b"any some";
    ///
    /// let mut buf = [0u8; 32];
    /// let pad = hex::encode_upper(msg, &mut buf).unwrap();
    ///
    /// assert_eq!(
    ///	    b"616E7920736F6D65",
    ///	    &buf[pad..]
    /// );
    /// ```
    encode_upper => encode_upper_unchecked
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_lower() {
        let msg = b"ethgen rules";

        let mut buf = [0u8; 32];
        let pad = encode(msg, &mut buf).unwrap();

        assert_eq!(b"65746867656e2072756c6573", &buf[pad..]);
    }

    #[test]
    fn test_encode_upper() {
        let msg = b"any some";

        let mut buf = [0u8; 32];
        let pad = encode_upper(msg, &mut buf).unwrap();

        assert_eq!(b"616E7920736F6D65", &buf[pad..]);
    }
}
