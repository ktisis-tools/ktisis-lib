use std::fs::File;
use std::path::Path;
use std::io::SeekFrom::*;
use std::io::Seek;

use binread::prelude::*;
use binread::{BinRead};

// DatReader

pub struct DatReader {
	file: File
}

impl DatReader {
	pub fn open(path: &Path) -> DatReader {
		let file = match File::open(&path) {
			Err(err) => panic!("failed to open path '{}': {}", path.display(), err),
			Ok(file) => file
		};
		
		DatReader {
			file: file
		}
	}

	pub fn offset(mut self, offset: u64) -> DatReader {
		self.file.seek(Start(offset)).expect("seek failed");
		return self;
	}

	pub fn read<T: BinRead>(&mut self) -> T {
		self.file.read_le().unwrap()
	}
}