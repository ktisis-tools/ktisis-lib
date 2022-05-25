use crate::excel::sheet::ColumnDataType;

use binread::BinRead;
use std::io::SeekFrom::*;

// ExhHeader (.exh) file definitions

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelColumnDefinition {
	pub data_type: ColumnDataType,
	pub offset: u16
}

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelPageDefinition {
	pub start_id: u32,
	pub row_count: u32
}

// ExdData (.exd) file definitions

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelDataHeader {
	pub version: u16,
	#[br(seek_before = Current(2))]
	pub index_size: u32
}

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelRowOffset {
	pub row_id: u32,
	pub offset: u32
}