pub mod files;
pub mod structs;
pub mod sheet;

use phf::phf_map;

// Language Enum / LANGUAGE map

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Language {
	None = 0,
	Japanese = 1,
	English = 2,
	German = 3,
	French = 4,
	ChineseSimplified = 5,
	ChineseTraditional = 6,
	Korean = 7
}

pub static LANGUAGE: phf::Map<u16, (&'static str, Language)> = phf_map! {
	0u16 => ( "" , Language::None),
	1u16 => ("ja", Language::Japanese),
	2u16 => ("en", Language::English),
	3u16 => ("de", Language::German),
	4u16 => ("fr", Language::French),
	5u16 => ("chs", Language::ChineseSimplified),
	6u16 => ("cht", Language::ChineseTraditional),
	7u16 => ("ko", Language::Korean)
};

impl Language {
	pub fn from_u16(i: u16) -> Language {
		LANGUAGE[&i].1
	}

	pub fn resolve(self) -> &'static str {
		LANGUAGE.entries.iter().find(|x| x.1.1 == self).unwrap().1.0
	}

	pub fn suffix(&self) -> String {
		let resolve = self.resolve();
		if resolve == "" {
			resolve.to_string()
		} else {
			format!("_{}", resolve)
		}
	}
}