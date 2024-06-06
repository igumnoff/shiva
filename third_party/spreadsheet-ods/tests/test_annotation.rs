mod lib_test;

use lib_test::*;
use spreadsheet_ods::draw::Annotation;
use spreadsheet_ods::{OdsError, Sheet, WorkBook};

#[test]
fn test_annotation() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");
    let mut ann = Annotation::new_empty();
    // ann.set_name("ann1");
    ann.push_text_str("bla warg!");
    sh.set_annotation(0, 0, ann);

    let mut ann = Annotation::new_empty();
    // ann.set_name("ann2");
    ann.push_text_str("first");
    ann.push_text_str("second");
    sh.set_annotation(1, 1, ann);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_annotation.ods")?;

    Ok(())
}
