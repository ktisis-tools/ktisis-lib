mod style;

use crate::sqpack::SqPack;
use crate::excel::sheet::ExcelSheet;

use eframe::egui;

use egui::Vec2;
use egui_extras::{Size, TableBuilder};

use std::cmp;

enum DemoType {
    Manual,
    ManyHomogenous,
    ManyHeterogenous,
}

// KtisisUI

pub struct KtisisUI {
	sqpack: SqPack,

	sheet_list: Vec<String>,
	sheet_current: (Option<String>, Option<ExcelSheet>),
	sheet_header: Vec<String>,

	sheet_search: String,
	num_rows: usize
}

impl KtisisUI {
	pub fn new(sqpack: SqPack) -> Self {
		let mut list = sqpack.get_sheet_list().expect("failed to read excel list");

		Self {
			sqpack: sqpack,

			sheet_list: list,
			sheet_current: (None, None),
			sheet_header: Vec::<String>::new(),

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

		let text_style = egui::TextStyle::Body;

		let left_panel = egui::SidePanel::left("left_panel").min_width(200.0).max_width(300.0);
		left_panel.show(ctx, |ui| {
			egui::Frame::none().inner_margin(Vec2 { x:0.0, y:10.0 }).show(ui, |ui| {
				ui.label("Sheet Name:");
				ui.text_edit_singleline(&mut self.sheet_search);
				ui.separator();
			});

			let total_rows = self.sheet_list.len();
			egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, ui.text_style_height(&text_style), total_rows, |ui, row_range| {
				let sel = match &self.sheet_current.0 {
					Some(name) => name.to_owned(),
					_ => "".to_string()
				};

				for row in row_range {
					let sheet = self.sheet_list.get(row).unwrap();

					if ui.selectable_label(sheet == &sel, sheet).clicked() {
						if let Ok(get) = self.sqpack.get_sheet(sheet) {

							let mut header = Vec::<String>::new();

							let mut i = 0;
							for column in &get.header.columns {
								header.push(format!("{}<{:?}>", i, column.data_type));
								i += 1;
							}

							self.sheet_current = (Some(sheet.to_string()), Some(get),);
							self.sheet_header = header;
						}
					}
				}
			});

			ui.separator();
		});

		let main_panel = egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(name) = &self.sheet_current.0 {
				ui.heading(name);
			}

			if let Some(sheet) = &mut self.sheet_current.1 {
				let total_rows = sheet.header.row_count as usize;
				egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, ui.text_style_height(&text_style), total_rows, |ui, row_range| {
					egui::Grid::new("sheet")
					.striped(true)
					.show(ui, |ui| {
						/*for column in &self.sheet_header {
							ui.label(column);
						}
						ui.end_row();*/
						for i in row_range {
							if let Ok(row) = sheet.get_row(sheet.start_id + i as u32) {
								for column in &row.columns {
									ui.label(column.get_string());
								}
								ui.end_row();
							}
						}
					});
				});
			}
		});
	}
}