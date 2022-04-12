use exam_builder::item_bank;

#[test]
fn question() {
    let res = item_bank::Builder::new("test.xlsx").build();
    assert!(res.is_ok());
}
