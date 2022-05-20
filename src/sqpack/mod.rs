mod lib;

use std::path::Path;
use std::default::Default;
use std::collections::HashMap;

// SqPackIndex

pub struct SqPackIndex {
}

// SqPack

#[derive(Default)]
pub struct SqPack<'a> {
	path: &'a str,
	map: HashMap<&'a str, SqPackIndex>
}

impl SqPack<'_> {
	fn index_repo(&self, repo: &str) {
		let repo_path = Path::new(self.path).join(repo);

		assert!(repo_path.exists(), "repo path does not exist: {repo}");

		for (cat, id) in &lib::CATEGORIES {
		}
	}

	fn get_file(&self, path: &str) {

	}
}

// Public methods

pub fn load_repo<'a>(path: &'a str, repo: &'a str) -> SqPack<'a> {
	assert!(Path::new(path).exists(), "sqpack path does not exist: {path}");

	let sqpack = SqPack {
		path: path,
		..Default::default()
	};
	sqpack.index_repo(repo);
	return sqpack;
}

pub fn load_repos<'a>(path: &'a str) {
	
}