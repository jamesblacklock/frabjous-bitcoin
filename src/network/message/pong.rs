use std::{
	io::{Read, Write},
};

use crate::{
	err::*,
	json::JsonValue,
};

use super::{
	Deserialize,
	Serialize,
	super::{
		read_u64,
		write_u64,
	}
};

#[derive(Clone)]
pub struct Pong {
	nonce: u64,
}

impl Pong {
	pub fn new(nonce: u64) -> Self {
		Pong { nonce }
	}

	pub fn into_json(&self) -> JsonValue {
		JsonValue::number(self.nonce)
	}
}

impl Deserialize for Pong {
	fn deserialize(stream: &mut dyn Read) -> Result<Pong> {
		Ok(Pong { nonce: read_u64(stream)? })
	}
}

impl Serialize for Pong {
	fn serialize(&self, stream: &mut dyn Write) -> Result<()> {
		write_u64(stream, self.nonce)
	}
}