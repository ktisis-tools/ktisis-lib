mod style;
mod sheets;

use crate::sqpack::SqPack;
use crate::excel::Language;
use crate::excel::sheet::ExcelSheet;

use sheets::SheetUI;

use native_dialog::{MessageDialog, MessageType};

use eframe::egui;
use egui::style::Margin;

// KtisisUI

enum KtisisView {
	Files,
	Sheets
}

pub struct KtisisUI {
	sqpack: SqPack,

	_init: bool,

	view: KtisisView,

	sheet_list: Vec<String>,
	sheet_current: Option<ExcelSheet>,
	sheet_name: String,
	sheet_header: Vec<String>,
	sheet_change_language: Option<Language>,
	column_display: Vec<f32>,

	sheet_search: String
}

// App Frame

impl KtisisUI {
	pub fn new(sqpack: SqPack) -> Self {
		let list = sqpack.get_sheet_list().expect("failed to read excel list");

		Self {
			sqpack: sqpack,

			_init: false,

			view: KtisisView::Sheets,

			sheet_list: list,
			sheet_current: None,
			sheet_name: "".to_string(),
			sheet_header: Vec::<String>::new(),
			sheet_change_language: None,
			column_display: Vec::<f32>::new(),

			sheet_search: "".to_string()
		}
	}

	fn no_impl(&mut self) {}

	fn error(&mut self, err: String) {
		MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("Error")
		.set_text(&err)
		.show_alert()
		.expect("encountered an error while trying to display an error");
	}

	// Sheets

	fn get_sheet(&mut self, sheet: &str) -> bool {
		if let Ok(get) = self.sqpack.get_sheet(sheet) {
			let mut header = Vec::<String>::new();

			for i in 0..get.header.columns.len() {
				let column = get.header.columns.get(i as usize).unwrap();
				header.push(format!("{}<{:?}>", i, column.data_type));
			}

			self.sheet_current = Some(get);
			self.sheet_name = sheet.to_string();
			self.sheet_header = header;

			true
		} else {
			false
		}
	}
}

impl eframe::App for KtisisUI {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if !self._init {
			// Style
			ctx.set_style(style::get_style());

			// Font
			let mut fonts = egui::FontDefinitions::default();
			ctx.set_fonts(fonts);

			// Finish
			self._init = true;
		}

		egui::TopBottomPanel::top("top_panel")
		.frame(egui::Frame::none().inner_margin(Margin {
			left: 5.0,
			right: 0.0,
			top: 8.0,
			bottom: 5.0
		}))
		.show(ctx, |ui| {
			ui.horizontal(|ui| {
				if ui.button("Files").clicked() {
					self.view = KtisisView::Files;
				}
				if ui.button("Sheets").clicked() {
					self.view = KtisisView::Sheets;
				}
			});
		});

		match &self.view {
			KtisisView::Sheets => SheetUI::render(self, ctx),
			_ => self.no_impl()
		};
	}
}