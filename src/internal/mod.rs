pub mod crc32;
pub mod reader;

use phf::phf_map;

// Constants

pub static CATEGORY: phf::Map<&'static str, u8> = phf_map! {
	"common"		=> 0,
	"bgcommon"		=> 1,
	"bg"			=> 2,
	"cut"			=> 3,
	"chara"			=> 4,
	"shader"		=> 5,
	"ui"			=> 6,
	"sound"			=> 7,
	"vfx"			=> 8,
	"ui_script"		=> 9,
	"exd"			=> 10,
	"game_script"   => 11,
	"music"			=> 12,
	"sqpack_test"   => 18,
	"debug"			=> 19
};

// Methods

pub fn hex_str<T: std::fmt::LowerHex>(args: &[T]) -> String {
	let mut res: String = "".to_string();
	for i in args {
		res += format!("{:02x}", i).as_str();
	}
	return res;
}

pub fn dat_str(cat: u8, ex: u8, chunk: u8, ftype: &str) -> String {
	format!("{}.{}.{}", hex_str::<u8>(&[cat, ex, chunk]), "win32", ftype)
}

pub fn parse_dat_stem(name: &str) -> [u8; 3] {
	let split = name.find(".").unwrap();
	let dat = &name[..split];
	[
		u8::from_str_radix(&dat[0..2], 16).unwrap(),
		u8::from_str_radix(&dat[2..4], 16).unwrap(),
		u8::from_str_radix(&dat[4..6], 16).unwrap()
	]
}

pub fn parse_repo(name: &str) -> u8 {
	if name == "ffxiv" { 0 }
	else if &name[..2] == "ex" {
		name[2..].parse::<u8>().unwrap()
	} else {
		panic!("Invalid repo name: {name}")
	}
}

pub fn hash_path(path: &str) -> u64 {
	let lower: &str = &path.to_lowercase();
	let last = lower.rfind("/").unwrap();

	let dir = crc32::hash(&lower[..last]) as u64;
	let file = crc32::hash(&lower[last+1..]) as u64;

	return ((dir << 32) | file).into();
}