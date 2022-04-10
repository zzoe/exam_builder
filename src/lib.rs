use docx_rs::{Run, RunFonts};

pub mod label;
pub mod question;

fn song() -> Run {
    Run::new().fonts(RunFonts::new().east_asia("宋体"))
}
