use binread::{BinRead, io::Cursor};

// HashTableEntry

#[derive(BinRead)]
pub struct HashTableEntry {
	pub hash: u64,
	pub data: u32
}

/*impl HashTableEntry {
	fn file_id(&self) -> u32 {
		return (self.data & 0b1110) >> 1;
	}

	fn offset(&self) -> u32 {
		return (self.data & !0xF) * 0x08;
	}
}*/