//!

use criterion::{criterion_group, criterion_main, Criterion};
use icu_locid::locale;
use spreadsheet_ods::{
    read_ods, write_ods_buf, write_ods_buf_uncompressed, OdsError, Sheet, WorkBook,
};

fn read_orders() -> Result<(), OdsError> {
    let _ = read_ods("test_write_read_1.ods")?;
    Ok(())
}

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
                sh.set_cellstyle(r, c, &"s0".into());
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
        let buf = write_ods_buf_uncompressed(wb, Vec::new())?;
        Ok(())
    }
}

fn criterion_read(c: &mut Criterion) {
    c.bench_function("read", |b| b.iter(|| read_orders()));
}

fn criterion_write(c: &mut Criterion) {
    c.bench_function("write", |b| {
        b.iter(|| {
            let mut wb = create_wb(100, 400).expect("create_wb");
            write_ods_buf(&mut wb, Vec::new()).expect("write_ods_buf");
        })
    });
}

///
criterion_group!(benches, criterion_read, criterion_write);
///
criterion_main!(benches);
