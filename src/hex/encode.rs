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

macro_rules! impl_encode {
    (	$(#[$meta:meta])*
	$name:ident => $fn:ident, $fallback:ident
    ) => {
	$(#[$meta])*
	pub fn $name(src: &[u8], dst: &mut [u8]) -> Result<usize, Error>
	{
	    let pad = match dst.len().checked_sub(src.len() << 1) {
		Some(pad) => pad,
		None => {
		    return Err(Error::BufferOverflow);
		}
	    };

	    #[cfg(not(feature = "faster-hex"))]
	    {
		for (i, byte) in src.iter().enumerate() {
		    dst[pad + (i << 1)] = $fallback(byte >> 4);
		    dst[pad + (i << 1) + 1] = $fallback(byte & 0x0f);
		}
	    }

	    #[cfg(feature = "faster-hex")]
	    {
		// Safe as all necessary length checks are already done.
		unsafe {
		    faster_hex::$fn(src, &mut dst[pad..]).unwrap_unchecked();
		}
	    }

	    Ok(pad)
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
    encode => hex_encode, nybl_lower
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
    encode_upper => hex_encode_upper, nybl_upper
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
