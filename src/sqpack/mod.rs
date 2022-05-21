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

	pub fn index_file(&mut self, path: &Path) {
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

		self.index_chunk(chunk);
	}

	pub fn index_category(&mut self, cat: u8) {
		let root = Path::new(&self.path);

		// Iterate repos
		let repos = root.read_dir();
		for repo in repos.expect("read_dir call failed") {
			if let Ok(repo) = repo {
				let path = repo.path();
				let name = path.file_name().unwrap().to_str().unwrap();
				let ex = lib::parse_repo(&name);
				println!("{} {}", name, ex);

				// Iterate chunks
				for chunk in 0..255 {
					let path = path.join(lib::dat_str(cat, ex, chunk, "index"));
					if !path.exists() { break; }
					
					self.index_file(&path);
				}
			}
		}
	}

	pub fn index_repo(&mut self, repo: &str) {
		let repo_path = Path::new(&self.path).join(repo);
		assert!(repo_path.exists(), "repo does not exist in path: {repo}");

		let files = repo_path.read_dir();
		assert!(files.is_ok(), "failed to read repo '{repo}': {}", files.err().unwrap());

		for file in files.expect("read_dir call failed") {
			if let Ok(file) = file {
				let path = file.path();

				let ext = path.extension().unwrap();
				if ext != "index" { continue };

				self.index_file(&path);
			}
		}
	}

	pub fn index_chunk(&mut self, chunk: SqPackChunk) {
		if !self.chunks.contains_key(&chunk.cat) {
			self.chunks.insert(
				chunk.cat,
				Vec::<SqPackChunk>::new()
			);
		}

		let cat_chunks = self.chunks.get_mut(&chunk.cat).unwrap();
		cat_chunks.push(chunk);
	}

	// File Fetching

	pub fn get_file(&self, path: &str) {

	}
}

// SqPack

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

// Category

pub fn category(name: &str) -> u8 {
	return lib::CATEGORY[&name];
}