use crate::item_bank;
use eframe::egui;
use eframe::egui::{Color32, RichText, Ui};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExamBuilder {
    picked_path: Vec<(String, String)>,
}

impl Default for ExamBuilder {
    fn default() -> Self {
        Self {
            picked_path: vec![
                ("Junior".to_string(), String::new()),
                ("Senior".to_string(), String::new()),
                ("Master".to_string(), String::new()),
            ],
        }
    }
}

impl ExamBuilder {
    pub fn new(picked_path: Vec<(String, String)>) -> Self {
        if picked_path.is_empty() {
            Self::default()
        } else {
            Self { picked_path }
        }
    }

    pub fn view(&mut self, ctx: &egui::Context) {
        // println!("{:?}", self.picked_path);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("grid")
                // .num_columns(4)
                // .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.picked_path.iter_mut().for_each(|(label, path)| {
                        Self::add_line(ui, label, path);
                    })
                });
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });
    }

    fn add_line(ui: &mut Ui, label: &str, path: &mut String) {
        let mut label = RichText::new(label);
        if ui.style().visuals.dark_mode {
            label = label.color(Color32::LIGHT_YELLOW);
        }
        ui.label(label);

        if !path.is_empty() {
            ui.monospace(path.as_str());
        }

        let mut sel_btn = RichText::new("Select");
        if ui.style().visuals.dark_mode {
            sel_btn = sel_btn.color(Color32::LIGHT_BLUE);
        }
        if ui
            .button(sel_btn)
            .on_hover_text("Selelct a question pool")
            .clicked()
        {
            if let Some(picked_path) = rfd::FileDialog::new().pick_file() {
                *path = picked_path.display().to_string();
            }
        }

        let mut gen_btn = RichText::new("Generate");
        if ui.style().visuals.dark_mode {
            gen_btn = gen_btn.color(Color32::LIGHT_GREEN);
        }
        if !path.is_empty()
            && ui
                .button(gen_btn)
                .on_hover_text("Generate a exam from the selected question pool")
                .clicked()
        {
            if let Err(e) = item_bank::Builder::new(path.as_str()).build() {
                eprintln!("{:?}", e);
            }
        }

        ui.end_row();
    }
}
