use binread::{BinRead};
use std::io::SeekFrom::*;

// SqPackheader

#[derive(BinRead)]
#[derive(Debug)]
pub struct SqPackHeader {
	#[br(seek_before = Start(8))]
	pub platform: u32,
	pub size: u32,
	pub version: u32,
	pub _type: u32
}

// IndexHeader

#[derive(BinRead)]
#[derive(Debug)]
pub struct IndexHeader {
	pub size: u32,
	pub _type: u32,
	pub data_offset: u32,
	pub data_size: u32
}