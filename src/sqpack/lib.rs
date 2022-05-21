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

// Platform Enums

enum Platform {
	Win32,
	Ps3,
	Ps4
}

// Methods

pub fn hex_str<T: std::fmt::LowerHex>(args: &[T]) -> String {
	let mut res: String = "".to_string();
	for i in args {
		res += format!("{:02x}", i).as_str();
	}
	return res;
}

pub fn dat_str(cat: u8, ex: u8, chunk: u8, ftype: &str) -> String {
	return format!("{}.{}.{}", hex_str::<u8>(&[cat, ex, chunk]), "win32", ftype);
}

pub fn parse_dat_stem(name: &str) -> [u8; 3] {
	let split = name.find(".").unwrap();

	let dat = &name[..split];
	let _plat = &name[split+1..];

	let cat = u8::from_str_radix(&dat[0..2], 16).unwrap();
	let ex  = u8::from_str_radix(&dat[2..4], 16).unwrap();
	let chk = u8::from_str_radix(&dat[4..6], 16).unwrap();

	return [cat, ex, chk]; // TODO: platform
}

pub fn parse_repo(name: &str) -> u8 {
	if name == "ffxiv" {
		return 0;
	} else if &name[..2] == "ex" {
		return name[2..].parse::<u8>().unwrap();
	}
	panic!("Invalid repo name: {name}");
}