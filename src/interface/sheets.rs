use crate::KtisisUI;

use eframe::egui;

use egui::{Vec2, RichText};
use egui_extras::{Size, TableBuilder};

// SheetUI

pub struct SheetUI {}

impl SheetUI {
	pub fn render(ktisis: &mut KtisisUI, ctx: &egui::Context) {
		// Change Language

		if let Some(language) = ktisis.sheet_change_language {
			let cur = ktisis.sqpack.language;

			ktisis.sqpack.set_language(language);
			let result = ktisis.get_sheet(&ktisis.sheet_name.to_owned());

			if !result {
				println!("language switch failed. going back to {:?}", cur);
				ktisis.sqpack.set_language(cur);

				ktisis.error(format!(
					"Failed to load language: {:?}\nSome languages (i.e. Korean, Chinese) require a special installation of the game.",
					language
				));
			}

			ktisis.sheet_change_language = None;
		}

		// Left Panel

		let text_style = egui::TextStyle::Body;

		egui::SidePanel::left("sheet_left")
		.min_width(200.0)
		.max_width(300.0)
		.show(ctx, |ui| {
			// Displayed Sheet List

			let mut lref = &mut ktisis.sheet_list;
			if ktisis._search_len > 0 {
				lref = &mut ktisis._search_res;
			}
			let list = lref.to_owned();

			// Sheet Search

			egui::Frame::none().inner_margin(Vec2 { x:0.0, y:5.0 }).show(ui, |ui| {
				ui.label("Sheet Name:");

				let search = ui.text_edit_singleline(&mut ktisis.sheet_search);
				if search.changed() {
					let len = ktisis.sheet_search.len() as u16;
					if len == 0 {
						ktisis._search_res.retain(|_| false);
					} else {
						let mut tar = &mut ktisis._search_res;
						if len < ktisis._search_len || ktisis._search_len == 0 {
							tar = &mut ktisis.sheet_list;
						}
						ktisis._search_res = tar.iter().filter(|name| {
							name.to_lowercase().contains(&ktisis.sheet_search.to_lowercase())
						}).cloned().collect();
					}
					ktisis._search_len = len;
				}

				ui.separator();

				// Select All / Export

				ui.horizontal(|ui| {
					let all_selected = &mut ktisis.sheet_selected_all;
					if ui.checkbox(all_selected, "").clicked() {
						// TODO: Not this. Reverse behaviour of sheet_selected while sheet_selected_all is true.
						if all_selected == &false {
							ktisis.sheet_selected.retain(|_| false);
						} else {
							ktisis.sheet_selected = list.to_owned();
						}
					}

					let selected_ct = ktisis.sheet_selected.len();
					ui.label(format!(
						"{selected_ct} sheet{} selected",
						if selected_ct == 1 { "" } else { "s" }
					));

					ui.with_layout(egui::Layout::right_to_left(), |ui| {
						let button = egui::Button::new("Export");
						if ui.add_enabled(selected_ct > 0, button).clicked() {

						}
					});
				});

				ui.separator();
			});

			// Sheet List

			let total_rows = list.len();
			egui::ScrollArea::vertical().auto_shrink([false; 2]).show_rows(ui, ui.text_style_height(&text_style), total_rows, |ui, row_range| {
				for row in row_range {
					let sheet = list.get(row).unwrap();

					ui.horizontal(|ui| {
						let index = ktisis.sheet_selected.iter().position(|x| x == &sheet.to_owned());
						let is_selected = index.is_some();
						if ui.checkbox(&mut is_selected.to_owned(), "").clicked() {
							if is_selected {
								ktisis.sheet_selected.remove(index.unwrap());
								if ktisis.sheet_selected_all {
									ktisis.sheet_selected_all = false;
								}
							} else {
								ktisis.sheet_selected.push(sheet.to_owned());
							}
						}
						if ui.selectable_label(sheet == &ktisis.sheet_name, sheet).clicked() {
							ktisis.get_sheet(&sheet.to_owned());
						}
					});
				}
			});
		});

		// Central Panel

		egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(sheet) = &mut ktisis.sheet_current {
				ui.label(RichText::new(ktisis.sheet_name.to_owned()).heading().strong());

				ui.horizontal(|ui| {
					// Select Language

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
							}
						}
					});
				});

				// Display Table

				ui.separator();

				let total_rows = sheet.header.row_count as usize;
				let total_cols = ktisis.sheet_header.len();

				let text_height = ui.text_style_height(&text_style);

				let offset = ui.min_rect().min.x;
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

									/*
										there's some funky shit going on here with spacing between columns
										so this is mildly inaccurate, but not enough to make a big impact on performance
										should revisit this at some point
									*/
									if total_width + offset <= rect.min.x - offset {
										/*
											is there a better alternative to this?
											feels like there should be a method to specify an offset to draw columns after, but can find none in docs.
										*/
										table_row.col(|_ui| {});
										continue;
									} else if total_width - width >= rect.max.x {
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