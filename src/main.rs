#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// hide console window on Windows in release

mod app;

fn main() {
    eframe::run_native(
        "试卷生成器",
        eframe::NativeOptions::default(),
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
