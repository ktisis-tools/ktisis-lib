pub mod headers;
pub mod files;

use std::fs::File;
use std::path::Path;
use std::default::Default;

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
		
		return DatReader {
			file: file
		};
	}

	pub fn read<T: BinRead>(&mut self) -> T {
		let res: T = self.file.read_le().unwrap();
		return res;
	}
}