use std::{
	io::{Read, Write},
};

use crate::{
	err::*,
	json::JsonValue,
	sha256::*,
};

use crate::common::{
	read_u32,
	read_i32,
	read_sha256,
	read_var_int,
	write_u32,
	write_i32,
	write_sha256,
	write_var_int,
};

use super::{
	Deserialize,
	Serialize,
};

#[derive(Clone)]
pub struct Header {
	pub version: i32,
	pub prev_block: Sha256,
	pub merkle_root: Sha256,
	pub timestamp: u32,
	pub bits: u32,
	pub nonce: u32,
	pub tx_count: usize,
}

impl Header {
	pub fn into_json(&self) -> JsonValue {
		JsonValue::object([
			("version", JsonValue::number(self.version)),
			("prev_block", JsonValue::string(format!("{}", self.prev_block))),
			("merkle_root", JsonValue::string(format!("{}", self.merkle_root))),
			("timestamp", JsonValue::number(self.timestamp)),
			("bits", JsonValue::number(self.bits)),
			("nonce", JsonValue::number(self.nonce)),
		])
	}

	pub fn tx_count(&self) -> usize {
		self.tx_count
	}

	pub fn compute_hash(&self) -> Sha256 {
		let mut buf = Vec::new();
		self.serialize_without_tx_count(&mut buf).unwrap();
		compute_double_sha256(&*buf)
	}

	fn serialize_without_tx_count(&self, stream: &mut dyn Write) -> Result<()> {
		write_i32(stream, self.version)?;
		write_sha256(stream, &self.prev_block)?;
		write_sha256(stream, &self.merkle_root)?;
		write_u32(stream, self.timestamp)?;
		write_u32(stream, self.bits)?;
		write_u32(stream, self.nonce)
	}
}

impl Deserialize for Header {
	fn deserialize(stream: &mut dyn Read) -> Result<Header> {
		let version = read_i32(stream)?;
		let prev_block = read_sha256(stream)?;
		let merkle_root = read_sha256(stream)?;
		let timestamp = read_u32(stream)?;
		let bits = read_u32(stream)?;
		let nonce = read_u32(stream)?;
		let tx_count = read_var_int(stream)? as usize;

		Ok(Header { 
			version,
			prev_block,
			merkle_root,
			timestamp,
			bits,
			nonce,
			tx_count,
		})
	}
}

impl Serialize for Header {
	fn serialize(&self, stream: &mut dyn Write) -> Result<()> {
		self.serialize_without_tx_count(stream)?;
		write_var_int(stream, self.tx_count as u64)
	}
}

#[derive(Clone)]
pub struct Headers(Vec<Header>);

impl Headers {
	// pub fn new() -> Self {
	// 	Headers {
	// 		???
	// 	}
	// }

	pub fn into_json(&self) -> JsonValue {
		JsonValue::array(self.0.iter().map(|e| e.into_json()))
	}

	pub fn iter(&self) -> std::slice::Iter<Header> {
		self.0.iter()
	}
}

impl Deserialize for Headers {
	fn deserialize(stream: &mut dyn Read) -> Result<Headers> {
		let count = read_var_int(stream)? as usize;
		let mut headers = Vec::new();
		for _ in 0..count {
			headers.push(Header::deserialize(stream)?);
		}
		Ok(Headers(headers))
	}
}

impl Serialize for Headers {
	fn serialize(&self, stream: &mut dyn Write) -> Result<()> {
		write_var_int(stream, self.0.len() as u64)?;
		for header in self.0.iter() {
			header.serialize(stream)?;
		}
		Ok(())
	}
}