use super::headers::*;

use std::io::Cursor;
use std::io::SeekFrom::*;
use std::io::{Read, Seek};
use std::collections::HashMap;

use inflate::inflate_bytes;

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

pub struct HashTableEntry {
	pub hash: u64,
	pub data: u32,
	pub file_id: u32,
	pub offset: u32
}

fn index_entry<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, _: ())
-> BinResult<HashMap<u64, HashTableEntry>> {
	let mut map = HashMap::new();

	let ct = ro.count.unwrap();
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

// SqPackFile

#[derive(BinRead)]
pub struct SqPackFileInfo {
	pub size: u32,
	_type: u32,
	size_raw: u32,
	#[br(seek_before = Current(8))]
	block_num: u32
}

#[derive(BinRead)]
struct SqPackBlockInfo {
	offset: u32,
	size: u16,
	size_uncomp: u16
}

#[derive(BinRead)]
struct SqPackBlockHeader {
	size: u32,
	#[br(seek_before = Current(4))]
	size_comp: u32,
	size_uncomp: u32
}

#[derive(BinRead)]
pub struct SqPackFile {
	#[br(parse_with = store_offset)]
	_offset: u64,
	pub finfo: SqPackFileInfo,
	#[br(count = finfo.block_num)]
	blocks: Vec<SqPackBlockInfo>,
	#[br(parse_with = read_blocks, args(&_offset, &finfo, &blocks))]
	pub content: Vec<u8>
}

impl SqPackFile {
	pub fn reader(&self) -> Cursor<&Vec<u8>> {
		Cursor::new(&self.content)
	}

	pub fn parse<T: BinRead>(&self) -> T {
		self.reader().read_be().unwrap()
	}
}

fn read_blocks<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, args: (&u64, &SqPackFileInfo, &Vec<SqPackBlockInfo>,))
-> BinResult<Vec<u8>> {
	let offset = args.0;
	let finfo = args.1;
	let blocks = args.2;

	let base_offset = offset + finfo.size as u64;

	let mut content = Vec::<u8>::new();
	for block in blocks {
		reader.seek(Start(base_offset + block.offset as u64)).expect("seek failed");

		let head: SqPackBlockHeader = reader.read_le().unwrap();
		
		let mut buffer = vec![0u8; head.size_comp as usize];
		reader.read_exact(&mut buffer)?;

		let decode = inflate_bytes(&buffer).unwrap();
		content = [content, decode].concat();
	}
	Ok(content)
}

// Save Offset

fn store_offset<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ())
-> BinResult<u64> {
	Ok(reader.seek(Current(0)).unwrap())
}