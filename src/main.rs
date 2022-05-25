// Modules

pub mod lib;
pub mod sqpack;
pub mod excel;
pub mod interface;

// Dependencies

use crate::interface::KtisisUI;

use eframe::egui;

// Constants

const PATH: &str = "D:/Program Files (x86)/SquareEnix/FINAL FANTASY XIV - A Realm Reborn/game/sqpack/";

// Main

fn main() {
	// SqPack

	let sqpack = sqpack::new(PATH).unwrap();

	// UI

	let options = eframe::NativeOptions::default();
	eframe::run_native(
		"Ktisis",
		options,
		Box::new(|_cc| Box::new(KtisisUI::new(sqpack)))
	);
}