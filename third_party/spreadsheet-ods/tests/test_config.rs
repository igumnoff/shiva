mod lib_test;

use lib_test::*;
use spreadsheet_ods::sheet::SplitMode;
use spreadsheet_ods::{read_ods, OdsError};

#[test]
fn read_orders() -> Result<(), OdsError> {
    let mut wb = read_ods("tests/test_config.ods")?;

    wb.config_mut().has_sheet_tabs = false;

    let cc = wb.sheet_mut(0).config_mut();
    cc.show_grid = true;
    cc.vert_split_pos = 2;
    cc.vert_split_mode = SplitMode::Heading;

    test_write_ods(&mut wb, "test_out/test_config.ods")?;
    Ok(())
}
