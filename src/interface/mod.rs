mod style;

use crate::sqpack::SqPack;

use eframe::egui;

use egui::Vec2;
use egui_extras::{Size, TableBuilder};

enum DemoType {
    Manual,
    ManyHomogenous,
    ManyHeterogenous,
}

// KtisisUI

pub struct KtisisUI {
	sqpack: SqPack,

	sheet_search: String,
	num_rows: usize
}

impl KtisisUI {
	pub fn new(sqpack: SqPack) -> Self {
		Self {
			sqpack: sqpack,

			// UI Values
			sheet_search: "".to_string(),
			num_rows: 32
		}
	}
}

// App Frame

impl eframe::App for KtisisUI {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		/*egui::TopBottomPanel::top("top").show(ctx, |ui| {
			
		});*/

		ctx.set_style(style::get_style());

		let left_panel = egui::SidePanel::left("left_panel").min_width(200.0).max_width(300.0);
		left_panel.show(ctx, |ui| {
			egui::Frame::none().inner_margin(Vec2 { x:0.0, y:10.0 }).show(ui, |ui| {
				ui.label("Sheet Name:");
				ui.text_edit_singleline(&mut self.sheet_search);
				ui.separator();
			});

			egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
				for i in 0..50 {
					ui.selectable_label(false, "SampleSheet");
				}
				ui.selectable_label(true, "SampleSheetSelected");
				for i in 0..100 {
					ui.selectable_label(false, "SampleSheet");
				}
			});

			ui.separator();
		});

		let main_panel = egui::CentralPanel::default().show(ctx, |ui| {
			TableBuilder::new(ui)
				.striped(true)
				.cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
				.column(Size::initial(60.0).at_least(40.0))
				.column(Size::initial(60.0).at_least(40.0))
				.column(Size::remainder().at_least(60.0))
				.header(20.0, |mut header| {
					header.col(|ui| {
						ui.heading("Col1");
					});
					header.col(|ui| {
						ui.heading("Col2");
					});
					header.col(|ui| {
						ui.heading("Col3");
					});
				})
				.body(|mut body| {
					for row_index in 0..20 {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label("test".to_string());
                            });
                            row.col(|ui| {
                                ui.label("123".to_string());
                            });
                            row.col(|ui| {
                                ui.label("a");
                            });
                        });
                    }
				});;
		});

		/*egui::SidePanel::left("asdf").show(ctx, |ui| {
			ui.heading("Ktisis");
			ui.horizontal(|ui| {
				ui.label("TEST")
			});
			if ui.button("Test").clicked() {
				println!("!!!");
			}
			ui.label("test");
		});*/
	}
}