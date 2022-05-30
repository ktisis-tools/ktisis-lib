mod style;
mod sheets;

use crate::sqpack::SqPack;
use crate::excel::sheet::ExcelSheet;

use sheets::SheetUI;

use eframe::egui;

use egui::style::Margin;

// KtisisUI

enum KtisisPage {
	Sheets = 1
}

pub struct KtisisUI {
	sqpack: SqPack,

	page: KtisisPage,

	sheet_list: Vec<String>,
	sheet_current: (Option<String>, Option<ExcelSheet>),
	sheet_header: Vec<String>,
	column_display: Vec<f32>,

	sheet_search: String,
	num_rows: usize
}

impl KtisisUI {
	pub fn new(sqpack: SqPack) -> Self {
		let list = sqpack.get_sheet_list().expect("failed to read excel list");

		Self {
			sqpack: sqpack,

			page: KtisisPage::Sheets,

			sheet_list: list,
			sheet_current: (None, None),
			sheet_header: Vec::<String>::new(),
			column_display: Vec::<f32>::new(),

			// UI Values
			sheet_search: "".to_string(),
			num_rows: 32
		}
	}
}

// App Frame

impl eframe::App for KtisisUI {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		ctx.set_style(style::get_style());

		let m = Margin {
			left: 5.0,
			right: 0.0,
			top: 8.0,
			bottom: 5.0
		};
		egui::TopBottomPanel::top("top_panel")
		.frame(egui::Frame::none().inner_margin(m))
		.show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui.button("Files").clicked() {
				}
				if ui.button("Sheets").clicked() {
					self.page = KtisisPage::Sheets;
				}
			});
		});

		match self.page {
			KtisisPage::Sheets => SheetUI::render(self, ctx)
		};
	}
}