use super::files::*;
use super::structs::*;
use super::Language;

use crate::sqpack::files::SqPackFile;

use std::str::from_utf8_unchecked;
use std::ops::Range;
use std::io::SeekFrom::*;
use std::io::{Cursor, Seek};
use std::collections::HashMap;

use binread::{BinRead, BinReaderExt};

// ColumnDataType

#[derive(Eq, PartialEq, PartialOrd, Copy, Clone, BinRead, Debug)]
#[repr(u16)]
#[br(repr = u16)]
pub enum ColumnDataType {
	String = 0x0,
	Bool = 0x1,
	Int8 = 0x2,
	UInt8 = 0x3,
	Int16 = 0x4,
	UInt16 = 0x5,
	Int32 = 0x6,
	UInt32 = 0x7,
	Float32 = 0x9,
	Int64 = 0xA,
	UInt64 = 0xB,

	PackedBool0 = 0x19,
	PackedBool1 = 0x1A,
	PackedBool2 = 0x1B,
	PackedBool3 = 0x1C,
	PackedBool4 = 0x1D,
	PackedBool5 = 0x1E,
	PackedBool6 = 0x1F,
	PackedBool7 = 0x20
}

#[derive(Clone, Debug)]
pub enum ExcelValue {
	String(String),
	Bool(bool),
	Int8(i8),
	UInt8(u8),
	Int16(i16),
	UInt16(u16),
	Int32(i32),
	UInt32(u32),
	Float32(f32),
	Int64(i64),
	UInt64(u64)
}

impl ExcelValue {
	pub fn get_string(&self) -> String {
		match self {
			ExcelValue::String(val) => val.to_string(),
			ExcelValue::Bool(val) => val.to_string(),
			ExcelValue::Int8(val) => val.to_string(),
			ExcelValue::UInt8(val) => val.to_string(),
			ExcelValue::Int16(val) => val.to_string(),
			ExcelValue::UInt16(val) => val.to_string(),
			ExcelValue::Int32(val) => val.to_string(),
			ExcelValue::UInt32(val) => val.to_string(),
			ExcelValue::Float32(val) => val.to_string(),
			ExcelValue::Int64(val) => val.to_string(),
			ExcelValue::UInt64(val) => val.to_string()
		}
	}
}

// ExcelRow

#[derive(Clone)]
pub struct ExcelRow {
	pub columns: Vec<ExcelValue>
}

// ExcelPage

pub struct ExcelPage {
	file: SqPackFile,
	data: ExdData,
	start_id: u32,
	row_count: u32,
	rows: Vec<ExcelRow>
}

impl ExcelPage {
	pub fn new(file: SqPackFile, data: ExdData, def: &ExcelPageDefinition) -> ExcelPage {
		ExcelPage {
			file: file,
			data: data,
			start_id: def.start_id,
			row_count: def.row_count,
			rows: Vec::<ExcelRow>::new()
		}
	}
}

// ExcelSheet

pub struct ExcelSheet {
	pub header: ExhHeader,
	pub language: Language,
	pub pages: Vec<ExcelPage>,
	pub start_id: u32,
	pub row_cache: HashMap<u32, ExcelRow>
}

impl ExcelSheet {
	pub fn new(header: ExhHeader, language: Language) -> ExcelSheet {
		ExcelSheet {
			header: header,
			language: language,
			pages: Vec::<ExcelPage>::new(),
			start_id: 0,
			row_cache: HashMap::<u32, ExcelRow>::new()
		}
	}

	////* Rows Fetching *////

	pub fn get_row_page(&self, row: u32) -> Option<&ExcelPage> {
		for page in &self.pages {
			if row >= page.start_id && row < page.start_id + page.row_count {
				return Some(page);
			} else {
				continue;
			}
		}
		None
	}

	// Read row from page

	pub fn read_page_row(&self, reader: &mut Cursor<&Vec<u8>>, page: &ExcelPage, row: u32) -> Result<ExcelRow, binread::Error> {
		let offset = page.data.row_offsets.get((row - page.start_id) as usize).unwrap();

		let mut columns = Vec::<ExcelValue>::new();

		for column in &self.header.columns {
			reader.seek(Start(6 + offset.offset as u64 + column.offset as u64))?;

			let dtype = column.data_type;

			if dtype == ColumnDataType::String {

				// Read string value
				let str_offset: u32 = reader.read_be()?;

				let start: usize = 6 + offset.offset as usize + str_offset as usize + self.header.data_offset as usize;
				//reader.seek(Start(start));

				let mut slice = &page.file.content[start..];
				let end: usize = slice.iter().position(|&x| x == 0).unwrap();
				slice = &slice[..end];
				
				let convert = unsafe {
					from_utf8_unchecked(slice).to_owned()
				};
				columns.push(ExcelValue::String(convert));

			} else if ColumnDataType::PackedBool0 <= dtype && dtype <= ColumnDataType::PackedBool7 {

				// Convert packed boolean
				let value: u8 = reader.read_be()?;
				let shift = (dtype as u16) - (ColumnDataType::PackedBool0 as u16);
				let bit = 1 << shift;
				columns.push(ExcelValue::Bool((value & bit) == bit));

			} else {

				// Convert integer values
				let value = match dtype {
					ColumnDataType::Bool => ExcelValue::Bool( reader.read_be::<u8>()? == 1 ),
					ColumnDataType::Int8 => ExcelValue::Int8( reader.read_be::<i8>()? ),
					ColumnDataType::UInt8 => ExcelValue::UInt8( reader.read_be::<u8>()? ),
					ColumnDataType::Int16 => ExcelValue::Int16( reader.read_be::<i16>()? ),
					ColumnDataType::UInt16 => ExcelValue::UInt16( reader.read_be::<u16>()? ),
					ColumnDataType::Int32 => ExcelValue::Int32( reader.read_be::<i32>()? ),
					ColumnDataType::UInt32 => ExcelValue::UInt32( reader.read_be::<u32>()? ),
					ColumnDataType::Float32 => ExcelValue::Float32( reader.read_be::<f32>()? ),
					ColumnDataType::Int64 => ExcelValue::Int64( reader.read_be::<i64>()? ),
					ColumnDataType::UInt64 => ExcelValue::UInt64( reader.read_be::<u64>()? ),
					_ => panic!("type not implemented")
				};
				columns.push(value);
			}
		}

		Ok(ExcelRow {
			columns: columns
		})
	}

	pub fn read_row(&self, row: u32) -> Result<ExcelRow, binread::Error> {
		let page = self.get_row_page(row).unwrap(); // ?
		self.read_page_row(&mut page.file.reader(), page, row)
	}

	// Get from cache / else fetch

	pub fn get_row(&mut self, row: u32) -> Result<&ExcelRow, binread::Error> {
		if !self.row_cache.contains_key(&row) {
			let read = self.read_row(row).unwrap();
			self.row_cache.insert(row, read);
		}
		Ok(self.row_cache.get(&row).unwrap())
	}

	pub fn get_rows(&self, range: Range<u32>) -> Result<Vec<ExcelRow>, binread::Error> {
		let mut page = self.get_row_page(range.start).unwrap();
		let mut reader = page.file.reader();

		let mut rows = Vec::<ExcelRow>::new();
		for i in range {
			if i > page.start_id + page.row_count {
				page = self.get_row_page(i).unwrap();
				reader = page.file.reader();
			}

			let read = self.read_page_row(&mut reader, page, i).unwrap();
			rows.push(read);
		}

		Ok(rows)
	}
}