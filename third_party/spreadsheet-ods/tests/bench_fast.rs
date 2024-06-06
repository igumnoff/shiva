#![allow(dead_code)]

mod lib_test;

use crate::lib_test::{Timing, Unit};
use icu_locid::locale;
use spreadsheet_ods::{
    read_ods, write_ods_buf, write_ods_buf_uncompressed, CellStyleRef, OdsError, Sheet, WorkBook,
};

const ROWS: u32 = 100;
const COLS: u32 = 400;

fn create_wb(rows: u32, cols: u32) -> Result<WorkBook, OdsError> {
    let mut wb = WorkBook::new_empty();
    wb.locale_settings(locale!("en_US"));
    let mut sh = Sheet::new("1");

    for r in 0..rows {
        if r % 2 == 0 {
            for c in 0..cols {
                sh.set_value(r, c, "1234");
            }
        } else {
            for c in 0..cols {
                sh.set_value(r, c, 1u32);
            }
        }
        if r % 2 == 0 {
            for c in 0..cols {
                sh.set_cellstyle(r, c, &CellStyleRef::from("s0"));
            }
        }
        if r % 10 == 0 {
            for c in 0..cols {
                sh.set_formula(r, c, "of:=1+1");
            }
        }
        if r % 50 == 0 {
            for c in 0..cols {
                sh.set_validation(r, c, &"v0".into());
            }
        }
    }

    wb.push_sheet(sh);

    Ok(wb)
}

fn write_wb<'a>(wb: &'a mut WorkBook) -> impl FnMut() -> Result<(), OdsError> + 'a {
    move || {
        let _buf = write_ods_buf_uncompressed(wb, Vec::new())?;
        Ok(())
    }
}

// #[test]
fn test_read_orders() -> Result<(), OdsError> {
    let mut t = Timing::<()>::default()
        .name("read_orders")
        .skip(2)
        .runs(30)
        .unit(Unit::Millisecond);

    let _ = t.run(|| read_ods("tests/bench_fast.ods"))?;

    println!("{}", t);

    Ok(())
}

// #[test]
fn test_create_wb() -> Result<(), OdsError> {
    let mut t = Timing::<()>::default()
        .name("create_wb")
        .skip(2)
        .runs(30)
        .divider(ROWS as u64 * COLS as u64);

    let _ = t.run(|| create_wb(ROWS, COLS))?;

    println!("{}", t);

    Ok(())
}

// #[test]
fn test_write_wb() -> Result<(), OdsError> {
    let mut t = Timing::<()>::default()
        .name("write_wb")
        .skip(2)
        .runs(30)
        .divider(ROWS as u64 * COLS as u64);

    let mut wb = create_wb(ROWS, COLS)?;
    let _ = t.run(|| write_ods_buf(&mut wb, Vec::new()));

    println!("{}", t);

    Ok(())
}
