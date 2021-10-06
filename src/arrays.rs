/*! Implement `Endian` on standard arrays.

I want this library to remain zero-allocation, so I can't return a new Vec that
has executed the conversion, and without type-level integers I can't accept
arbitrary arrays.

This mutates an array in place, replacing each element with its converted form.
!*/

use super::Endian;
use core::ptr;

impl<T: Endian, const N: usize> Endian for [T; N] {
	fn from_be(mut self) -> Self {
		for el in self.iter_mut() { unsafe {
			ptr::write(el, ptr::read(el).from_be());
		} }
		self
	}
	fn from_le(mut self) -> Self {
		for el in self.iter_mut() { unsafe {
			ptr::write(el, ptr::read(el).from_le());
		} }
		self
	}
	fn to_be(mut self) -> Self {
		for el in self.iter_mut() { unsafe {
			ptr::write(el, ptr::read(el).to_be());
		} }
		self
	}
	fn to_le(mut self) -> Self {
		for el in self.iter_mut() { unsafe {
			ptr::write(el, ptr::read(el).to_le());
		} }
		self
	}
}

#[cfg(test)]
mod tests {
	use crate::Endian;

	#[test]
	fn arrays() {
		let src: [i32; 8] = [
			1, 2, 3, 4, 5, 6, 7, 8,
		];
		let flip: [i32; 8] = src.clone().to_be();
		let mut comp: [i32; 8] = src.clone();
		for e in comp.iter_mut() {
			*e = e.to_be();
		}

		assert_eq!(flip, comp);
	}
}
