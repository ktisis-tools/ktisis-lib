pub mod lib;
mod parser;

use parser::DatReader;
use parser::files::*;

use std::path::Path;
use std::default::Default;
use std::collections::HashMap;

// SqPackChunk

pub struct SqPackChunk {
	cat: u8,
	ex: u8,
	chunk: u8,
	index: SqPackIndex
}

// SqPack

#[derive(Default)]
pub struct SqPack {
	path: String,
	chunks: HashMap<u8, Vec<SqPackChunk>>
}

impl SqPack {
	// File Indexing
	fn index_repo(&mut self, repo: &str) {
		let repo_path = Path::new(&self.path).join(repo);
		assert!(repo_path.exists(), "repo does not exist in path: {repo}");

		let files = repo_path.read_dir();
		assert!(files.is_ok(), "failed to read repo '{repo}': {}", files.err().unwrap());

		for file in files.expect("read_dir call failed") {
			if let Ok(file) = file {
				let path = file.path();

				let ext = path.extension().expect("index");
				if ext != "index" { continue };

				// Parse filename

				let stem = path.file_stem().unwrap().to_str().unwrap();
				let [cat, ex, chk] = lib::parse_dat_stem(stem);

				// Index chunk

				let index = DatReader::open(&path).read::<SqPackIndex>();
				println!("{:?}: {} entries indexed.", path.file_name().unwrap(), index.map.keys().len());

				let chunk = SqPackChunk {
					cat: cat,
					ex: ex,
					chunk: chk,
					index: index
				};

				// Push to category map

				if !self.chunks.contains_key(&cat) {
					self.chunks.insert(
						cat,
						Vec::<SqPackChunk>::new()
					);
				}
				let cat_chunks = self.chunks.get_mut(&cat).unwrap();
				cat_chunks.push(chunk);
			}
		}
	}



	// File Fetching

	fn get_file(&self, path: &str) {

	}
}

// Public methods

pub fn new(dir: &str) -> SqPack {
	assert!(Path::new(dir).exists(), "sqpack path does not exist: {dir}");

	let mut sqpack = SqPack {
		path: dir.to_string(),
		..Default::default()
	};
	return sqpack;
}

pub fn load_all(dir: &str) -> SqPack {
	let mut sqpack = new(dir);

	let path = Path::new(dir);

	let files = path.read_dir();
	for file in files.expect("read_dir call failed") {
		if let Ok(file) = file {
			let repo = file.path();
			sqpack.index_repo(repo.file_name().unwrap().to_str().unwrap());
		}
	}

	return sqpack;
}

pub fn load_repo(dir: &str, repo: &str) -> SqPack {
	let mut sqpack = new(dir);
	sqpack.index_repo(repo);
	return sqpack;
}