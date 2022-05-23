use super::structs::*;

use std::io::{Read, Seek};
use std::io::SeekFrom::*;

use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};

// ExhHeader (.exh) file

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExhHeader {
	#[br(seek_before = Current(4))]
	version: u16,
	data_offset: u16,
	column_count: u16,
	page_count: u16,
	language_count: u16,
	
	#[br(seek_before = Current(3))]
	variant: u8,

	#[br(seek_before = Current(2))]
	row_count: u32,

	#[br(seek_before = Current(8), count = column_count)]
	columns: Vec<ExcelColumnDefinition>,

	#[br(count = page_count)]
	pages: Vec<ExcelPageDefinition>,

	#[br(count = language_count, little)]
	languages: Vec<u16>
}