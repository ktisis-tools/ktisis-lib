pub mod files;
mod headers;

use crate::internal;
use crate::internal::reader::DatReader;

use crate::excel::Language;
use crate::excel::sheet::*;
use crate::excel::files::*;

use files::{SqPackFile, SqPackIndex, HashTableEntry};

use std::path::Path;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

// SqPack

pub struct SqPack {
	path: String,
	chunks: HashMap<u8, HashMap<u32, SqPackChunk>>, // category: [chunks]
	pub language: Language // default
}

impl SqPack {
	pub fn new(path: &str) -> Result<SqPack, Error> {
		if Path::new(path).exists() {
			Ok(SqPack {
				path: path.to_string(),
				chunks: HashMap::<u8, HashMap<u32, SqPackChunk>>::new(),
				language: Language::English
			})
		} else {
			Err(Error::from(ErrorKind::NotFound))
		}
	}

	pub fn set_language(&mut self, language: Language) {
		self.language = language;
	}
	
	////* Indexing *////

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

	pub fn index_category(&mut self, cat: u8) {
		let root = Path::new(&self.path);

		// Iterate repos
		let repos = root.read_dir();
		for repo in repos.expect("read_dir call failed") {
			if let Ok(repo) = repo {
				let path = repo.path();
				let name = path.file_name().unwrap().to_str().unwrap();
				let ex = internal::parse_repo(&name);

				// Iterate chunks
				for chunk in 0..255 {
					let path = path.join(internal::dat_str(cat, ex, chunk, "index"));
					if !path.exists() { break; }
					
					self.index_file(&path);
				}
			}
		}
	}

	pub fn index_file(&mut self, path: &Path) {
		// Parse filename

		let stem = path.file_stem().unwrap().to_str().unwrap();
		let [cat, ex, chk] = internal::parse_dat_stem(stem);

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

	pub fn index_chunk(&mut self, chunk: SqPackChunk) {
		if !self.chunks.contains_key(&chunk.cat) {
			self.chunks.insert(
				chunk.cat,
				HashMap::<u32, SqPackChunk>::new()
			);
		}

		let cat_chunks = self.chunks.get_mut(&chunk.cat).unwrap();
		cat_chunks.insert(
			chunk.hash().try_into().unwrap(),
			chunk
		);
	}

	////* File Handling *////

	pub fn find_file(&self, file: String) -> Result<FileFindResult, Error> {
		// Category
		let first = file.find("/").unwrap();
		let cat = category(&file[..first]);

		// Hash
		let hash = internal::hash_path(&file);

		// Search chunks
		for (_cat, chunk) in &self.chunks[&cat] {
			if chunk.index.map.contains_key(&hash) {
				let res = FileFindResult {
					chunk: chunk,
					entry: chunk.index.map.get(&hash).unwrap()
				};
				return Ok(res);
			} else {
				continue;
			}
		}

		Err(Error::from(ErrorKind::NotFound))
	}

	pub fn get_file(&self, file: String) -> Result<SqPackFile, Error> {
		let find = self.find_file(file)?;

		let mut reader = DatReader::open(
			&Path::new(&self.path).join(find.resolve())
		);
		reader.offset(find.entry.offset as u64);
		Ok(reader.read::<SqPackFile>())
	}

	////* Sheets *////

	pub fn get_sheet_list(&self) -> Result<Vec<String>, Error> {
		let content = self.get_file("exd/root.exl".to_string())?.to_string();

		let mut entries = Vec::<String>::new();

		let split: Vec<&str> = content.split("\r\n").collect();
		for i in 1..split.len()-1 {
			let x = split[i].find(",").unwrap();
			entries.push(split[i][..x].to_string());
		}

		Ok(entries)
	}

	pub fn get_sheet_header(&self, sheet: &str) -> Result<ExhHeader, Error> {
		Ok(self.get_file(format!("exd/{sheet}.exh"))?.parse::<ExhHeader>())
	}

	pub fn get_sheet(&self, name: &str) -> Result<ExcelSheet, Error> {
		println!("getting sheet: {name}");

		let find = self.find_file(format!("exd/{name}.exh"))?;

		// Open file reader for exd chunk

		let mut reader = DatReader::open(
			&Path::new(&self.path).join(find.resolve())
		);
		reader.offset(find.entry.offset as u64);

		// Read Header

		let header = reader.read::<SqPackFile>().parse::<ExhHeader>();

		let language = if header.languages.contains(&self.language) {
			self.language
		} else {
			header.languages[0]
		};

		// Construct Sheet

		let mut sheet = ExcelSheet::new(
			header,
			language
		);

		// Read Pages

		for i in 0..sheet.header.pages.len() {
			let page_def = sheet.header.pages.get(i).unwrap();
			if i == 0 {
				sheet.start_id = page_def.start_id;
			}

			let path = format!("exd/{name}_{}{}.exd", page_def.start_id, language.suffix());
			let hash = internal::hash_path(&path);

			if let Some(entry) = find.chunk.index.map.get(&hash) {
			//let entry = find.chunk.index.map.get(&hash).unwrap();
				reader.offset(entry.offset as u64);
					
				let file = reader.read::<SqPackFile>();
				let data = file.parse::<ExdData>();

				let page = ExcelPage::new(file, data, page_def);
				sheet.pages.push(page);
			} else {
				return Err(Error::from(ErrorKind::NotFound));
			}
		}

		Ok(sheet)
	}
}

// FileFindResult

pub struct FileFindResult<'a> {
	pub chunk: &'a SqPackChunk,
	pub entry: &'a HashTableEntry
}

impl FileFindResult<'_> {
	pub fn resolve(&self) -> String {
		self.chunk.dat_path(format!("dat{}", self.entry.file_id).as_str())
	}
}

// SqPackChunk

pub struct SqPackChunk {
	cat: u8,
	ex: u8,
	chunk: u8,
	index: SqPackIndex
}

impl SqPackChunk {
	pub fn new(cat: u8, ex: u8, chunk: u8, index: SqPackIndex) -> SqPackChunk {
		SqPackChunk {
			cat: cat,
			ex: ex,
			chunk: chunk,
			index: index
		}
	}

	pub fn hash(&self) -> u32 {
		((self.cat as u32) ^ (self.ex as u32) << 8 ^ (self.chunk as u32) << 16).into()
	}

	pub fn ex_dir(&self) -> String {
		if self.ex == 0 {
			"ffxiv".to_owned()
		} else {
			format!("ex{}", self.ex)
		}
	}

	pub fn dat_str(&self) -> String {
		internal::hex_str::<u8>(&[self.cat, self.ex, self.chunk])
	}

	pub fn dat_path(&self, ext: &str) -> String {
		format!("{}/{}.win32.{}", self.ex_dir(), self.dat_str(), ext)
	}
}

// Global

pub fn new(dir: &str) -> Result<SqPack, Error> {
	SqPack::new(dir)
}

pub fn load_all(dir: &str) -> Result<SqPack, Error> {
	let mut sqpack = new(dir)?;

	let path = Path::new(dir);

	let files = path.read_dir();
	for file in files.expect("read_dir call failed") {
		if let Ok(file) = file {
			let repo = file.path();
			sqpack.index_repo(repo.file_name().unwrap().to_str().unwrap());
		}
	}

	Ok(sqpack)
}

pub fn load_repo(dir: &str, repo: &str) -> Result<SqPack, Error> {
	let mut sqpack = new(dir)?;
	sqpack.index_repo(repo);
	Ok(sqpack)
}

// Category

pub fn category(name: &str) -> u8 {
	internal::CATEGORY[&name]
}