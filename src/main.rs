// debugging file.

pub mod lib;
pub mod sqpack;
pub mod excel;

use std::fs;
use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

fn main() {
	let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	//let repo = sqpack::load_all(PATH);

	let mut sqpack = sqpack::new(PATH).unwrap();
	sqpack.index_category(sqpack::category("exd"));

	sqpack.get_sheet("Race");

	let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

	println!("Execution time: {}ms", end - start);

	//thread::sleep(time::Duration::from_millis(60000));
}