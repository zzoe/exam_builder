use crate::item_bank;
use eframe::egui::{Context, Ui};
use eframe::epi::file_storage::FileStorage;
use eframe::epi::{Frame, Storage};
use eframe::{egui, epi};
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
                ("junior".to_string(), String::new()),
                ("senior".to_string(), String::new()),
                ("master".to_string(), String::new()),
            ],
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.picked_path.iter_mut().for_each(|(label, path)| {
                Self::ui(ui, label, path);
                if "master".ne(label) {
                    ui.separator();
                }
            })
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

    fn ui(ui: &mut Ui, label: &str, path: &mut String) {
        ui.horizontal(|ui| {
            ui.label(label);

            if !path.is_empty() {
                ui.monospace(path.as_str());
            }

            if ui.button("Open file…").clicked() {
                if let Some(picked_path) = rfd::FileDialog::new().pick_file() {
                    *path = picked_path.display().to_string();
                }
            }

            if !path.is_empty() && ui.button("generate").clicked() {
                if let Err(e) = item_bank::Builder::new(path.as_str()).build() {
                    eprintln!("{:?}", e);
                }
            }
        });
    }
}
