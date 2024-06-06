use crate::io::write::calc_col_headers;
use crate::sheet_::dedup_colheader;
use crate::Length;
use crate::{Sheet, WorkBook};
use icu_locid::locale;

fn setup_test_calc_col_headers() -> WorkBook {
    let mut wb = WorkBook::new(locale!("de_AT"));

    let mut sh0 = Sheet::new("1");
    sh0.set_col_width(0, cm!(2.54));
    sh0.set_col_width(1, cm!(2.54));
    sh0.set_col_width(2, cm!(2.54));
    sh0.set_col_width(3, cm!(2.54));
    sh0.set_col_width(4, cm!(2.54));
    sh0.set_col_width(5, cm!(2.54));
    sh0.set_col_width(6, cm!(2.54));
    dedup_colheader(&mut sh0).unwrap();

    wb.push_sheet(sh0);

    wb
}

#[test]
fn test_calc_col_headers() {
    let mut wb = setup_test_calc_col_headers();
    let sh0 = wb.sheet_mut(0);
    sh0.set_header_cols(0, 1);
    calc_col_headers(&mut wb).unwrap();

    let sh0 = wb.sheet_mut(0);
    assert!(sh0.col_header.get(&0).is_some());
    assert!(sh0.col_header.get(&2).is_some());
    assert_eq!(sh0.col_header.get(&0).unwrap().span, 2);
    assert_eq!(sh0.col_header.get(&2).unwrap().span, 5);

    let mut wb = setup_test_calc_col_headers();
    let sh0 = wb.sheet_mut(0);
    sh0.add_col_group(1, 3);
    sh0.add_col_group(4, 6);
    calc_col_headers(&mut wb).unwrap();

    let sh0 = wb.sheet_mut(0);
    assert!(sh0.col_header.get(&0).is_some());
    assert!(sh0.col_header.get(&1).is_some());
    assert!(sh0.col_header.get(&4).is_some());
    assert_eq!(sh0.col_header.get(&0).unwrap().span, 1);
    assert_eq!(sh0.col_header.get(&1).unwrap().span, 3);
    assert_eq!(sh0.col_header.get(&4).unwrap().span, 3);

    let mut wb = setup_test_calc_col_headers();
    let sh0 = wb.sheet_mut(0);
    sh0.add_col_group(4, 9);
    calc_col_headers(&mut wb).unwrap();

    let sh0 = wb.sheet_mut(0);
    assert!(sh0.col_header.get(&0).is_some());
    assert!(sh0.col_header.get(&4).is_some());
    assert_eq!(sh0.col_header.get(&0).unwrap().span, 4);
    assert_eq!(sh0.col_header.get(&4).unwrap().span, 3);

    let mut wb = WorkBook::new(locale!("de_AT"));
    let mut sh0 = Sheet::new("1");
    sh0.add_col_group(4, 9);
    calc_col_headers(&mut wb).unwrap();
    wb.push_sheet(sh0);

    let sh0 = wb.sheet_mut(0);
    assert!(sh0.col_header.is_empty());
}
