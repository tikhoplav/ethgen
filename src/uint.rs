use crypto_bigint::{Uint, CheckedAdd};
use core::{fmt, ops::{Add}};

// Ethereum uint
//
// A thin wrapper around `crypto_bigint` Uint. Uses checked math by default and
// implements custom hex encoding to fit the ethereum JSON RPC convention.
pub struct EthUint<const L: usize>(Uint<L>);

impl<const L: usize> EthUint<L> {
	pub const ZERO: Self = Self(Uint::<L>::ZERO);
	pub const ONE: Self = Self(Uint::<L>::ONE);
	pub const MAX: Self = Self(Uint::<L>::MAX);

	pub fn from(val: u8) -> Self {
		Self(Uint::from(val))
	}

	pub fn checked_add(self, rhs: Self) -> Option<Self> {
		let opt = self.0.checked_add(&rhs.0);
		match Into::<bool>::into(opt.is_some()) {
			true => Some(Self(opt.unwrap())),
			false => None,
		}
	}
}

impl<const L:usize> From<Uint<L>> for EthUint<L> {
	fn from(val: Uint<L>) -> Self {
		Self(val)
	}
}

impl<const L:usize> Into<Uint<L>> for EthUint<L> {
	fn into(self: Self) -> Uint<L> {
		self.0
	}
}

impl<const L: usize> Add for EthUint<L> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		self.0.checked_add(&rhs.0)
			.expect("Attempt to add with overflow")
			.into()
	}
}

#[allow(non_camel_case_types)]
pub type uint256 = EthUint<4>;

impl fmt::Debug for uint256 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "uint256 {:x}", self.0)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	// #[test]
	// fn uint256_add() {
	// 	let a = uint256::MAX;
	// 	let arr = a.0.to_be_byte_array();
	// 	panic!("{:?}", arr);
	// 	// implement from u64
	// 	// implement from hex string
	// 	// implement Eq and PartialEq
	// }

	#[test]
	#[should_panic]
	fn uint256_add_overflow() {
		let a = uint256::MAX;
		let b = uint256::from(127u8);
		let _ = a + b;
	}
}
