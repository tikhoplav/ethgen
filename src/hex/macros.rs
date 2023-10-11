/// Decode hex string in const time
///
/// Can be used to decode a hex string to bytes during compilation.
///
/// ### Panics
///
/// - if hex string has odd number of characters;
/// - if resulting array doesn't have enough capacity;
/// - if invalid hex character presented in the string;
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

#[cfg(test)]
mod test {
    const FOO: [u8; 4] = unhex!("face2bad");

    #[test]
    fn test_unhex() {
        assert_eq!([0xfau8, 0xce, 0x2b, 0xad], FOO);

        let bar = unhex!("def4dead");
        assert_eq!([0xdeu8, 0xf4, 0xde, 0xad], bar);
    }
}
