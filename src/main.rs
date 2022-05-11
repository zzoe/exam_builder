#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release

use exam_builder::app::{App, APP_NAME};

fn main() {
    eframe::run_native(
        APP_NAME,
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
