use crypto_bigint::{Uint, CheckedAdd};
use core::{fmt, ops::Add};

/// BigUint
///
/// A thin wrapper for `crypto_bigint::Uint` with checked math operaions
/// enabled by default with additional implementations for required features.
pub struct BigUint<const N: usize>(Uint<N>);

impl<const N: usize> BigUint<N> {
    pub const ZERO: Self = Self(Uint::<N>::ZERO);
    pub const ONE: Self = Self(Uint::<N>::ONE);
    pub const MAX: Self = Self(Uint::<N>::MAX);
}

impl<const N: usize> From<Uint<N>> for BigUint<N> {
    fn from(value: Uint<N>) -> Self {
        Self(value)
    }
}

impl<const N: usize> Into<Uint<N>> for BigUint<N> {
    fn into(self) -> Uint<N> {
        self.0
    }
}

impl<const N: usize> From<u64> for BigUint<N> {
    fn from(value: u64) -> Self {
        Self(Uint::<N>::from_u64(value))
    }
}

impl<const N: usize> Add for BigUint<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self.0
            .checked_add(&rhs.0)
            .expect("Attempt to add with overflow")
            .into()
    }
}


/// uint256
///
/// ```should_panic
/// use ethgen::uint256;
///
/// let a = uint256::MAX;
/// let b = uint256::ONE;
/// let _ = a + b;
/// ```
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type uint256 = BigUint<8>;

/// uint256
///
/// ```should_panic
/// use ethgen::uint256;
///
/// let a = uint256::MAX;
/// let b = uint256::ONE;
/// let _ = a + b;
/// ```
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type uint256 = BigUint<4>;

/// Alias for `uint256`
#[allow(non_camel_case_types)]
pub type uint = uint256;

impl fmt::Debug for uint256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "uint256 {:x}", self.0)
    }
}
