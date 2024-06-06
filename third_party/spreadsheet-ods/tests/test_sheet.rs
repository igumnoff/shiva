mod lib_test;

use icu_locid::locale;
use lib_test::*;
use spreadsheet_ods::defaultstyles::DefaultFormat;
use spreadsheet_ods::{
    cm, currency, percent, read_ods, CellRange, CellStyle, CellStyleRef, Length, OdsError,
    OdsOptions, Sheet, Value, ValueType, WorkBook,
};
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_colwidth() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Sheet1");
    sh.set_value(0, 0, 1234);
    sh.set_col_width(0, cm!(2.54));
    sh.set_row_height(0, cm!(1.27));
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_sheet_1.ods")?;

    let wb = read_ods("test_out/test_sheet_1.ods")?;

    assert_eq!(wb.sheet(0).col_width(0), cm!(2.54));
    assert_eq!(wb.sheet(0).row_height(0), cm!(1.27));

    Ok(())
}

#[test]
fn test_cell() {
    let mut sh = Sheet::new("1");

    sh.set_value(5, 5, 1u32);
    sh.set_value(6, 6, 2u32);

    if let Some(c) = sh.cell(5, 5) {
        assert_eq!(c.value().as_i32_or(0), 1);
    }

    // let c = sh.cell_mut(6, 6);
    // c.set_value(3);
    // let mut x = SCell::new();
    // std::mem::swap(c, &mut x);
    // assert_eq!(x.value().as_f64_or(0.0), 3.0);
}

#[test]
fn test_row_repeat() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(2, 2, 1);
    sh.set_value(4, 4, 2);
    sh.set_row_repeat(4, 2);

    wb.push_sheet(sh);
    test_write_ods(&mut wb, "test_out/test_sheet_2.ods")?;

    let r = BufReader::new(File::open("test_out/test_sheet_2.ods")?);
    let wb = OdsOptions::default().use_repeat_for_cells().read_ods(r)?;

    assert_eq!(wb.sheet(0).row_repeat(4), 2);

    Ok(())
}

#[test]
fn test_currency() {
    let mut sh = Sheet::new("1");
    sh.set_value(0, 0, currency!("€", 20));
    assert_eq!(sh.value(0, 0).value_type(), ValueType::Currency);

    assert_eq!(currency!("F", 20).currency(), "F");
    assert_eq!(currency!("FR", 20).currency(), "FR");
    assert_eq!(currency!("FRB", 20).currency(), "FRB");
    assert_eq!(currency!("FRBX", 20).currency(), "FRBX");
}

#[test]
fn test_percentage() {
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, percent!(17.22));
    assert_eq!(sh.value(0, 0).value_type(), ValueType::Percentage);
}

#[test]
fn test_span() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("1");
    sh.set_value(0, 0, "A");
    sh.set_value(0, 1, "A2");
    sh.set_value(0, 2, "bomb");
    sh.set_value(1, 0, "bomb");
    sh.set_value(1, 1, "bomb");
    sh.set_value(1, 2, "bomb");
    sh.set_col_span(0, 0, 2);
    wb.push_sheet(sh);

    let mut sh = Sheet::new("1");
    sh.set_value(1, 0, "B");
    sh.set_value(2, 0, "B2");
    sh.set_value(1, 1, "bomb");
    sh.set_value(2, 1, "bomb");
    sh.set_value(3, 0, "bomb");
    sh.set_value(3, 1, "bomb");
    sh.set_row_span(1, 0, 2);
    wb.push_sheet(sh);

    let mut sh = Sheet::new("1");
    sh.set_value(3, 0, "C");
    sh.set_value(3, 1, "C2");
    sh.set_value(4, 0, "C2");
    sh.set_value(4, 1, "C2");
    sh.set_value(3, 2, "bomb");
    sh.set_value(4, 2, "bomb");
    sh.set_value(5, 0, "bomb");
    sh.set_value(5, 1, "bomb");
    sh.set_value(5, 2, "bomb");
    sh.set_col_span(3, 0, 2);
    sh.set_row_span(3, 0, 2);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_sheet_3.ods")?;
    let wi = read_ods("test_out/test_sheet_3.ods")?;

    let si = wi.sheet(0);

    assert_eq!(si.value(0, 0).as_str_or(""), "A");
    assert_eq!(si.col_span(0, 0), 2);

    let si = wi.sheet(1);

    assert_eq!(si.value(1, 0).as_str_or(""), "B");
    assert_eq!(si.row_span(1, 0), 2);

    Ok(())
}

#[test]
fn test_print_range() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("1");
    for i in 0..10 {
        for j in 0..10 {
            sh.set_value(i, j, i * j);
        }
    }
    sh.add_print_range(CellRange::local(1, 1, 9, 9));
    sh.add_print_range(CellRange::local(11, 11, 19, 19));
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_sheet_4.ods")?;

    let wb = read_ods("test_out/test_sheet_4.ods")?;
    let sh = wb.sheet(0);

    let r = sh.print_ranges().unwrap();
    assert_eq!(r[0], CellRange::local(1, 1, 9, 9));
    assert_eq!(r[1], CellRange::local(11, 11, 19, 19));

    Ok(())
}

#[test]
fn display_print() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut s0 = Sheet::new("1");
    s0.set_value(0, 0, "display");
    s0.set_display(false);
    wb.push_sheet(s0);

    let mut s1 = Sheet::new("1");
    s1.set_value(0, 0, "print");
    s1.set_print(false);
    wb.push_sheet(s1);

    test_write_ods(&mut wb, "test_out/test_sheet_5.ods")?;

    Ok(())
}

#[test]
fn split_table() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut sh = Sheet::new("Split0");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 2);
    sh.set_value(1, 0, 3);
    sh.set_value(1, 1, 4);
    sh.split_col_header(3);
    wb.push_sheet(sh);

    let mut sh = Sheet::new("Split1");
    sh.set_value(0, 0, 1);
    sh.set_value(0, 1, 2);
    sh.set_value(1, 0, 3);
    sh.set_value(1, 1, 4);
    sh.split_horizontal(250);
    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_sheet_6.ods")?;

    Ok(())
}

#[test]
fn test_iterator() {
    let mut sh = Sheet::new("1");
    for r in 1..100 {
        for c in 1..10 {
            if r % c == 0 {
                sh.set_styled_value(r, c, 4711, &CellStyleRef::from("foo"));
            }
        }
    }

    let mut it = sh.into_iter();
    while let Some(((_cur_row, _cur_col), _cell)) = it.next() {
        if let Some(_p) = it.peek_cell() {
            // println!("{:?} -> {:?}", (cur_row, cur_col), p);
        } else {
            // println!("{:?} -> {:?}", (cur_row, cur_col), ());
        }
    }
}

#[test]
fn test_cell_style() {
    let mut wb = WorkBook::new(locale!("de_AT"));

    let s0 = CellStyle::new("a21", &DefaultFormat::number());
    let s0 = wb.add_cellstyle(s0);

    let ss0 = wb.cellstyle(&s0).expect("style");
    assert_eq!(ss0.name(), "a21");
}
