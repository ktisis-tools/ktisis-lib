mod lib;

use std::path::Path;
use std::default::Default;
use std::collections::HashMap;

// SqPackIndex

#[derive(Default)]
pub struct SqPackIndex {
	cat: u8,
	ex: u8,
	chunk: u8
}

// SqPackChunk

pub struct SqPackChunk {
	map: HashMap<String, SqPackIndex>
}

// SqPack

#[derive(Default)]
pub struct SqPack {
	path: String,
	chunks: Vec<SqPackChunk>
}

impl SqPack {
	fn index_repo(&self, repo: &str) {
		let repo_path = Path::new(&self.path).join(repo);
		assert!(repo_path.exists(), "repo does not exist in path: {repo}");

		let files = repo_path.read_dir();
		assert!(files.is_ok(), "failed to read repo '{repo}': {}", files.err().unwrap());

		for file in files.expect("read_dir call failed") {
			if let Ok(file) = file {
				let path = file.path();
			}
		}

		for (cat, id) in &lib::CATEGORIES {

		}
	}

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