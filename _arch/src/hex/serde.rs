use super::encode::encode;
use crate::Encode;
use core::{
    ops::{Add, Mul},
    str,
};
use generic_array::{
    typenum::{Prod, Sum, U2},
    ArrayLength, GenericArray,
};
use serde::Serializer;

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
    // Encode's `into_bytes` can panic when buffer length is incufficient to
    // contain the whole strucutre, however since the buffer is created using
    // the ByteSize of the trait, the buffer capacity is guarantied to fit the
    // structure bytes.
    data.into_bytes(&mut buf);

    // Allocate the buffer 2*n + 2 bytes to contain hex encoded structure
    let mut hex = GenericArray::<u8, HexPrefSize<E>>::default();

    // Add hex prefix `0x`
    hex[0] = 48u8;
    hex[1] = 120u8;

    // Shall not panic, as the buffer is allocated corresponding to ByteSize
    encode(&buf, &mut hex[2..]).unwrap();

    // Safety: bytes returned from Encode::into_hex are valid ascii cahracters
    let s = unsafe { str::from_utf8_unchecked(&hex) };

    serializer.serialize_str(&s)
}

#[cfg(test)]
mod test {
    use super::*;
    use crypto_bigint::{Encoding, U256};
    use generic_array::typenum::U96;

    #[derive(Debug, PartialEq)]
    struct UniswapV2Reserves {
        pub reserve0: U256,
        pub reserve1: U256,
        pub block_timestamp_last: u32,
    }

    impl Encode for UniswapV2Reserves {
        // When encoded for the JSON RPC body takes exactly 3 slots,
        // which is 32 * 3 + one byte for the prefix, which is stripped
        // when deserialized.
        type ByteSize = U96;

        fn into_bytes(&self, bytes: &mut [u8]) {
            self.reserve0
                .to_be_bytes()
                .iter()
                .chain(self.reserve1.to_be_bytes().iter())
                .chain(core::iter::repeat(&0u8).take(28))
                .chain(self.block_timestamp_last.to_be_bytes().iter())
                .zip(bytes.iter_mut())
                .for_each(|(src, dst)| *dst = *src);
        }

        fn from_bytes(src: &[u8]) -> Self {
            let reserve0 = U256::from_be_slice(&src[0..32]);
            let reserve1 = U256::from_be_slice(&src[32..64]);
            let block_timestamp_last = u32::from_be_bytes(src[92..96].try_into().unwrap());

            Self {
                reserve0,
                reserve1,
                block_timestamp_last,
            }
        }
    }

    #[test]
    fn test_serialize() {
        let foo = UniswapV2Reserves {
            reserve0: U256::from_u64(872887175),
            reserve1: U256::from_u64(544087615712082577),
            block_timestamp_last: 1696744694u32,
        };

        let mut buf = [0u8; 512];
        let mut serializer = serde_json_core::ser::Serializer::new(&mut buf);
        serialize(foo, &mut serializer).unwrap();
        let n = serializer.end();
        let result = unsafe { core::str::from_utf8_unchecked(&buf[..n]) };

        let expected = r#""0x0000000000000000000000000000000000000000000000000000000034073387000000000000000000000000000000000000000000000000078cfccdc5353a9100000000000000000000000000000000000000000000000000000000652244f6""#;

        assert_eq!(expected, result);
    }
}
