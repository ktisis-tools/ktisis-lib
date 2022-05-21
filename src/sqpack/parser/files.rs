use crate::sqpack::lib::hex_str;

use super::headers::*;

use std::fmt::Debug;
use std::io::SeekFrom::*;
use std::io::{Read, Seek};
use std::collections::HashMap;

use binread::{BinRead, BinResult, ReadOptions, BinReaderExt};

// SqPackIndex

#[derive(BinRead)]
pub struct SqPackIndex {
	pub header: SqPackHeader,
	#[br(seek_before = Start(header.size.into()))]
	pub index: IndexHeader,
	#[br(seek_before = Start(index.data_offset.into()), parse_with = index_entry, count = index.data_size / 16)]
	pub map: HashMap<u64, HashTableEntry>
}

#[derive(Debug)]
pub struct HashTableEntry {
	pub hash: u64,
	pub data: u32,
	pub file_id: u32,
	pub offset: u32
}

fn index_entry<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, _: ())
-> BinResult<HashMap<u64, HashTableEntry>> {
	let mut map = HashMap::new();

	let mut ct = ro.count.unwrap();
	for _i in 0..ct {
		let hash: u64 = reader.read_le().unwrap();
		let data: u32 = reader.read_le().unwrap();
		reader.seek(Current(4)).expect("seek failed");

		let entry = HashTableEntry {
			hash: hash,
			data: data,
			file_id: (data & 0b1110) >> 1,
			offset: (data & !0xF) * 0x08
		};
		map.insert(
			hash,
			entry
		);
	}

	Ok(map)
}