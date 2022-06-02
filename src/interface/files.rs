use crate::KtisisUI;

use eframe::egui;

use egui::{Vec2, RichText};

// FileUI

pub struct FileUI {}

impl FileUI {
	pub fn render(ktisis: &mut KtisisUI, ctx: &egui::Context) {
		// Left Panel

		egui::SidePanel::left("file_left")
		.min_width(200.0)
		.max_width(300.0)
		.show(ctx, |ui| {
			egui::Frame::none().inner_margin(Vec2 { x:0.0, y:10.0 }).show(ui, |ui| {
				ui.label("Sheet Name:");

				ui.text_edit_singleline(&mut ktisis.file_search);
				
				ui.separator();
			});
		});

		// Central Panel

		egui::CentralPanel::default().show(ctx, |ui| {
		});
	}
}