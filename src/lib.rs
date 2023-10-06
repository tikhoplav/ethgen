#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use generic_array::typenum;

pub mod hex;
pub mod rpc;

mod uint;
#[doc(inline)]
pub use uint::uint256;
