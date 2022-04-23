#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release

use exam_builder::app::App;

fn main() {
    eframe::run_native(Box::new(App::new()), eframe::NativeOptions::default());
}
