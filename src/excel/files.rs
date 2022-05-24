use super::structs::*;

use std::collections::HashMap;

use std::io::{Read, Seek};
use std::io::SeekFrom::*;

use binread::{BinRead, BinResult, BinReaderExt, ReadOptions};

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

// ExdData (.exd) file

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExdData {
	#[br(seek_before = Current(4))]
	pub header: ExcelDataHeader,
	#[br(seek_before = Current(20), count = header.index_size / 8)]
	pub row_offsets: Vec<ExcelRowOffset>
}

/*	// oops, got fucked by 1-indexed sheets

fn map_rows<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, args: ())
-> BinResult<HashMap<u32, ExcelRowOffset>> {
	let mut map = HashMap::<u32, ExcelRowOffset>::new();

	let ct: usize = ro.count.unwrap();
	for i in 0..ct {
		let row: ExcelRowOffset = reader.read_be().unwrap();
		map.insert(
			row.row_id,
			row
		);
	}

	Ok(map)
}
*/

// ExlList (.exl) file
// TODO