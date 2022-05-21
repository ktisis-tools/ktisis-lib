pub mod sqpack;

use std::{thread, time};

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

fn main() {
	//let repo = sqpack::load_repo(PATH, "ffxiv");
	let repo = sqpack::load_all(PATH);
	thread::sleep(time::Duration::from_millis(30000));
}