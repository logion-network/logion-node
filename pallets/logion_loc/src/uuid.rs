use frame_support::codec::{Decode, Encode};
use frame_support::traits::Vec;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Encode, Decode)]
pub struct UUID {
	bytes: [u8; 32]
}

impl UUID {

	pub fn new(bytes_vec: Vec<u8>) -> Result<UUID, ()> {
		if bytes_vec.len() != 32 {
			Err(())
		} else {
			let mut bytes: [u8; 32] = [0; 32];
			for (i, x) in bytes_vec.iter().enumerate() {
				bytes[i] = *x;
			}
			Ok(UUID {
				bytes
			})
		}
	}

	pub fn bytes(self) -> [u8; 32] {
		self.bytes
	}
}

impl Default for UUID {
	fn default() -> UUID {
		UUID {
			bytes: [42; 32]
		}
	}
}
