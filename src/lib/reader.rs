use std::fs::read;

use std::io::Seek;
use std::io::Cursor;
use std::io::SeekFrom::*;
use std::path::Path;

use binread::prelude::*;
use binread::{BinRead};

// DatReader

pub struct DatReader {
	reader: Cursor<Vec<u8>>
}

impl DatReader {
	pub fn open(path: &Path) -> DatReader {
		let reader = Cursor::new(
			read(path).unwrap()
		);

		DatReader {
			reader: reader
		}
	}

	pub fn offset(mut self, offset: u64) -> DatReader {
		self.reader.seek(Start(offset)).expect("seek failed");
		return self;
	}

	pub fn read<T: BinRead>(&mut self) -> T {
		self.reader.read_le().unwrap()
	}
}