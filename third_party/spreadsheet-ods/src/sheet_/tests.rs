use crate::sheet_::dedup_colheader;
use crate::Length;
use crate::Sheet;

#[test]
fn test_dedup_colheader() {
    let mut sh = Sheet::new("one");

    sh.set_col_width(0, cm!(2.54));
    sh.set_col_width(1, cm!(2.54));
    sh.set_col_width(2, cm!(2.54));
    sh.set_col_width(3, cm!(2.54));
    sh.set_col_width(4, cm!(5.08));
    sh.set_col_width(5, cm!(5));

    let _ = dedup_colheader(&mut sh);

    assert_eq!(sh.col_header.len(), 3);
}
