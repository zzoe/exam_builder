use docx_rs::{Run, RunFonts};

pub mod error;
pub mod item_bank;
pub mod question;
pub mod title;

fn song() -> Run {
    Run::new().fonts(RunFonts::new().east_asia("宋体"))
}
