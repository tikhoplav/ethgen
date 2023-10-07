#[inline(always)]
fn nybl_lower(b: u8) -> u8 {
	let b = b as i16;
	(b + 0x30 + (((0x9 - b) >> 8) & (0x61i16 - 0x3a))) as u8
}

#[inline(always)]
fn nybl_upper(b: u8) -> u8 {
	let b = b as i16;
	(b + 0x30 + (((0x9 - b) >> 8) & (0x41i16 - 0x3a))) as u8
}

/// An iterator that yields hex encoded bytes
pub trait Encoder: Iterator<Item = u8> + ExactSizeIterator {}

macro_rules! impl_iter_encoder {
	(	$(#[$meta:meta])*
		$name:ident => $fn:ident
	) => {
		$(#[$meta])*
		pub struct $name<'a, I>
		where
			I: Iterator<Item = &'a u8>
		{
			inner: I,
			last: Option<&'a u8>,
		}

		impl<'a, I> $name<'a, I>
		where
			I: Iterator<Item = &'a u8>
		{
			pub fn new(inner: I) -> Self {
				Self { inner, last: None }
			}
		}

		impl<'a, I> Iterator for $name<'a, I>
		where
			I: Iterator<Item = &'a u8>
		{
			type Item = u8;

			fn next(&mut self) -> Option<Self::Item> {
				match self.last {
					Some(byte) => {
						self.last = None;
						Some($fn(byte & 0x0f))
					},
					None => {
						match self.inner.next() {
							Some(byte) => {
								self.last = Some(byte);
								Some($fn(byte >> 4))
							},
							None => None,
						}
					}
				}
			}

			fn size_hint(&self) -> (usize, Option<usize>) {
				let (mut lower, mut upper) = self.inner.size_hint();
				lower = lower << 1;
				upper = match upper {
					Some(upper) => Some(upper << 1),
					None => None,
				};
				(lower, upper)
			}
		}

		impl<'a, I> ExactSizeIterator for $name<'a, I>
		where
			I: Iterator<Item = &'a u8>
		{
			//
		}

		impl<'a, I> Encoder for $name<'a, I>
		where
			I: Iterator<Item = &'a u8>
		{
			//
		}
	}
}

impl_iter_encoder! {
	/// Encode to lower hex string
	IterEncoder => nybl_lower
}

impl_iter_encoder! {
	/// Encode to upper hex string
	IterEncoderUpper => nybl_upper
}
