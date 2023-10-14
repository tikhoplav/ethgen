#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Types required to construct Bytes and conversions
pub use generic_array::typenum;

mod encoding;
#[doc(inline)]
pub use encoding::{as_hex, from_hex, hex, into_hex, Bytes};
