use super::Error;
#[cfg(feature = "faster-hex")]
use faster_hex::{hex_check, hex_decode_unchecked};

#[inline(always)]
const fn nybl(b: u8) -> u8 {
    let b = b as i16;
    (-1i16
        + ((((0x2fi16 - b) & (b - 0x3a)) >> 8) & (b - 47))
        + ((((0x40i16 - b) & (b - 0x47)) >> 8) & (b - 54))
        + ((((0x60i16 - b) & (b - 0x67)) >> 8) & (b - 86))) as u8
}

/// Internal function to decode hex bytes at compile time
#[doc(hidden)]
#[inline(always)]
pub const fn const_decode<const V: usize>(src: &[u8]) -> [u8; V] {
    assert!(src.len() % 2 == 0, "Odd number of hex characters");
    assert!(src.len() <= V << 1, "Incufficient capacity for the source");

    let pad = V - (src.len() >> 1);
    let (mut dst, mut i) = ([0u8; V], 0usize);

    while i < src.len() {
        let nyb = nybl(src[i]);
        assert!(nyb < 16, "Invalid hex character");

        dst[pad + (i >> 1)] |= nyb << (1 - (i % 2)) * 4;
        i += 1;
    }

    dst
}

/// Decode hex encoded string
///
/// Writes decoded bytes into the destination buffer, returning error if buffer
/// capacity is not enough to fit the entire load, if source has an odd length
/// or if invalid hex characters are encountered.
///
/// ```
/// use ethgen::hex;
///
///
/// let mut buf = [0u8; 8];
/// hex::decode("000065746867656e", &mut buf).unwrap();
///
/// assert_eq!(b"\0\0ethgen", &buf);
/// ```
#[inline]
pub fn decode(src: impl AsRef<[u8]>, dst: &mut [u8]) -> Result<usize, Error> {
    let src = src.as_ref();

    if src.len() % 2 != 0 {
        return Err(Error::InvalidLength);
    }

    let pad = match dst.len().checked_sub(src.len() >> 1) {
        Some(pad) => pad,
        None => {
            return Err(Error::BufferOverflow);
        }
    };

    #[cfg(not(feature = "faster-hex"))]
    {
        let mut err = 0usize;
        src.chunks(2)
            .zip(dst.iter_mut().skip(pad))
            .for_each(|(src, dst)| {
                let (a, b) = (nybl(src[0]), nybl(src[1]));
                *dst = a << 4 | b;
                err += (a > 15 || b > 15) as usize;
            });

        if err > 0 {
            return Err(Error::InvalidEncoding);
        }
    }

    #[cfg(feature = "faster-hex")]
    {
        if !hex_check(src) {
            return Err(Error::InvalidEncoding);
        }

        hex_decode_unchecked(src, &mut dst[pad..])
    }

    Ok(pad)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_const_decode() {
        const DUMMY: [u8; 8] = const_decode(b"000065746867656e");
        assert_eq!(b"\0\0ethgen", &DUMMY);
    }

    #[test]
    fn test_const_decode_pad() {
        const DUMMY: [u8; 8] = const_decode(b"0face342");
        const RES: u64 = u64::from_be_bytes(DUMMY);
        assert_eq!(262988610u64, RES);
    }

    #[test]
    fn test_decode() {
        let mut buf = [0u8; 8];
        decode("000065746867656e", &mut buf).unwrap();
        assert_eq!(b"\0\0ethgen", &buf);
    }

    #[test]
    fn test_decode_padding() {
        let mut buf = [0u8; 8];
        decode("fa", &mut buf).unwrap();
        let res = u64::from_be_bytes(buf);

        assert_eq!(250u64, res);
    }

    #[test]
    #[should_panic]
    fn test_decode_overflow() {
        let mut buf = [0u8; 4];
        decode("000065746867656e", &mut buf).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_decode_invalid() {
        let mut buf = [0u8; 8];
        decode("non-hex", &mut buf).unwrap();
    }
}
