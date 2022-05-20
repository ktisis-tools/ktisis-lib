use crate::sqpack::lib::hex_str;

use super::headers::*;

use std::fmt::Debug;
use std::collections::HashMap;
use std::io::{Read, Seek};

use std::io::SeekFrom::*;
use binread::{BinRead, BinResult, ReadOptions, BinReaderExt};

// SqPackIndex

#[derive(BinRead)]
#[derive(Debug)]
pub struct SqPackIndex {
	pub header: SqPackHeader,
	#[br(seek_before = Start(header.size.into()))]
	pub index: IndexHeader,
	#[br(seek_before = Start(index.data_offset.into()), parse_with = index_entry, count = index.data_size / 16)]
	pub map: HashMap<String, HashTableEntry>
}

#[derive(Debug)]
pub struct HashTableEntry {
	pub hash: String,
	pub data: u32,
	pub file_id: u32,
	pub offset: u32
}

fn index_entry<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, _: ())
-> BinResult<HashMap<String, HashTableEntry>> {
	let mut map = HashMap::new();

	let ct = ro.count.unwrap();
	for _i in 0..ct {
		let hash: u64 = reader.read_le().unwrap();
		let data: u32 = reader.read_le().unwrap();
		reader.seek(Current(4)).expect("Unexpected error while moving to next entry.");

		let digest = hex_str::<u64>(&[hash]).to_string();

		let entry = HashTableEntry {
			hash: digest.to_string(),
			data: data,
			file_id: (data & 0b1110) >> 1,
			offset: (data & !0xF) * 0x08
		};
		map.insert(
			entry.hash.to_owned(),
			entry
		);
	}

	Ok(map)
}