mod lib_test;

use lib_test::*;
use spreadsheet_ods::{read_ods, OdsError};

#[test]
fn test_draw_image() -> Result<(), OdsError> {
    let mut wb = read_ods("tests/test_draw.ods")?;

    let sh = wb.sheet(0);
    assert!(sh.draw_frames(1, 1).is_some());

    test_write_ods(&mut wb, "test_out/test_draw.ods")?;
    let wb = read_ods("test_out/test_draw.ods")?;

    let sh = wb.sheet(0);
    assert!(sh.draw_frames(1, 1).is_some());

    Ok(())
}

#[test]
fn test_images() -> Result<(), OdsError> {
    let wb = read_ods("tests/test_draw.ods")?;
    let sh = wb.sheet(0);

    for ((_r, _c), d) in sh.iter() {
        if let Some(_draw_frames) = d.draw_frames {
            // dbg!(draw_frames);
        }
    }

    Ok(())
}
