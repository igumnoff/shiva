use std::fs;
use std::fs::File;
use std::io::BufReader;

use lib_test::*;
use spreadsheet_ods::defaultstyles::DefaultFormat;
use spreadsheet_ods::sheet::Visibility;
use spreadsheet_ods::{read_ods, CellStyle, OdsError, OdsOptions, Sheet, Value, WorkBook};

mod lib_test;

// basic case, data in the very first row
#[test]
fn test_write_first() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_1.ods")?;

    let wb = read_ods("test_out/test_core_writing_loop_1.ods")?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(0, 0).as_i32_or(0), 1);

    Ok(())
}

// basic case, empty rows before the first data row.
#[test]
fn test_write_empty_before() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(5, 0, 1);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_2.ods")?;

    let wb = read_ods("test_out/test_core_writing_loop_2.ods")?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(5, 0).as_i32_or(0), 1);

    Ok(())
}

// basic case, row1 after row2
#[test]
fn test_write_simple() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(4, 0, 1);
    sh.set_value(5, 0, 1);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_3.ods")?;

    let wb = read_ods("test_out/test_core_writing_loop_3.ods")?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(4, 0).as_i32_or(0), 1);
    assert_eq!(sh.value(5, 0).as_i32_or(0), 1);

    Ok(())
}

// basic case, row1 gap row2
#[test]
fn test_write_gap() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(2, 0, 1);
    sh.set_value(5, 0, 1);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_4.ods")?;

    let wb = read_ods("test_out/test_core_writing_loop_4.ods")?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(2, 0).as_i32_or(0), 1);
    assert_eq!(sh.value(5, 0).as_i32_or(0), 1);

    Ok(())
}

// basic case, row1 gap row2
// row1 with a repeat of 2
#[test]
fn test_write_gap_repeat() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(2, 0, 1);
    sh.set_row_repeat(2, 2);
    sh.set_value(5, 0, 1);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_5.ods")?;

    let wb = read_ods("test_out/test_core_writing_loop_5.ods")?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(2, 0).as_i32_or(0), 1);
    assert_eq!(sh.value(5, 0).as_i32_or(0), 1);

    Ok(())
}

#[test]
#[should_panic]
fn test_write_row_overlap() -> () {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(2, 0, 1);
    sh.set_row_repeat(2, 2);
    sh.set_value(3, 0, 1);
    wb.push_sheet(sh);

    match test_write_ods(&mut wb, "test_out/test_core_writing_loop_6.ods") {
        Ok(_) => {}
        Err(_) => {
            let _ = fs::remove_file("test_out/test_core_writing_loop_6.ods");
            panic!();
        }
    }
}

#[test]
#[should_panic]
fn test_write_col_overlap() -> () {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(3, 0, 100);
    sh.set_cell_repeat(3, 0, 5);
    sh.set_value(3, 4, 101);
    wb.push_sheet(sh);

    match test_write_ods(&mut wb, "test_out/test_core_writing_loop_7.ods") {
        Ok(_) => {}
        Err(_) => {
            let _ = fs::remove_file("test_out/test_core_writing_loop_7.ods");
            panic!();
        }
    }
}

#[test]
fn test_write_repeat() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");

    sh.set_value(9, 9, "X");

    sh.set_value(2, 0, 100);
    sh.set_cell_repeat(2, 0, 5);

    sh.set_value(3, 0, 100);
    sh.set_cell_repeat(3, 0, 20);

    sh.set_value(4, 0, 100);
    sh.set_cell_repeat(4, 0, 5);
    sh.set_value(4, 5, 101);

    sh.set_value(5, 1, "V");
    sh.set_col_span(5, 1, 2);
    sh.set_row_span(5, 1, 2);

    sh.set_value(6, 0, 100);
    sh.set_cell_repeat(6, 0, 5);
    sh.set_value(6, 5, 101);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_8.ods")?;

    let read = BufReader::new(File::open("test_out/test_core_writing_loop_8.ods")?);
    let wb = OdsOptions::default()
        .use_repeat_for_cells()
        .read_ods(read)?;
    let sh = wb.sheet(0);

    assert_eq!(sh.value(9, 9).as_str_or(""), "X");
    assert_eq!(sh.value(2, 0).as_u32_or(0), 100);
    assert_eq!(sh.cell_repeat(2, 0), 5);
    assert_eq!(sh.value(4, 5).as_u32_or(0), 101);
    assert_eq!(sh.value(6, 0).as_u32_or(0), 100);
    assert_eq!(sh.cell_repeat(6, 0), 1);
    assert_eq!(sh.value(6, 1).as_u32_or(0), 100);
    assert_eq!(sh.cell_repeat(6, 1), 2);
    assert_eq!(sh.value(6, 3).as_u32_or(0), 100);
    assert_eq!(sh.cell_repeat(6, 3), 2);
    assert_eq!(sh.value(6, 5).as_u32_or(0), 101);
    assert_eq!(sh.cell_repeat(6, 5), 1);

    Ok(())
}

#[test]
fn test_row_repeat() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(2, 0, 1);
    sh.set_row_repeat(2, 2);
    sh.set_value(5, 0, 1);
    sh.set_value(5, 1, Value::Empty);
    sh.set_row_repeat(5, 5);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_9.ods")?;

    let f = BufReader::new(File::open("test_out/test_core_writing_loop_9.ods")?);
    let wb = OdsOptions::default().use_repeat_for_cells().read_ods(f)?;
    let sh = wb.sheet(0);

    assert_eq!(sh.row_repeat(2), 2);
    assert_eq!(sh.row_repeat(5), 5);

    Ok(())
}

#[test]
fn test_void() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut s_0 = CellStyle::new("", &DefaultFormat::number());
    s_0.set_font_bold();
    let s_0 = wb.add_cellstyle(s_0);

    let mut sh = Sheet::new("Sheet1");

    // empty should be ignored
    sh.set_value(1, 0, Value::Empty);

    // default-cellstyle should be ignored
    sh.set_col_cellstyle(1, &s_0);
    sh.set_styled(1, 1, "text", &s_0);
    sh.set_cellstyle(2, 1, &s_0);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_core_writing_loop_10.ods")?;

    let f = BufReader::new(File::open("test_out/test_core_writing_loop_10.ods")?);
    let wb = OdsOptions::default().use_repeat_for_cells().read_ods(f)?;
    let sh = wb.sheet(0);

    // dbg!(sh);

    assert!(sh.cell_ref(1, 0).is_none());
    assert!(sh.cell_ref(2, 1).is_none());

    Ok(())
}

#[test]
fn test_header_span() -> Result<(), OdsError> {
    let mut sh = Sheet::new("Sheet1");

    sh.set_row_visible(0, Visibility::Collapsed);
    sh._set_row_header_span(0, 10);

    assert_eq!(sh.row_visible(0), Visibility::Collapsed);
    assert_eq!(sh.row_visible(9), Visibility::Collapsed);

    sh.set_row_visible(0, Visibility::Filtered);

    assert_eq!(sh.row_visible(0), Visibility::Filtered);
    assert_eq!(sh.row_visible(1), Visibility::Collapsed);
    assert_eq!(sh.row_visible(9), Visibility::Collapsed);
    assert_eq!(sh.row_visible(10), Visibility::default());

    sh.set_row_visible(5, Visibility::Filtered);

    assert_eq!(sh.row_visible(0), Visibility::Filtered);
    assert_eq!(sh.row_visible(1), Visibility::Collapsed);
    assert_eq!(sh.row_visible(5), Visibility::Filtered);
    assert_eq!(sh.row_visible(9), Visibility::Collapsed);
    assert_eq!(sh.row_visible(10), Visibility::default());

    sh.set_row_visible(9, Visibility::Filtered);

    assert_eq!(sh.row_visible(0), Visibility::Filtered);
    assert_eq!(sh.row_visible(1), Visibility::Collapsed);
    assert_eq!(sh.row_visible(5), Visibility::Filtered);
    assert_eq!(sh.row_visible(8), Visibility::Collapsed);
    assert_eq!(sh.row_visible(9), Visibility::Filtered);
    assert_eq!(sh.row_visible(10), Visibility::default());

    // dbg!(&sh);

    Ok(())
}
