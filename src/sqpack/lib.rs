use phf::phf_map;

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

// Platform Enums

enum Platform {
	Win32,
	Ps3,
	Ps4
}

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

pub fn parse_dat_stem(name: &str) -> [u8; 3] {
	let split = name.find(".").unwrap();

	let dat = &name[..split];
	let plat = &name[split+1..];

	let cat = u8::from_str_radix(&dat[0..2], 16).unwrap();
	let ex  = u8::from_str_radix(&dat[2..4], 16).unwrap();
	let chk = u8::from_str_radix(&dat[4..6], 16).unwrap();

	return [cat, ex, chk]; // TODO: Optimise determining platform
}