mod lib_test;

use lib_test::*;
use spreadsheet_ods::sheet::Grouped;
use spreadsheet_ods::{read_ods, OdsError, Sheet, WorkBook};

#[test]
fn test_write_group1() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(1, 0, 1);
    sh.set_value(2, 0, 1);
    sh.set_value(3, 0, 1);
    sh.set_value(4, 0, 1);
    sh.set_value(5, 0, 1);
    sh.set_value(6, 0, 1);
    sh.set_value(7, 0, 1);
    sh.set_value(8, 0, 1);
    sh.set_value(9, 0, 1);

    sh.add_row_group(1, 4);
    sh.add_row_group(1, 2);
    sh.add_row_group(1, 3);
    sh.add_row_group(4, 4);
    sh.add_row_group(6, 9);
    sh.set_row_group_displayed(6, 9, false);
    sh.add_row_group(7, 9);
    sh.add_row_group(8, 9);
    sh.add_row_group(9, 9);
    sh.add_row_group(40, 45);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_grouped_1.ods")?;

    let wb = read_ods("test_out/test_grouped_1.ods")?;
    let sh = wb.sheet(0);

    let v = sh.row_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 4, true)));
    assert!(v.contains(&Grouped::new(1, 2, true)));
    assert!(v.contains(&Grouped::new(1, 3, true)));
    assert!(v.contains(&Grouped::new(4, 4, true)));
    assert!(v.contains(&Grouped::new(6, 9, false)));
    assert!(v.contains(&Grouped::new(7, 9, true)));
    assert!(v.contains(&Grouped::new(8, 9, true)));
    assert!(v.contains(&Grouped::new(9, 9, true)));

    Ok(())
}

#[test]
fn test_write_group2() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(1, 1, 1);
    sh.set_value(2, 2, 1);
    sh.set_value(3, 3, 1);
    sh.set_value(4, 4, 1);
    sh.set_value(5, 5, 1);
    sh.set_value(6, 6, 1);
    sh.set_value(7, 7, 1);
    sh.set_value(8, 8, 1);
    sh.set_value(9, 9, 1);

    sh.add_col_group(1, 4);
    sh.add_col_group(1, 2);
    sh.add_col_group(1, 3);
    sh.add_col_group(4, 4);
    sh.add_col_group(6, 9);
    sh.set_col_group_displayed(6, 9, false);
    sh.add_col_group(7, 9);
    sh.add_col_group(8, 9);
    sh.add_col_group(9, 9);
    sh.add_col_group(40, 45);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_grouped_2.ods")?;

    let wb = read_ods("test_out/test_grouped_2.ods")?;
    let sh = wb.sheet(0);

    let v = sh.col_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 4, true)));
    assert!(v.contains(&Grouped::new(1, 2, true)));
    assert!(v.contains(&Grouped::new(1, 3, true)));
    assert!(v.contains(&Grouped::new(4, 4, true)));
    assert!(v.contains(&Grouped::new(6, 9, false)));
    assert!(v.contains(&Grouped::new(7, 9, true)));
    assert!(v.contains(&Grouped::new(8, 9, true)));
    assert!(v.contains(&Grouped::new(9, 9, true)));

    Ok(())
}

#[test]
#[should_panic]
fn test_write_group3() {
    let mut sh = Sheet::new("Sheet1");
    sh.add_col_group(4, 1);
}

#[test]
#[should_panic]
fn test_write_group4() {
    let mut sh = Sheet::new("Sheet1");
    sh.add_row_group(4, 1);
}

#[test]
#[should_panic]
fn test_write_group5() {
    let mut sh = Sheet::new("Sheet1");
    sh.add_col_group(1, 4);
    sh.add_col_group(2, 5);
}

#[test]
#[should_panic]
fn test_write_group6() {
    let mut sh = Sheet::new("Sheet1");
    sh.add_row_group(1, 4);
    sh.add_row_group(2, 5);
}
