//!
//! For now defines functions to create cell references for formulas.
//!

use crate::refs::{CellRange, CellRef};

/// Simple macro for formula.
#[macro_export]
macro_rules! formula {
    ($format:literal) => {{
        let res = std::fmt::format(
            format_args!(concat!("of:=", $format))
        );
        res
    }};
    ($format:literal , $($arg:tt)*) => {{
        let res = std::fmt::format(
            format_args!(concat!("of:=", $format), $($arg)*)
        );
        res
    }};
}

/// Macro for cell-references. Returns as string with the cell-reference in
/// a format suitable for formulas.
///
/// Syntax:
/// ```bnf
///     cell!(\[abs] row, \[abs] col);
///     cell!(\[abs] row, \[abs] col, \[abs] row_to, \[abs] col_to);
///     cell!(table => \[abs] row, \[abs] col);
///     cell!(table => \[abs] row, \[abs] col, \[abs] row_to, \[abs] col_to);
/// ```
#[macro_export]
macro_rules! fcell {
    ($($arg:tt)*) => {
        cell!($($arg)*).to_formula()
    }
}

/// Macro for cell-references. This one returns the reference itself.
/// For use in formulas use fcell, which returns the correct string for formulas.
///
/// Syntax:
/// ```bnf
///     cell!(\[abs] row, \[abs] col);
///     cell!(\[abs] row, \[abs] col, \[abs] row_to, \[abs] col_to);
///     cell!(table => \[abs] row, \[abs] col);
///     cell!(table => \[abs] row, \[abs] col, \[abs] row_to, \[abs] col_to);
/// ```
#[macro_export]
macro_rules! cell {
    (abs $row:expr, abs $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, None, true, $row, true, $col)
    };
    (abs $row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, None, true, $row, false, $col)
    };
    ($row:expr, abs $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, None, false, $row, true, $col)
    };
    ($row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, None, false, $row, false, $col)
    };

    (abs $row:expr, abs $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, true, $col, None, true, $row2, true, $col2,
        )
    };
    (abs $row:expr, abs $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, true, $col, None, true, $row2, false, $col2,
        )
    };
    (abs $row:expr, abs $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, true, $col, None, false, $row2, true, $col2,
        )
    };
    (abs $row:expr, abs $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, true, $col, None, false, $row2, false, $col2,
        )
    };
    (abs $row:expr, $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, false, $col, None, true, $row2, true, $col2,
        )
    };
    (abs $row:expr, $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, false, $col, None, true, $row2, false, $col2,
        )
    };
    (abs $row:expr, $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, false, $col, None, false, $row2, true, $col2,
        )
    };
    (abs $row:expr,  $col:expr,  $row2:expr,  $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, true, $row, false, $col, None, false, $row2, false, $col2,
        )
    };
    ( $row:expr, abs $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, true, $col, None, true, $row2, true, $col2,
        )
    };
    ( $row:expr, abs $col:expr, abs $row2:expr,  $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, true, $col, None, true, $row2, false, $col2,
        )
    };
    ( $row:expr, abs $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, true, $col, None, false, $row2, true, $col2,
        )
    };
    ($row:expr, abs $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, true, $col, None, false, $row2, false, $col2,
        )
    };
    ($row:expr, $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, false, $col, None, true, $row2, true, $col2,
        )
    };
    ($row:expr, $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, false, $col, None, true, $row2, false, $col2,
        )
    };
    ($row:expr, $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, false, $col, None, false, $row2, true, $col2,
        )
    };
    ($row:expr, $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None, None, false, $row, false, $col, None, false, $row2, false, $col2,
        )
    };

    ($table:expr => abs $row:expr, abs $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, Some($table.into()), true, $row, true, $col)
    };
    ($table:expr => abs $row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, Some($table.into()), true, $row, true, $col)
    };
    ($table:expr => $row:expr, abs $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, Some($table.into()), true, $row, true, $col)
    };
    ($table:expr => $row:expr, $col:expr) => {
        spreadsheet_ods::CellRef::new_all(None, Some($table.into()), true, $row, true, $col)
    };

    ($table:expr => abs $row:expr, abs $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            true,
            $col,
            None,
            true,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, abs $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            true,
            $col,
            None,
            true,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, abs $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            true,
            $col,
            None,
            false,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, abs $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            true,
            $col,
            None,
            false,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            false,
            $col,
            None,
            true,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            false,
            $col,
            None,
            true,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr => abs $row:expr, $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            false,
            $col,
            None,
            false,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => abs $row:expr,  $col:expr,  $row2:expr,  $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            true,
            $row,
            false,
            $col,
            None,
            false,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr =>  $row:expr, abs $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            true,
            $col,
            None,
            true,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr =>  $row:expr, abs $col:expr, abs $row2:expr,  $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            true,
            $col,
            None,
            true,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr =>  $row:expr, abs $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            true,
            $col,
            None,
            false,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => $row:expr, abs $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            true,
            $col,
            None,
            false,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr => $row:expr, $col:expr, abs $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            false,
            $col,
            None,
            true,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => $row:expr, $col:expr, abs $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            false,
            $col,
            None,
            true,
            $row2,
            false,
            $col2,
        )
    };
    ($table:expr => $row:expr, $col:expr, $row2:expr, abs $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            false,
            $col,
            None,
            false,
            $row2,
            true,
            $col2,
        )
    };
    ($table:expr => $row:expr, $col:expr, $row2:expr, $col2:expr) => {
        spreadsheet_ods::CellRange::new_all(
            None,
            Some($table.into()),
            false,
            $row,
            false,
            $col,
            None,
            false,
            $row2,
            false,
            $col2,
        )
    };
}

/// Creates a cell-reference for use in formulas.
pub fn fcellref(row: u32, col: u32) -> String {
    CellRef::local(row, col).to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute row reference.
pub fn fcellrefr(row: u32, col: u32) -> String {
    CellRef::local(row, col).absolute_row().to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute col reference.
pub fn fcellrefc(row: u32, col: u32) -> String {
    CellRef::local(row, col).absolute_col().to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute reference.
pub fn fcellrefa(row: u32, col: u32) -> String {
    CellRef::local(row, col).absolute().to_formula()
}

/// Creates a cell-reference for use in formulas.
pub fn fcellref_table<S: Into<String>>(table: S, row: u32, col: u32) -> String {
    CellRef::remote(table, row, col).to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute row reference.
pub fn fcellrefr_table<S: Into<String>>(table: S, row: u32, col: u32) -> String {
    CellRef::remote(table, row, col).absolute_row().to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute col reference.
pub fn fcellrefc_table<S: Into<String>>(table: S, row: u32, col: u32) -> String {
    CellRef::remote(table, row, col).absolute_col().to_formula()
}

/// Creates a cell-reference for use in formulas.
/// Creates an absolute reference.
pub fn fcellrefa_table<S: Into<String>>(table: S, row: u32, col: u32) -> String {
    CellRef::remote(table, row, col).absolute().to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangeref(row: u32, col: u32, row_to: u32, col_to: u32) -> String {
    CellRange::local(row, col, row_to, col_to).to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefr(row: u32, col: u32, row_to: u32, col_to: u32) -> String {
    CellRange::local(row, col, row_to, col_to)
        .absolute_rows()
        .to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefc(row: u32, col: u32, row_to: u32, col_to: u32) -> String {
    CellRange::local(row, col, row_to, col_to)
        .absolute_cols()
        .to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefa(row: u32, col: u32, row_to: u32, col_to: u32) -> String {
    CellRange::local(row, col, row_to, col_to)
        .absolute()
        .to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangeref_table<S: Into<String>>(
    table: S,
    row: u32,
    col: u32,
    row_to: u32,
    col_to: u32,
) -> String {
    CellRange::remote(table, row, col, row_to, col_to).to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefr_table<S: Into<String>>(
    table: S,
    row: u32,
    col: u32,
    row_to: u32,
    col_to: u32,
) -> String {
    CellRange::remote(table, row, col, row_to, col_to)
        .absolute_rows()
        .to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefc_table<S: Into<String>>(
    table: S,
    row: u32,
    col: u32,
    row_to: u32,
    col_to: u32,
) -> String {
    CellRange::remote(table, row, col, row_to, col_to)
        .absolute_cols()
        .to_formula()
}

/// Creates a cellrange-reference for use in formulas.
pub fn frangerefa_table<S: Into<String>>(
    table: S,
    row: u32,
    col: u32,
    row_to: u32,
    col_to: u32,
) -> String {
    CellRange::remote(table, row, col, row_to, col_to)
        .absolute()
        .to_formula()
}
