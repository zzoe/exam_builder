use crate::item_bank;
use eframe::{
    egui::{self, Color32, Context, RichText, Ui},
    epi::{self, file_storage::FileStorage, Frame, Storage},
};
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "试卷生成器";

#[derive(Serialize, Deserialize)]
pub struct App {
    picked_path: Vec<(String, String)>,
}

impl Default for App {
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

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
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

    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        ctx.set_visuals(egui::Visuals::dark());
        // ctx.set_debug_on_hover(true);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        APP_NAME
    }
}

impl App {
    pub fn new() -> Self {
        FileStorage::from_app_name(APP_NAME)
            .and_then(|storage| epi::get_value(&storage, epi::APP_KEY))
            .unwrap_or_default()
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

        // let open_btn = Button::new("Open File").fill(Color32::BLUE).ui(ui);
        // if open_btn.clicked() {
        // }

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
