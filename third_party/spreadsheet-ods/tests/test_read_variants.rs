use spreadsheet_ods::{read_ods, OdsError};

#[test]
fn read_google() -> Result<(), OdsError> {
    let _wb = read_ods("tests/test_read_google.ods")?;
    // dbg!(wb.sheet(0).cell_ref(1, 1));
    Ok(())
}

#[test]
fn read_libreoffice() -> Result<(), OdsError> {
    let _wb = read_ods("tests/test_read_libreoffice.ods")?;
    // dbg!(wb.sheet(0).cell_ref(1, 1));
    Ok(())
}

#[test]
fn read_office365() -> Result<(), OdsError> {
    let _wb = read_ods("tests/test_read_office365.ods")?;
    // dbg!(wb.sheet(0).cell_ref(1, 1));
    Ok(())
}
