//! Hex (base16 [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-8)) encoding
//!
//! Contains low level function to encode / decode hex nibbles into bytes.

#[cfg(feature = "faster-hex")]
use faster_hex;

#[inline(always)]
const fn nybl_lower(b: u8) -> u8 {
    let b = b as i16;
    (b + 0x30 + (((0x9 - b) >> 8) & (0x61i16 - 0x3a))) as u8
}

#[inline(always)]
const fn nybl_upper(b: u8) -> u8 {
    let b = b as i16;
    (b + 0x30 + (((0x9 - b) >> 8) & (0x41i16 - 0x3a))) as u8
}

macro_rules! impl_encode {
    (   $(#[$meta:meta])*
        $name:ident => $fast:ident, $fallback:ident
    ) => {
        $(#[$meta])*
        pub fn $name(src: &[u8], dst: &mut [u8]) -> usize {
            let pad = match dst.len().checked_sub(src.len() << 1) {
                Some(pad) => pad,
                None => panic!("incufficient buffer length for hex encode"),
            };

            #[cfg(feature = "faster-hex")]
            unsafe {
                // Safe as all necessary length checks are passed
                faster_hex::$fast(src, &mut dst[pad..]).unwrap_unchecked();
            }

            #[cfg(not(feature = "faster-hex"))]
            {
                for (i, nybl) in src.iter().enumerate() {
                    dst[pad + (i << 1)    ] = $fallback(nybl >>   4);
                    dst[pad + (i << 1) + 1] = $fallback(nybl & 0x0f);
                }
            }

            pad
        }
    }
}

impl_encode! {
    /// Hex encode byte slice
    ///
    /// Fills the destination slice with hex nibbles from the higher end,
    /// resulting in leading-zero padded hex string. Return the number of
    /// leading zeroes (not the amount of bytes written).
    ///
    /// ## Panics
    ///
    /// - if buffer capacity is incufficient to containt the entire encoding
    ///   product;
    ///
    /// <br>
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ethgen::hex;
    ///
    ///
    ///
    /// let msg = b"ethgen rules";
    ///
    /// let mut buf = [0u8; 32];
    /// let pad = hex::encode(msg, &mut buf);
    ///
    /// assert_eq!(
    ///	    b"65746867656e2072756c6573",
    ///	    &buf[pad..]
    /// );
    /// ```
    encode => hex_encode, nybl_lower
}

impl_encode! {
    /// Hex encode byte slice (upper case)
    ///
    /// Fills the destination slice with upper-case hex nibbles from the higher
    /// end, resulting in leading-zero padded hex string. Return the number of
    /// leading zeroes (not the amount of bytes written).
    ///
    /// ## Panics
    ///
    /// - if buffer capacity is incufficient to containt the entire encoding
    ///   product;
    ///
    /// <br>
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ethgen::hex;
    ///
    ///
    ///
    /// let msg = b"ethgen rules";
    ///
    /// let mut buf = [0u8; 32];
    /// let pad = hex::encode_upper(msg, &mut buf);
    /// assert_eq!(
    ///	    b"65746867656E2072756C6573",
    ///	    &buf[pad..]
    /// );
    /// ```
    encode_upper => hex_encode_upper, nybl_upper
}

/// Encode bytes to lowercase hex nibbles compilation time
///
///
/// ## Panics
///
/// - if requested array length is odd;
/// - if resulting array doesn't have enough capacity;
///
/// <br>
///
/// ## Example
///
/// ```rust
/// use ethgen::hex;
///
///
/// const MSG: [u8; 24] = hex::const_encode(b"ethgen rules");
/// assert_eq!(b"65746867656e2072756c6573", &MSG);
/// ```
#[inline(always)]
pub const fn const_encode<const U: usize>(src: &[u8]) -> [u8; U] {
    assert!(U % 2 == 0, "Unable to encode to odd number of hex nibbles");
    assert!(src.len() << 1 <= U, "Incufficient capacity to hex encode");

    let pad = U - (src.len() << 1);
    let (mut dst, mut i) = ([0u8; U], 0usize);

    while i < src.len() {
        dst[pad + (i << 1)] = nybl_lower(src[i] >> 4);
        dst[pad + (i << 1) + 1] = nybl_lower(src[i] & 0x0f);

        i += 1;
    }

    dst
}

#[inline(always)]
const fn decode_nybl(b: u8) -> u8 {
    let b = b as i16;
    (-1i16
        + ((((0x2fi16 - b) & (b - 0x3a)) >> 8) & (b - 47))
        + ((((0x40i16 - b) & (b - 0x47)) >> 8) & (b - 54))
        + ((((0x60i16 - b) & (b - 0x67)) >> 8) & (b - 86))) as u8
}

/// Decode hex nibbles to bytes
///
/// Writes decoded bytes into byte buffer from the higher end resulting in
/// leading-zero bytes prepended Big Endian byte representation of the hex
/// string.
///
/// ## Panics
///
/// - if source length is odd;
/// - if buffer capacity is incufficient to contain decoded bytes;
/// - if invalid hex characters encountered;
///
/// <br>
///
/// ## Example
///
/// ```rust
/// use ethgen::hex;
///
///
///
/// let mut buf = [0u8; 8];
/// hex::decode(b"000065746867656e", &mut buf);
///
/// assert_eq!(b"\0\0ethgen", &buf);
/// ```
pub fn decode(src: &[u8], dst: &mut [u8]) -> usize {
    assert!(src.len() % 2 == 0, "Invalid odd length of hex input");

    let pad = match dst.len().checked_sub(src.len() >> 1) {
        Some(pad) => pad,
        None => panic!("incufficient buffer length for hex decode"),
    };

    #[cfg(feature = "faster-hex")]
    {
        if !faster_hex::hex_check(src) {
            panic!("invalid hex encoding");
        }

        faster_hex::hex_decode_unchecked(src, &mut dst[pad..]);
    }

    #[cfg(not(feautre = "faster-hex"))]
    {
        src.chunks(2)
            .zip(dst.iter_mut().skip(pad))
            .for_each(|(src, dst)| {
                let (a, b) = (decode_nybl(src[0]), decode_nybl(src[1]));
                assert!(a < 16 && b < 16, "invalid hex encoding");
                *dst = a << 4 | b;
            });
    }

    pad
}

/// Decode hex nibbles to bytes constant (compile time)
///
/// Writes decoded bytes to byte array from the high end, resulting in a zero
/// padded byte representation of the hex nybls.
///
/// ## Panics
///
/// - if hex nybble slice has odd length;
/// - if resulting array doesn't have enough capacity;
/// - if invalid hex character presented;
///
/// <br>
///
/// ## Example
///
/// ```
/// use ethgen::hex;
///
///
/// const ETHGEN: [u8; 8] = hex::const_decode(b"000065746867656e");
/// assert_eq!(b"\0\0ethgen", &ETHGEN);
///
///
/// const PADDED: [u8; 8] = hex::const_decode(b"0face342");
/// const VAR: u64 = u64::from_be_bytes(PADDED);
/// assert_eq!(262988610u64, VAR);
/// ```
#[inline(always)]
pub const fn const_decode<const V: usize>(src: &[u8]) -> [u8; V] {
    assert!(src.len() % 2 == 0, "Odd number of hex characters");
    assert!(src.len() <= V << 1, "Incufficient capacity for hex decode");

    let pad = V - (src.len() >> 1);
    let (mut dst, mut i) = ([0u8; V], 0usize);

    while i < src.len() {
        let nyb = decode_nybl(src[i]);
        assert!(nyb < 16, "Invalid hex character");

        dst[pad + (i >> 1)] |= nyb << (1 - (i % 2)) * 4;
        i += 1;
    }

    dst
}

/// Decode hex string compilation time
///
/// ## Panics
///
/// - if hex string has odd number of characters;
/// - if resulting array doesn't have enough capacity;
/// - if invalid hex character presented in the string;
///
/// <br>
///
/// ## Example
///
/// ```
/// use ethgen::unhex;
///
///
/// const FOO: [u8; 3] = unhex!("666f6f");
/// assert_eq!(b"foo", &FOO);
///
/// let bar = unhex!("626172");
/// assert_eq!(b"bar", &bar);
/// ```
#[macro_export]
macro_rules! unhex {
    ($s:literal) => {{
        const SRC: &'static [u8] = $s.as_bytes();
        $crate::hex::const_decode(SRC)
    }};
}
