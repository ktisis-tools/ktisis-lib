pub mod sqpack;

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

fn main() {
	//let repo = sqpack::load_repo(PATH, "ffxiv");
	//let repo = sqpack::load_all(PATH);

	let mut sqpack = sqpack::new(PATH);
	sqpack.index_category(sqpack::category("exd"));
	//sqpack.index_category(sqpack::category("bg"));
}