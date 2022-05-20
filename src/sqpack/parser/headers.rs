use std::io::SeekFrom::*;
use std::fmt::Debug;

use binread::{BinRead, io::Cursor};

// SqPackheader

#[derive(BinRead)]
pub struct SqPackHeader {
	#[br(seek_before = Start(8))]
	pub platform: u32,
	pub size: u32,
	pub version: u32,
	pub _type: u32
}

// Indexheader

#[derive(BinRead)]
pub struct IndexHeader {
	pub header: SqPackHeader,
	#[br(seek_before = Start(header.size.into()))]
	pub size: u32,
	pub _type: u32,
	pub data_offset: u32,
	pub data_size: u32
}

