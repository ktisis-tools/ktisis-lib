// debugging file.

pub mod lib;
pub mod sqpack;
pub mod excel;

use sqpack::SqPack;

use std::fs;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

fn main() {
	let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	//let repo = sqpack::load_all(PATH);

	let mut sqpack = SqPack::new(PATH);
	sqpack.index_category(sqpack::category("exd"));

	//sqpack.get_file("exd/race.exh");

	//let file = sqpack.get_file("exd/root.exl");
	//fs::write("./result.txt", file.content).expect("oh no");

	sqpack.get_sheet("custom/000/CmnBhtEnterLv020St0003_00013");

	let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	println!("Execution time: {}ms", end - start);

	//thread::sleep(time::Duration::from_millis(60000));
}