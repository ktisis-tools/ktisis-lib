pub mod lib;
mod parser;

use parser::DatReader;
use parser::files::*;

use std::path::Path;
use std::default::Default;

// SqPackChunk

#[derive(Default)]
pub struct SqPackChunk {
	cat: u8,
	ex: u8,
	chunk: u8
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
		let stuff = DatReader::open(path).read::<SqPackIndex>();
		println!("{:?}: {} entries indexed.", path.file_name().unwrap(), stuff.index.data_size / 16);
	}

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

				let stem = path.file_stem().unwrap().to_str().unwrap();
				let [cat, ex, chk] = lib::parse_dat_stem(stem);

				let index = SqPackChunk {
					cat: cat,
					ex: ex,
					chunk: chk,
					..Default::default()
				};
				self.index_file(&path);
				self.chunks.push(index);
			}
		}
	}

	// File Fetching

	fn get_file(&self, path: &str) {

	}
}

// Public methods

pub fn load_repo(path: &str, repo: &str) -> SqPack {
	assert!(Path::new(path).exists(), "sqpack path does not exist: {path}");

	let mut sqpack = SqPack {
		path: path.to_string(),
		..Default::default()
	};
	sqpack.index_repo(repo);
	return sqpack;
}

pub fn load_repos(path: &str) {
	
}