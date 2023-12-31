use generic_array::typenum::*;
use generic_array::{sequence::GenericSequence, ArrayLength, GenericArray};

pub mod hex;

/// Stack-allocated byte array with a constant length
///
/// This structure is used in place of regular `[u8; N]` byte arrays for
/// several reasons:
/// - `GenericArray` can be cast to the underlying byte array with minimal
///   to no overhead;
/// - Representing a byte array in this way allows for the calculation of its
///   length during compilation time using types like `Sum`, `Prod`, and others
///   from `typenum` crate;
///
/// Achieving the same using generic constants is not possible with current
/// stable Rust release (1.73.0):
/// <https://doc.rust-lang.org/beta/reference/items/generics.html#const-generics>
///
/// In general, any entity that can be used as input to or received from an EVM
/// must be encoded as a byte array, according to ABI for EVM communication.
/// You can refer to the Solidity documentation for the ABI specification:
/// <https://docs.soliditylang.org/en/latest/abi-spec.html>
///
/// The RLP encoding is used for transactions both to provide an input for the
/// hashing function during signing and to form an input for the exectution of
/// a signed transaction. The exact implementation depends on the transaction
/// version described by corresponding EIP. The description of RPL encoding:
/// <https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp>
///
/// Apart from described above there is a number of special cases, such as:
/// block number for `eth_call`, transaction value and gas parameters for the
/// `eth_sign` (would be depricated soon) JSON RPC calls, which doesn't have
/// some encoding specification. The `geth` encoding implementation should be
/// referred in such cases.
///
/// Regardless the encoding standart any data should be encoded into the hex
/// string in order to be sent over JSON RPC (or GraphQl). The `Bytes` acts as
/// an adaptor for such data providing JSON PRC serialization / deserialization
/// ability.
pub type Bytes<N> = GenericArray<u8, N>;

/// Encode into hex nibbles with `0x` prefix
///     
/// The resulting string stores Big Endian hex representation of the array,
/// prefixed with `0x` and leading zeroes, that so the resulting string has
/// exactly 2N + 2 characters.
pub fn into_hex<N>(src: impl Into<Bytes<N>>) -> Bytes<Sum<Prod<N, U2>, U2>>
where
    N: ArrayLength,
    N: core::ops::Mul<U2>,
    Prod<N, U2>: core::ops::Add<U2>,
    Sum<Prod<N, U2>, U2>: ArrayLength,
{
    let bytes: Bytes<N> = src.into();
    let mut nybls: GenericArray<u8, Sum<Prod<N, U2>, U2>> = GenericArray::generate(|_| 48u8);

    nybls[1] = 120u8;

    hex::encode(&bytes, &mut nybls);
    nybls
}

/// Decode a byte array from hex nibbles
///
/// Fills decoded bytes from the Big Endian hex string from the higher end,
/// resulting in array being filled with leading zeroes in case of source
/// string underflow.
///
/// *Important*: In case of source string overflow, the most signigicant
/// bytes are ommitted, effectively acting as `as` casting for primitives.
/// In case if preserving the data integrity is required, consider adding
/// an explicit length check, using the const `len` method:
///
/// ```rust
/// use ethgen::{Bytes, typenum::U8};
///
///
/// let bytes: Bytes<U8> = [0u8; 8].into();
/// assert_eq!(bytes.len(), 8);
/// ```
pub fn from_hex<T, N>(src: &str) -> T
where
    N: ArrayLength,
    T: From<Bytes<N>>,
{
    let src = src.as_bytes();

    // Reduce the `0x` prefix if presented, it is required to be done to check
    // if the resulting string can fit into the buffer.
    let src = match src[0] == 48u8 && src[1] == 120u8 {
        true => &src[2..],
        false => src,
    };

    // The padding is applied to the source string to insure that the allocated
    // byte buffer will fit the decoding product. In case if the source require
    // larger buffer, the overflowing nybls are stripped (form the lower end).
    let pad = match src.len().checked_sub(1 + (N::USIZE << 1)) {
        Some(pad) => pad,
        // No padding required, the whole string when decoded will fit
        // into the byte representation of the data structure.
        None => 0,
    };

    let mut bytes = Bytes::<N>::default();
    hex::decode(&src[pad..], &mut bytes);

    bytes.into()
}

pub mod as_hex {
    //! Hex string serialization / deserialization
    //!
    //! The module contains `serialize` and `deserialize` functions and can be
    //! used with `serde` macros to enable data structure serialization /
    //! deserialization from JSON hex encoded string. Require a strucute to
    //! only implement `From<Bytes<N>>` and `Into<Bytes<N>>` traits. The byte
    //! representation of a struct for hex encoding (like RLP or ABI) should be
    //! picked corresponding to the role of a data struct in the JSON RPC call.
    //!
    //! ``` rust
    //! use serde::{Serialize, Deserialize};
    //! use ethgen::{Bytes, typenum::U4, as_hex};
    //!
    //! #[derive(Debug, PartialEq)]
    //! struct Foo(u32);
    //!
    //! impl From<Bytes<U4>> for Foo {
    //!     fn from(value: Bytes<U4>) -> Self {
    //!         Self(u32::from_be_bytes(value.into()))
    //!     }
    //! }
    //!
    //! impl Into<Bytes<U4>> for &Foo {
    //!     fn into(self) -> Bytes<U4> {
    //!         Bytes::from_array(self.0.to_be_bytes())
    //!     }
    //! }
    //!
    //!
    //!
    //! #[derive(Debug, Serialize, Deserialize, PartialEq)]
    //! struct Bar {
    //!     #[serde(with = "as_hex")]
    //!     foo: Foo
    //! }
    //!
    //!
    //!
    //! let bar = Bar{ foo: Foo(262988610u32) };
    //!
    //! let mut buf = [0u8; 256];
    //! let n = serde_json_core::to_slice(&bar, &mut buf).unwrap();
    //! let json = unsafe { core::str::from_utf8_unchecked(&buf[..n]) };
    //!
    //!
    //! let (result, _): (Bar, usize) = serde_json_core::from_str(json).unwrap();
    //!
    //!
    //! assert_eq!(bar, result);
    //! ```

    use super::{from_hex, into_hex, Bytes};
    use core::fmt;
    use core::marker::PhantomData;
    use generic_array::{
        typenum::{Prod, Sum, U2},
        ArrayLength,
    };
    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serializer};

    /// Serialize a data struct into `0x` prefixed hex string.
    ///
    /// Serialize a data struct as a JSON string containing `0x` prefixed
    /// representation of data bytes. The length of the string and leading-zero
    /// padding is determined by it's `Into<Bytes<N>>` trait implementation.
    ///
    /// ``` rust
    /// use serde::Serialize;
    /// use ethgen::{Bytes, typenum::U4, as_hex};
    ///
    ///
    ///
    /// struct Foo(u32);
    ///
    /// impl From<&Foo> for Bytes<U4> {
    ///    fn from(value: &Foo) -> Self {
    ///        Self::from_array(value.0.to_be_bytes())
    ///    }
    /// }
    ///
    ///
    ///
    /// #[derive(Serialize)]
    /// struct Bar {
    ///     #[serde(with = "as_hex")]
    ///     foo: Foo,
    /// }
    ///
    ///
    ///
    /// let bar = Bar{ foo: Foo(262988610u32) };
    ///
    ///
    /// let mut buf = [0u8; 256];
    /// let n = serde_json_core::to_slice(&bar, &mut buf).unwrap();
    /// let result = unsafe { core::str::from_utf8_unchecked(&buf[..n]) };
    ///
    ///
    /// let expected = r#"{"foo":"0x0face342"}"#;
    /// assert_eq!(expected, result);
    /// ```
    pub fn serialize<S, T, N>(data: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        N: ArrayLength,
        T: Into<Bytes<N>>,
        N: core::ops::Mul<U2>,
        Prod<N, U2>: core::ops::Add<U2>,
        Sum<Prod<N, U2>, U2>: ArrayLength,
    {
        let nybls = into_hex(data);

        // Safe since `into_hex` can return only valid ascii characters
        let s = unsafe { core::str::from_utf8_unchecked(&nybls) };

        serializer.serialize_str(s)
    }

    /// Deserialize a data struct from hex encoded string
    ///
    /// Deserialize a data struct from a JSON string containting hex encoded
    /// byte representation of the string. Supports lower, upper and mixed case
    /// hex strings. `0x` preffix is stripped automatically if presented.
    ///
    /// Before data structure is finalized it's decoded bytes are written into
    /// a byte buffer, which length is determined by it's `From<Bytes<N>>`
    /// implementation. All encodings are assumed to be Big Endian. In case if
    /// hex string exceeds expected length only the expected amount of bytes is
    /// taken from the least significant endm which makes it applicable only
    /// for data structures with **predetermined** size.
    ///
    /// ```rust
    /// use serde::Deserialize;
    /// use ethgen::{Bytes, typenum::U4, as_hex};
    ///
    ///
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Foo(u32);
    ///
    /// impl From<Bytes<U4>> for Foo {
    ///     fn from(value: Bytes<U4>) -> Self {
    ///         Self(u32::from_be_bytes(value.into()))
    ///     }
    /// }
    ///
    ///
    ///
    /// #[derive(Debug, Deserialize, PartialEq)]
    /// struct Bar {
    ///     #[serde(with = "as_hex")]
    ///     foo: Foo,
    /// }
    ///
    ///
    ///
    /// let json = r#"{"foo":"0x0face342"}"#;
    ///
    /// let (result, _): (Bar, usize) = serde_json_core::from_str(json).unwrap();
    /// let expected = Bar{ foo: Foo(262988610u32) };
    ///
    /// assert_eq!(expected, result);
    /// ```
    pub fn deserialize<'de, D, T, N>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        N: ArrayLength,
        T: From<Bytes<N>>,
    {
        struct HexVisitor<T, N>(PhantomData<T>, PhantomData<N>);

        impl<'de, T, N> Visitor<'de> for HexVisitor<T, N>
        where
            N: ArrayLength,
            T: From<Bytes<N>>,
        {
            type Value = T;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "hex encoded string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(from_hex(&v))
            }
        }

        deserializer.deserialize_str(HexVisitor(PhantomData, PhantomData))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo(u32);

    impl From<Foo> for Bytes<U4> {
        fn from(value: Foo) -> Self {
            Self::from_array(value.0.to_be_bytes())
        }
    }

    impl From<Bytes<U4>> for Foo {
        fn from(value: Bytes<U4>) -> Self {
            Self(u32::from_be_bytes(value.into()))
        }
    }

    #[test]
    fn test_into_hex() {
        let foo = Foo(262988610u32);
        let nybls = into_hex(foo);

        assert_eq!(b"0x0face342", nybls.as_slice());
    }

    #[test]
    fn test_from_hex() {
        // To test actual deserialization behavior, the string is constructed
        // at runtime to mitigate possible size predictions with string literal
        let b = b"0x0face342";
        let s = unsafe { core::str::from_utf8_unchecked(b) };

        let foo: Foo = from_hex(s);
        assert_eq!(Foo(262988610u32), foo);
    }
}
