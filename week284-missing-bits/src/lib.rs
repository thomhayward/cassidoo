use std::fmt::{self, Write};

pub fn missing_bits<T, U>(src: impl IntoIterator<IntoIter = U>) -> Result<String, fmt::Error>
where
	U: Iterator<Item = T> + MissingBits<T>,
{
	src.into_iter().missing_bits()
}

pub trait MissingBits<T> {
	fn missing_bits(&mut self) -> Result<String, fmt::Error>;
}

macro_rules! impl_missing_bits {
	($type:ty) => {
		impl<I: Iterator<Item = $type>> MissingBits<$type> for I {
			fn missing_bits(&mut self) -> Result<String, fmt::Error> {
				let mut buffer = String::new();
				let mut previous = None;

				write!(&mut buffer, "[")?;
				for (index, value) in self.enumerate() {
					if index > 0 {
						write!(&mut buffer, ",")?;
					}
					match value - previous.unwrap_or(value) {
						0 | 1 => write!(&mut buffer, "{value}"),
						2 => write!(&mut buffer, "{},{value}", value - 1),
						_ => write!(&mut buffer, "...,{value}"),
					}?;
					previous = Some(value);
				}
				write!(&mut buffer, "]")?;
				Ok(buffer)
			}
		}
	};
}

impl_missing_bits!(i8);
impl_missing_bits!(i16);
impl_missing_bits!(i32);
impl_missing_bits!(i64);
impl_missing_bits!(i128);
impl_missing_bits!(isize);
impl_missing_bits!(u8);
impl_missing_bits!(u16);
impl_missing_bits!(u32);
impl_missing_bits!(u64);
impl_missing_bits!(u128);
impl_missing_bits!(usize);

#[cfg(test)]
mod tests {
	use super::missing_bits;
	#[test]
	fn example1() {
		assert_eq!(
			missing_bits([1, 2, 3, 4, 20, 21, 22, 23]).unwrap(),
			"[1,2,3,4,...,20,21,22,23]"
		)
	}
	#[test]
	fn example2() {
		assert_eq!(missing_bits([1, 2, 3, 5, 6]).unwrap(), "[1,2,3,4,5,6]")
	}
	#[test]
	fn example3() {
		assert_eq!(
			missing_bits([1, 3, 20, 27]).unwrap(),
			"[1,2,3,...,20,...,27]"
		)
	}
}
