use super::structs::*;

use binread::BinRead;
use std::io::SeekFrom::*;

// ExhHeader (.exh) file

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExhHeader {
	#[br(seek_before = Current(4))]
	pub version: u16,
	pub data_offset: u16,
	pub column_count: u16,
	pub page_count: u16,
	pub language_count: u16,
	
	#[br(seek_before = Current(3))]
	pub variant: u8,

	#[br(seek_before = Current(2))]
	pub row_count: u32,

	#[br(seek_before = Current(8), count = column_count)]
	pub columns: Vec<ExcelColumnDefinition>,

	#[br(count = page_count)]
	pub pages: Vec<ExcelPageDefinition>,

	#[br(count = language_count, little)]
	pub languages: Vec<u16>
}