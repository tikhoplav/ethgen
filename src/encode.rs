use core::fmt;
pub use generic_array::typenum;

/// Encoding / Decoding error
#[derive(PartialEq)]
pub enum Error {
    /// When a strcuture is encoded / decoded the byte buffer is used to write
    /// into / read from. In case if buffer size doesn't match the required for
    /// the data structure, this error is returned.
    BufferOverflow,

    /// When a structure can't be restored from the encoded byte sequence, this
    /// error is returned.
    InvalidEncoding,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BufferOverflow => {
                write!(f, "{}", "insufficient buffer length")
            }
            Self::InvalidEncoding => {
                write!(f, "{}", "invalid encoding")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

/// Type representable as a byte array.
///
/// Big-Endian byte representations are assumed by default.
pub trait Encode: Sized {
    /// Size of a data structure in bytes when encoded
    ///
    /// Used to determine the minimum length of the byte buffer required to
    /// contain the encoded data structure.
    type ByteSize: typenum::Unsigned;

    /// Copy the structure byte representation (Big endian) into a byte slice
    fn into_bytes(&self, bytes: &mut [u8]) -> Result<usize, Error>;

    /// Retreive the data structure using bytes (Big Endian) from the slice
    fn from_bytes(src: &[u8]) -> Result<Self, Error>;
}

#[cfg(test)]
mod test {
    use super::*;
    use crypto_bigint::{Encoding, U256};
    use generic_array::typenum::Unsigned;
    use generic_array::GenericArray;

    #[derive(Debug, PartialEq)]
    struct Foo(U256, GenericArray<u8, typenum::U24>, u64);

    impl Encode for Foo {
        type ByteSize = typenum::U64;

        fn into_bytes(&self, bytes: &mut [u8]) -> Result<usize, Error> {
            let a = self.0.to_be_bytes();
            let b = &self.1;
            let c = self.2.to_be_bytes();

            let len = a.len() + b.len() + c.len();
            if len > bytes.len() {
                return Err(Error::BufferOverflow);
            }

            a.iter()
                .chain(b.iter())
                .chain(c.iter())
                .zip(bytes.iter_mut())
                .for_each(|(src, dst)| *dst = *src);

            Ok(len)
        }

        fn from_bytes(src: &[u8]) -> Result<Self, Error> {
            if src.len() < Self::ByteSize::to_usize() {
                return Err(Error::InvalidEncoding);
            }

            let a = U256::from_be_slice(&src[..32]);
            let b = GenericArray::<u8, typenum::U24>::from_slice(&src[32..56]);
            let c = u64::from_be_bytes(src[56..64].try_into().unwrap());

            Ok(Self(a, *b, c))
        }
    }

    #[test]
    fn compound_struct_encode() {
        let foo = Foo(U256::ONE, (*b"ethgen operates w/ bytes").into(), 42);

        // Allocate a buffer using the length provided via the trait
        let mut buf = GenericArray::<u8, <Foo as Encode>::ByteSize>::default();
        foo.into_bytes(&mut buf).unwrap();

        let bar = Foo::from_bytes(&buf).unwrap();

        assert_eq!(foo, bar);
    }
}
