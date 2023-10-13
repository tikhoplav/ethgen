pub use generic_array::typenum;
use generic_array::{ArrayLength, GenericArray};

// TODO:
// Technically, everything that can be presented as a continuous array of
// bytes can be encoded / decoded into hex. So the trait can provide this
// functionallity as default as soon as rules to byte silce conversion are
// set.
//
// Check EVM encoding rules, probably some of those can be enbeded in the 
// trait (slots e.t.c).
//
// Check how GenericArray and crypto_bigint does the byte representation,
// and probably generalize this for all the Encode supporting structures.
//
// Structures are serialize / deserialize as a whole when encoded for EVM.
// The only thing that stands out is bigInt numbers used for `value` and gas.
// This can be addressed by using the custom serde macro ("compact hex"), the
// rest never going to need this, as EVM compatible hex encoded values are 
// always take up integer amount of slots

/// Type representable as a byte array.
///
/// Big-Endian byte representations are assumed by default.
pub trait Encode: Sized {
    /// Size of a data structure in bytes when encoded
    ///
    /// Used to determine the minimum length of the byte buffer required to
    /// contain the encoded data structure.
    type ByteSize: typenum::Unsigned;

    /// Allocate empty buffer to contain the structure bytes
    fn buffer() -> GenericArray<u8, <Self as Encode>::ByteSize>
    where
        <Self as Encode>::ByteSize: ArrayLength,
    {
        GenericArray::<u8, <Self as Encode>::ByteSize>::default()
    }

    /// Copy the byte repr (Big endian) into a byte slice
    ///
    /// The amount of bytes written should exactly match the ByteSize. Allowed
    /// to panic if buffer length is incufficient, as buffers should be created
    /// using ByteSize.
    fn into_bytes(&self, bytes: &mut [u8]);

    /// Retreive the structure from byte slice (Big Endian)
    ///
    /// The amount of bytes "consumed" should exactly match the ByteSize. Can
    /// panic, as buffers should be created using ByteSize.
    fn from_bytes(src: &[u8]) -> Self;
}

#[cfg(test)]
mod test {
    use super::*;
    use crypto_bigint::{Encoding, U256};
    use generic_array::GenericArray;

    #[derive(Debug, PartialEq)]
    struct Foo(U256, GenericArray<u8, typenum::U24>, u64);

    impl Encode for Foo {
        type ByteSize = typenum::U64;

        fn into_bytes(&self, bytes: &mut [u8]) {
            let a = self.0.to_be_bytes();
            let b = &self.1;
            let c = self.2.to_be_bytes();

            a.iter()
                .chain(b.iter())
                .chain(c.iter())
                .zip(bytes.iter_mut())
                .for_each(|(src, dst)| *dst = *src);
        }

        fn from_bytes(src: &[u8]) -> Self {
            let a = U256::from_be_slice(&src[..32]);
            let b = GenericArray::<u8, typenum::U24>::from_slice(&src[32..56]);
            let c = u64::from_be_bytes(src[56..64].try_into().unwrap());

            Self(a, *b, c)
        }
    }

    #[test]
    fn compound_struct_encode() {
        let foo = Foo(U256::ONE, (*b"ethgen operates w/ bytes").into(), 42);

        let mut buf = Foo::buffer();
        foo.into_bytes(&mut buf);

        let bar = Foo::from_bytes(&buf);

        assert_eq!(foo, bar);
    }
}
