pub mod headers;
pub mod index;

use std::io::Read;
use std::io::Seek;
use std::fs::File;
use std::path::Path;
use std::default::Default;

use binread::prelude::*;
use binread::{BinRead, io::Cursor};

// DatReader

#[derive(Default)]
pub struct DatReader {
	file: Option<File>
}

impl DatReader {
	pub fn open(path: &Path) -> DatReader {
		let mut reader = DatReader {
			..Default::default()
		};

		reader.file = match File::open(&path) {
			Err(err) => panic!("failed to open path '{}': {}", path.display(), err),
			Ok(file) => Some::<File>(file)
		};
		
		return reader;
	}

	pub fn read<T: BinRead>(&self) -> T {
		let res: T = self.file.as_ref().unwrap().read_le().unwrap();

		return res;
	}
}