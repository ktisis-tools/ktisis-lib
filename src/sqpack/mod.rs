mod lib;
mod parser;

use parser::*;
use parser::headers::*;

use std::path::Path;
use std::fmt::Debug;
use std::default::Default;
use std::collections::HashMap;

use binread::{BinRead, io::Cursor};

// SqPackIndex

#[derive(Default)]
#[derive(Debug)]
pub struct SqPackIndex {
}

// SqPackChunk

#[derive(Default)]
#[derive(Debug)]
pub struct SqPackChunk {
	cat: u8,
	ex: u8,
	chunk: u8,
	map: HashMap<String, SqPackIndex>
}

// SqPack

#[derive(Default)]
pub struct SqPack {
	path: String,
	chunks: Vec<SqPackChunk>
}

impl SqPack {
	// File Indexing

	fn index_file(&self, path: &Path) {
		let stuff = DatReader::open(path).read::<IndexHeader>();

		println!("{:?}", stuff);

		//let parse = DatReader::read::<parser::index::IndexHeader>();
	}

	fn index_repo(&self, repo: &str) {
		let repo_path = Path::new(&self.path).join(repo);
		assert!(repo_path.exists(), "repo does not exist in path: {repo}");

		let files = repo_path.read_dir();
		assert!(files.is_ok(), "failed to read repo '{repo}': {}", files.err().unwrap());

		for file in files.expect("read_dir call failed") {
			if let Ok(file) = file {
				let path = file.path();

				let ext = path.extension().expect("index");
				if ext != "index" { continue };

				let stem = path.file_stem().unwrap().to_str().unwrap();
				let [cat, ex, chk] = lib::parse_dat_stem(stem);

				let index = SqPackChunk {
					cat: cat,
					ex: ex,
					chunk: chk,
					..Default::default()
				};

				self.index_file(&path);
			}
		}

		for (cat, id) in &lib::CATEGORIES {

		}
	}

	// File Fetching

	fn get_file(&self, path: &str) {

	}
}

// Public methods

pub fn load_repo(path: &str, repo: &str) -> SqPack {
	assert!(Path::new(path).exists(), "sqpack path does not exist: {path}");

	let sqpack = SqPack {
		path: path.to_string(),
		..Default::default()
	};
	sqpack.index_repo(repo);
	return sqpack;
}

pub fn load_repos(path: &str) {
	
}