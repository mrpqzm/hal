#[macro_use]
extern crate serde_derive;
extern crate bitcoin;
extern crate hex;
extern crate serde;

pub mod address;
pub mod bip32;
pub mod psbt;
pub mod tx;

/// Utility struct to serialize byte strings as hex.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct HexBytes(pub Vec<u8>);

impl From<Vec<u8>> for HexBytes {
	fn from(vec: Vec<u8>) -> HexBytes {
		HexBytes(vec)
	}
}

impl From<&[u8]> for HexBytes {
	fn from(slice: &[u8]) -> HexBytes {
		HexBytes(slice.to_vec())
	}
}

impl ::serde::Serialize for HexBytes {
	fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
		s.serialize_str(&hex::encode(&self.0))
	}
}

impl<'de> ::serde::Deserialize<'de> for HexBytes {
	fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HexBytes, D::Error> {
		use serde::de::Error;

		let hex_str: String = ::serde::Deserialize::deserialize(d)?;
		Ok(HexBytes(hex::decode(hex_str).map_err(D::Error::custom)?))
	}
}
