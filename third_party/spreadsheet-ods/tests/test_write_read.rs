pub mod lib_test;

use lib_test::*;
use spreadsheet_ods::sheet::SplitMode;
use spreadsheet_ods::{
    read_ods, read_ods_buf, write_ods_buf, write_ods_to, OdsError, Sheet, ValueType, WorkBook,
};
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::time::Instant;

pub fn timingr<E, R>(name: &str, mut fun: impl FnMut() -> Result<R, E>) -> Result<R, E> {
    let now = Instant::now();
    let result = fun()?;
    println!("{} {:?}", name, now.elapsed());
    Ok(result)
}

pub fn timingn<E>(name: &str, mut fun: impl FnMut()) -> Result<(), E> {
    let now = Instant::now();
    fun();
    println!("{} {:?}", name, now.elapsed());
    Ok(())
}

#[test]
fn read_rw_orders() -> Result<(), OdsError> {
    let mut wb = read_ods("tests/test_write_read_1.ods")?;
    test_write_ods(&mut wb, "test_out/test_write_read_1.ods")?;
    let _wb = read_ods("test_out/test_write_read_1.ods")?;
    Ok(())
}

#[test]
fn read_orders() -> Result<(), OdsError> {
    let mut wb = read_ods("tests/test_write_read_1.ods")?;

    wb.config_mut().has_sheet_tabs = false;

    let cc = wb.sheet_mut(0).config_mut();
    cc.show_grid = true;
    cc.vert_split_pos = 2;
    cc.vert_split_mode = SplitMode::Heading;

    test_write_ods(&mut wb, "test_out/test_write_read_1_2.ods")?;
    Ok(())
}

#[test]
fn test_write_read_write_read() -> Result<(), OdsError> {
    let path = Path::new("tests/test_write_read_2.ods");
    let temp = Path::new("test_out/test_write_read_2.ods");

    std::fs::copy(path, temp)?;

    let mut ods = read_ods(temp)?;
    test_write_ods(&mut ods, temp)?;
    let _ods = read_ods(temp)?;

    Ok(())
}

#[should_panic]
#[test]
fn test_write_repeat_overlapped() {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");
    sh.set_row_repeat(0, 3);
    sh.set_value(1, 0, "X");
    sh.set_value(2, 0, "X");
    sh.set_value(3, 0, "B");

    wb.push_sheet(sh);

    let path = Path::new("test_out/test_write_read_3.ods");
    test_write_odsbuf(&mut wb).unwrap();

    let _ods = read_ods(path).unwrap();
}

#[test]
fn test_write_repeat_overlapped2() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");
    sh.set_row_repeat(0, 3);
    sh.set_value(4, 0, "X");
    sh.set_value(5, 0, "X");
    sh.set_value(6, 0, "B");

    wb.push_sheet(sh);

    let path = Path::new("test_out/test_write_read_4.ods");
    test_write_ods(&mut wb, path)?;

    let _ods = read_ods(path)?;

    Ok(())
}

#[test]
fn test_write_buf() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");
    wb.push_sheet(sh);

    let len_1 = {
        let p = Path::new("test_out/test_write_read_5_1.ods");
        test_write_ods(&mut wb, p)?;
        p.to_path_buf().metadata()?.len() as usize
    };

    let len_2 = {
        let v = Vec::new();
        let v = write_ods_buf(&mut wb, v)?;

        let mut ff = File::create("test_out/test_write_read_5_2.ods")?;
        ff.write_all(&v)?;

        v.len()
    };

    // This can sporadically fail because of some instability in hashtable.
    // Writing syncs a few structures and then *something* happens.
    // A different ordering of some attributes occurs and this diff fails.
    assert_eq!(len_1, len_2);

    Ok(())
}

#[test]
fn test_read_buf() -> Result<(), OdsError> {
    let mut buf = Vec::new();
    let mut f = File::open("tests/test_write_read_1.ods")?;
    f.read_to_end(&mut buf)?;

    let mut wb = read_ods_buf(&buf)?;

    wb.config_mut().has_sheet_tabs = false;

    let cc = wb.sheet_mut(0).config_mut();
    cc.show_grid = true;
    cc.vert_split_pos = 2;
    cc.vert_split_mode = SplitMode::Heading;

    test_write_ods(&mut wb, "test_out/test_write_read_6.ods")?;
    Ok(())
}

#[test]
fn test_write_write() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");
    wb.push_sheet(sh);

    let v = Cursor::new(Vec::new());
    write_ods_to(&mut wb, v)?;

    Ok(())
}

#[test]
fn test_write_read() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();
    let mut sh = Sheet::new("1");

    sh.set_value(0, 0, "A");

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_write_read_7.ods")?;

    let wi = read_ods("test_out/test_write_read_7.ods")?;
    let si = wi.sheet(0);

    assert_eq!(si.value(0, 0).as_str_or(""), "A");

    Ok(())
}

#[test]
fn read_text() -> Result<(), OdsError> {
    let wb = read_ods("tests/test_write_read_3.ods")?;
    let sh = wb.sheet(0);

    let v = sh.value(0, 0);

    assert_eq!(v.value_type(), ValueType::TextXml);

    Ok(())
}
