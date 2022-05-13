use eframe::{egui, Frame, Storage, APP_KEY};
use exam_builder::ui::ExamBuilder;

#[derive(Default)]
pub struct App {
    exam_builder: ExamBuilder,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.exam_builder.view(ctx);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, APP_KEY, &self.exam_builder);
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // cc.egui_ctx.set_debug_on_hover(true);

        let mut slf = Self::default();
        if let Some(storage) = cc.storage {
            slf.exam_builder = eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }

        slf
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "consola".to_owned(),
        egui::FontData::from_static(include_bytes!("../resource/consola.ttf")),
    );
    fonts.font_data.insert(
        "simkai".to_owned(),
        egui::FontData::from_static(include_bytes!("../resource/simkai.ttf")),
    );

    let entry = fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    let entry = fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default();
    entry.push("consola".to_owned());
    entry.push("simkai".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
