use phf::phf_map;
use std::path::Path;

// Constants

pub static CATEGORIES: phf::Map<&'static str, u8> = phf_map! {
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

pub fn hex_str(args: &[i32]) -> String {
	let mut res: String = "".to_string();
	for i in args {
		res += format!("{:02}", i).as_str();
	}
	return res;
}

pub fn dat_str(cat: i32, ex: i32, chunk: i32, ftype: &str, plat: &str) -> String {
	return format!("{}.{}.{}", hex_str(&[cat, ex, chunk]), ftype, plat);
}

pub fn parse_dat(path: &Path) -> [&str; 3] {
	let name = path.file_name().unwrap().to_str().unwrap();

	let left = name.find(".").unwrap();
	let right = name.rfind(".").unwrap();

	let chunk = &name[..left];
	let plat = &name[left+1..right];
	let ext = &name[right+1..];

	return [chunk, plat, ext];
}