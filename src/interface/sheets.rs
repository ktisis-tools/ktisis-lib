use crate::KtisisUI;

use eframe::egui;

use egui::{Vec2, RichText};
use egui_extras::{Size, TableBuilder};

pub struct SheetUI {}

impl SheetUI {
	pub fn render(ktisis: &mut KtisisUI, ctx: &egui::Context) {
		if let Some(language) = ktisis.sheet_change_language {
			let cur = ktisis.sqpack.language;

			ktisis.sqpack.set_language(language);
			let result = ktisis.get_sheet(&ktisis.sheet_name.to_owned());

			if !result {
				println!("language switch failed. going back to {:?}", cur);
				ktisis.sqpack.set_language(cur);

				ktisis.error(format!(
					"Failed to load language: {:?}\nSome languages (i.e. Korean, Chinese) require a unique installation of the game.",
					language
				));
			}

			ktisis.sheet_change_language = None;
		}

		let text_style = egui::TextStyle::Body;

		egui::SidePanel::left("left_panel").min_width(200.0).max_width(300.0)
		.show(ctx, |ui| {
			egui::Frame::none().inner_margin(Vec2 { x:0.0, y:10.0 }).show(ui, |ui| {
				ui.label("Sheet Name:");
				ui.text_edit_singleline(&mut ktisis.sheet_search);
				ui.separator();
			});

			let total_rows = ktisis.sheet_list.len();
			egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, ui.text_style_height(&text_style), total_rows, |ui, row_range| {
				for row in row_range {
					let sheet = ktisis.sheet_list.get(row).unwrap();

					if ui.selectable_label(sheet == &ktisis.sheet_name, sheet).clicked() {
						ktisis.get_sheet(&sheet.to_owned());
					}
				}
			});

			ui.separator();
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(sheet) = &mut ktisis.sheet_current {
				//ui.heading(name);
				ui.label(RichText::new(ktisis.sheet_name.to_owned()).heading().strong());

				egui::ComboBox::from_id_source("sheet_language")
				.selected_text(format!("{:?}", sheet.language))
				.show_ui(ui, |ui| {
					for language in &sheet.header.languages {
						let click = ui.selectable_label(
							language == &sheet.language,
							format!("{:?}", language
						));

						if click.clicked() {
							ktisis.sheet_change_language = Some(*language);
							//ktisis.sqpack.set_language(*language);
							//ktisis.get_sheet(name);
						}
					}
				});

				ui.separator();

				let total_rows = sheet.header.row_count as usize;
				let total_cols = ktisis.sheet_header.len();

				let text_height = ui.text_style_height(&text_style);
				egui::ScrollArea::horizontal().id_source(ktisis.sheet_name.to_owned()).auto_shrink([false; 2]).show_viewport(ui, |ui, rect| {
					TableBuilder::new(ui)
					.striped(true)
					.resizable(true)
					.columns(Size::remainder().at_least(100.0), total_cols + 1)
					.cell_layout(egui::Layout::left_to_right().with_main_wrap(false))
					.header(text_height, |mut header| {
						header.col(|ui| {
							ui.strong("Key");
						});

						for column in &ktisis.sheet_header {
							header.col(|ui| {
								ui.strong(column);
							});
						}
					})
					.body(|body| {
						let widths = body.widths().to_owned();
						body.rows(text_height, total_rows, |row_index, mut table_row| {
							if let Ok(row) = sheet.get_row(sheet.start_id + row_index as u32) {
								let mut total_width = 0.0;
								for i in 0..row.columns.len()+1 {
									let width = widths[i];
									total_width += width;
									if total_width + width*2.0 < rect.min.x - width*2.0 {
										table_row.col(|_ui| {});
										continue;
									} else if total_width > rect.max.x {
										break;
									}

									if i == 0 {
										table_row.col(|ui| {
											ui.label(format!("{row_index}"));
										});
									} else {
										let column = &row.columns[i - 1];
										table_row.col(|ui| {
											ui.label(column.get_string());
										});
									}
								}
							}
						});
					});
				});
			}
		});
	}
}