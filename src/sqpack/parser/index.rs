use binread::{BinRead, io::Cursor};

// Indexheader

#[derive(BinRead)]
pub struct IndexHeader {
	pub size: u32,
	pub itype: u32,
	pub data_offset: u32,
	pub data_size: u32
	// seek to offset
}

// HashTableEntry

#[derive(BinRead)]
pub struct HashTableEntry {
	pub hash: u64,
	pub data: u32
}

impl HashTableEntry {
	fn file_id(&self) -> u32 {
		return (self.data & 0b1110) >> 1;
	}

	fn offset(&self) -> u32 {
		return (self.data & !0xF) * 0x08;
	}
}