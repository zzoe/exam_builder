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
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // cc.egui_ctx.set_debug_on_hover(true);

        let mut slf = Self::default();
        if let Some(storage) = cc.storage {
            slf.exam_builder = eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }

        slf
    }
}
