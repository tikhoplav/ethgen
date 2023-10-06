use generic_array::{typenum::{U2, Sum}, GenericArray, ArrayLength};
use core::ops::Add;
use super::Encode;

/// Length of the encoded hex string
type HLen<E> = Sum<<E as Encode>::BytesLength, <E as Encode>::BytesLength>;
/// Length of the encoded human-readable hex string
type HRLen<E> = Sum<HLen<E>, U2>;

/// Serialize a struct as a hex string
pub fn serialize<E, S>(data: &E, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
	E: Encode,
	E::BytesLength: Add<E::BytesLength>,
	Sum<E::BytesLength, E::BytesLength>: Add<U2>,
	Sum<E::BytesLength, E::BytesLength>: ArrayLength,
	Sum<Sum<E::BytesLength, E::BytesLength>, U2>: ArrayLength,
{
	let src = data.as_ref();

	if serializer.is_human_readable() {
		let mut dst: GenericArray::<u8, HRLen<E>> = GenericArray::default();

	    dst[0] = 48u8;
	    dst[1] = 120u8;

	    unsafe {
	    	faster_hex::hex_encode_upper(src, &mut dst[2..]).unwrap_unchecked()
	    };

	    let s = unsafe { core::str::from_utf8_unchecked(&dst) };
	    serializer.serialize_str(&s)
	} else {
		let mut dst: GenericArray::<u8, HLen<E>> = GenericArray::default();

		unsafe {
	    	faster_hex::hex_encode_upper(src, &mut dst).unwrap_unchecked()
	    };

	    let s = unsafe { core::str::from_utf8_unchecked(&dst) };
	    serializer.serialize_str(&s)
	}
}

#[cfg(test)]
mod test {
    use super::*;
    use generic_array::typenum::U8;

    struct Dummy {
    	bytes: [u8; 8],
    }

    impl AsRef<[u8]> for Dummy {
    	fn as_ref(&self) -> &[u8] {
    		self.bytes.as_ref()
    	}
    }

    impl Encode for Dummy {
    	type BytesLength = U8;
    }

    #[test]
    fn test_serialize() {
    	let dummy = Dummy { bytes: *b"\0\0ethers" };

    	let mut buf = [0u8; 256];
    	let mut serializer = serde_json_core::ser::Serializer::new(&mut buf);
    	serialize(&dummy, &mut serializer).unwrap();
    	let n = serializer.end();

        assert_eq!(b"\"0x0000657468657273\"", &buf[..n]);
    }
}
