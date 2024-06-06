//!
//! Output formatting for the AST.
//!

use std::fmt;

/// Appends the spreadsheet row name
pub(crate) fn fmt_abs(f: &mut impl fmt::Write, abs: bool) -> fmt::Result {
    if abs {
        write!(f, "$")?;
    }
    Ok(())
}

/// Appends the spreadsheet row name
pub(crate) fn fmt_row_name(f: &mut impl fmt::Write, row: u32) -> fmt::Result {
    let mut i = 0;
    let mut dbuf = [0u8; 10];

    // temp solution
    let mut row: u64 = row.into();
    row += 1;
    while row > 0 {
        dbuf[i] = (row % 10) as u8;
        row /= 10;

        i += 1;
    }

    // reverse order
    let mut j = i;
    while j > 0 {
        write!(f, "{}", (b'0' + dbuf[j - 1]) as char)?;
        j -= 1;
    }

    Ok(())
}

/// Appends the spreadsheet column name.
pub(crate) fn fmt_col_name(f: &mut impl fmt::Write, mut col: u32) -> fmt::Result {
    let mut i = 0;
    let mut dbuf = [0u8; 7];

    if col == u32::MAX {
        // unroll first loop because of overflow
        dbuf[0] = 21;
        i += 1;
        col /= 26;
    } else {
        col += 1;
    }

    while col > 0 {
        dbuf[i] = (col % 26) as u8;
        if dbuf[i] == 0 {
            dbuf[i] = 25;
            col = col / 26 - 1;
        } else {
            dbuf[i] -= 1;
            col /= 26;
        }

        i += 1;
    }

    // reverse order
    let mut j = i;
    while j > 0 {
        write!(f, "{}", (b'A' + dbuf[j - 1]) as char)?;
        j -= 1;
    }

    Ok(())
}
