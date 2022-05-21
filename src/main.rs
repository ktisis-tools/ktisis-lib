// debugging file.

pub mod sqpack;

use sqpack::SqPack;

use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

use sqpack::reader::chunk::ChunkReader;

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

fn main() {
	//let repo = sqpack::load_repo(PATH, "ffxiv");
	let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	//let repo = sqpack::load_all(PATH);

	let mut sqpack = SqPack::new(PATH);
	//sqpack.index_category(sqpack::category("exd"));
	sqpack.index_category(sqpack::category("chara"));
	//sqpack.find_file("chara/xls/charamake/human.cmp");
	//sqpack.index_category(sqpack::category("bg"));

	let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	println!("Execution time: {}ms", end - start);

	//thread::sleep(time::Duration::from_millis(60000));
}