use spreadsheet_ods::{read_fods, read_ods, write_fods, OdsError};

#[test]
fn read_write_fods() -> Result<(), OdsError> {
    let mut wb = read_ods("tests/test_fods.ods")?;
    write_fods(&mut wb, "test_out/test_fods.fods")?;
    let _wb = read_fods("test_out/test_fods.fods")?;
    Ok(())
}
