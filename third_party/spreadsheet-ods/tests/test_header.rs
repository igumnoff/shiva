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

    sh.set_header_rows(1, 3);
    sh.add_row_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_1.ods")?;

    let wb = read_ods("test_out/test_header_1.ods")?;
    let sh = wb.sheet(0);

    let v = sh.row_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_rows() {
        assert!(h.from == 1 && h.to == 3);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_group2() -> Result<(), OdsError> {
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

    sh.set_header_rows(0, 3);
    sh.add_row_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_2.ods")?;

    let wb = read_ods("test_out/test_header_2.ods")?;
    let sh = wb.sheet(0);

    let v = sh.row_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_rows() {
        assert!(h.from == 0 && h.to == 3);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_group3() -> Result<(), OdsError> {
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

    sh.set_header_rows(0, 4);
    sh.add_row_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_3.ods")?;

    let wb = read_ods("test_out/test_header_3.ods")?;
    let sh = wb.sheet(0);

    let v = sh.row_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_rows() {
        assert!(h.from == 0 && h.to == 4);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_group4() -> Result<(), OdsError> {
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

    sh.set_header_rows(2, 4);
    sh.add_row_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_4.ods")?;

    let wb = read_ods("test_out/test_header_4.ods")?;
    let sh = wb.sheet(0);

    let v = sh.row_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_rows() {
        assert!(h.from == 2 && h.to == 4);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_colgroup1() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 1);
    sh.set_value(0, 2, 1);
    sh.set_value(0, 3, 1);
    sh.set_value(0, 4, 1);
    sh.set_value(0, 5, 1);
    sh.set_value(0, 6, 1);
    sh.set_value(0, 7, 1);
    sh.set_value(0, 8, 1);
    sh.set_value(0, 9, 1);

    sh.set_header_cols(1, 3);
    sh.add_col_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_5.ods")?;

    let wb = read_ods("test_out/test_header_5.ods")?;
    let sh = wb.sheet(0);

    let v = sh.col_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_cols() {
        assert!(h.from == 1 && h.to == 3);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_colgroup2() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 1);
    sh.set_value(0, 2, 1);
    sh.set_value(0, 3, 1);
    sh.set_value(0, 4, 1);
    sh.set_value(0, 5, 1);
    sh.set_value(0, 6, 1);
    sh.set_value(0, 7, 1);
    sh.set_value(0, 8, 1);
    sh.set_value(0, 9, 1);

    sh.set_header_cols(0, 3);
    sh.add_col_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_6.ods")?;

    let wb = read_ods("test_out/test_header_6.ods")?;
    let sh = wb.sheet(0);

    let v = sh.col_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_cols() {
        assert!(h.from == 0 && h.to == 3);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_colgroup3() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 1);
    sh.set_value(0, 2, 1);
    sh.set_value(0, 3, 1);
    sh.set_value(0, 4, 1);
    sh.set_value(0, 5, 1);
    sh.set_value(0, 6, 1);
    sh.set_value(0, 7, 1);
    sh.set_value(0, 8, 1);
    sh.set_value(0, 9, 1);

    sh.set_header_cols(0, 4);
    sh.add_col_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_7.ods")?;

    let wb = read_ods("test_out/test_header_7.ods")?;
    let sh = wb.sheet(0);

    let v = sh.col_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_cols() {
        assert!(h.from == 0 && h.to == 4);
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_write_colgroup4() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 1);
    sh.set_value(0, 2, 1);
    sh.set_value(0, 3, 1);
    sh.set_value(0, 4, 1);
    sh.set_value(0, 5, 1);
    sh.set_value(0, 6, 1);
    sh.set_value(0, 7, 1);
    sh.set_value(0, 8, 1);
    sh.set_value(0, 9, 1);

    sh.set_header_cols(2, 4);
    sh.add_col_group(1, 3);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_header_8.ods")?;

    let wb = read_ods("test_out/test_header_8.ods")?;
    let sh = wb.sheet(0);

    let v = sh.col_group_iter().cloned().collect::<Vec<_>>();
    assert!(v.contains(&Grouped::new(1, 3, true)));

    if let Some(h) = sh.header_cols() {
        assert!(h.from == 2 && h.to == 4);
    } else {
        panic!();
    }

    Ok(())
}
