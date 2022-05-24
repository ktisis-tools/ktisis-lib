use super::files::*;
use super::structs::*;
use super::Language;

use crate::sqpack::files::SqPackFile;

use std::any::Any;
use std::collections::HashMap;
use std::io::{Cursor, Seek};
use std::io::SeekFrom::*;

// ColumnDataType

#[derive(Eq, PartialEq, Debug)]
#[repr(u16)]
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

// ExcelRow

pub struct ExcelRow {
	pub columns: Vec<Box<dyn Any>>
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
	pub row_cache: HashMap<u32, ExcelRow>
}

impl ExcelSheet {
	pub fn new(header: ExhHeader, language: Language) -> ExcelSheet {
		ExcelSheet {
			header: header,
			language: language,
			pages: Vec::<ExcelPage>::new(),
			row_cache: HashMap::<u32, ExcelRow>::new()
		}
	}

	////* Rows Fetching *////

	pub fn get_row_page(&self, row: u32) -> Option<&ExcelPage> {
		for page in &self.pages {
			if row >= page.start_id && row < page.start_id + page.row_count - 1 {
				return Some(page);
			} else {
				continue;
			}
		}
		None
	}

	// Read row from page

	pub fn read_page_row(&self, reader: Cursor<&Vec<u8>>, page: &ExcelPage, row: u32) {
		let offset = page.data.row_offsets.get((row - page.start_id) as usize);
		println!("{:#?}", offset);

		let columns = Vec::<Box<dyn Any>>::new();

		for column in &self.header.columns {
			//reader.seek(Start( + self.header.data_offset));
		}

		//reader.seek(Current());
	}

	pub fn read_row(&self, row: u32) {
		let page = self.get_row_page(row).unwrap(); // ?
		self.read_page_row(page.file.reader(), page, row);
	}

	// Get from cache / else fetch

	pub fn get_row(&self, row: u32) {
		if self.row_cache.contains_key(&row) {
			//self.row_cache.get(&row).unwrap()
			self.read_row(row) // TODO: UNCOMMENT THIS ^^^^^
		} else {
			self.read_row(row)
		}
	}
}