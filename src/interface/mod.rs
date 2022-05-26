mod style;

use crate::sqpack::SqPack;
use crate::excel::sheet::ExcelSheet;

use eframe::egui;

use egui::{Vec2, RichText};
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
	column_display: Vec<f32>,

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

							for i in 0..get.header.columns.len() {
								let column = get.header.columns.get(i as usize).unwrap();
								header.push(format!("{}<{:?}>", i, column.data_type));
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
			if let Some(sheet) = &mut self.sheet_current.1 {
				let name = self.sheet_current.0.as_ref().unwrap();
				//ui.heading(name);
				ui.label(RichText::new(name).heading().strong());

				let total_rows = sheet.header.row_count as usize;
				let total_cols = self.sheet_header.len();

				let text_height = ui.text_style_height(&text_style);
				egui::ScrollArea::horizontal().id_source(name).auto_shrink([false; 2]).show(ui, |ui| {
					TableBuilder::new(ui)
					.striped(true)
					.resizable(true)
					.columns(Size::remainder().at_least(100.0), total_cols)
					.cell_layout(egui::Layout::left_to_right().with_main_wrap(false))
					.header(text_height, |mut header| {
						for column in &self.sheet_header {
							header.col(|ui| {
								ui.strong(column);
							});
						}
					})
					.body(|mut body| {
						body.rows(text_height, total_rows, |row_index, mut table_row| {
							if let Ok(row) = sheet.get_row(sheet.start_id + row_index as u32) {
								for column in &row.columns {
									table_row.col(|ui| {
										ui.label(column.get_string());
									});
								}
							}
						});

							/*for i in row_range {
								if let Ok(row) = sheet.get_row(sheet.start_id + i as u32) {
									body.row(text_height, |mut table_row| {
										for column in &row.columns {
											table_row.col(|ui| {
												ui.label(column.get_string());
											});
										}
									});
								}
							}*/
					});
				});

				/*egui::ScrollArea::both().auto_shrink([false; 2]).show_rows(ui, ui.text_style_height(&text_style), total_rows, |ui, row_range| {
					egui::Grid::new("sheet")
					.striped(true)
					.max_col_width(700.0)
					.show(ui, |ui| {
						if row_range.start == 0 {
							for column in &self.sheet_header {
								ui.label(column);
							}
							ui.end_row();
						}
						for i in row_range {
							if let Ok(row) = sheet.get_row(sheet.start_id + i as u32) {
								for column in &row.columns {
									ui.label(column.get_string());
								}
								ui.end_row();
							}
						}
					});
				});*/
			}
		});
	}
}