use exam_builder::item_bank;

fn main() {
    if let Err(e) = item_bank::Builder::new("test.xlsx").build() {
        eprintln!("{:?}", e);
    }
}
