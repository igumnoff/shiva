//!
//! Defines types for cell references.
//!

use crate::refs::format_refs::{
    fmt_cell_range, fmt_cell_ref, fmt_col, fmt_col_range, fmt_row, fmt_row_range,
};
use crate::refs::parser::CRCode::{CRCellRange, CRCellRef, CRColRange, CRRowRange};
use crate::refs::parser::{CRCode, KTokenizerError};
use crate::OdsError;
use get_size::GetSize;
use get_size_derive::GetSize;
use kparse::provider::StdTracker;
use kparse::Track;
use std::fmt;
use std::fmt::{Display, Formatter};

mod format;
mod parser;

/// Basic cell reference.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, GetSize)]
pub struct CRow {
    /// Row reference is fixed.
    row_abs: bool,
    /// Row.
    row: u32,
}

impl CRow {
    /// Row
    pub fn new(row: u32) -> Self {
        Self {
            row_abs: false,
            row,
        }
    }

    /// Row
    pub fn row(&self) -> u32 {
        self.row
    }

    /// Row
    pub fn set_row(&mut self, row: u32) {
        self.row = row;
    }

    /// "$" row reference
    pub fn row_abs(&self) -> bool {
        self.row_abs
    }

    /// "$" row reference
    pub fn set_row_abs(&mut self, abs: bool) {
        self.row_abs = abs;
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.row_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The column remains relative, the row is fixed.
    pub fn absolute_row(mut self) -> Self {
        self.row_abs = true;
        self
    }
}

impl Display for CRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_row(f, self)
    }
}

/// Basic cell reference.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, GetSize)]
pub struct CCol {
    /// Column reference is fixed.
    col_abs: bool,
    /// Column.
    col: u32,
}

impl CCol {
    /// Col
    pub fn new(col: u32) -> Self {
        Self {
            col_abs: false,
            col,
        }
    }

    /// Column
    pub fn col(&self) -> u32 {
        self.col
    }

    /// Column
    pub fn set_col(&mut self, col: u32) {
        self.col = col;
    }

    /// "$" column reference
    pub fn col_abs(&self) -> bool {
        self.col_abs
    }

    /// "$" column reference
    pub fn set_col_abs(&mut self, abs: bool) {
        self.col_abs = abs;
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.col_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The row remains relative, the column is fixed.
    pub fn absolute_col(mut self) -> Self {
        self.col_abs = true;
        self
    }
}

impl Display for CCol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_col(f, self)
    }
}

/// A reference to a cell, possibly in another sheet in another file.
/// ```
/// use spreadsheet_ods::CellRef;
/// let c1 = CellRef::local(5,2);
/// let c2 = CellRef::local(7,4).absolute_col();
/// let c3 = CellRef::remote("spreadsheet-2", 9,6);
/// let c4 = CellRef::try_from(".A6");
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq, GetSize)]
pub struct CellRef {
    /// External reference.
    iri: Option<String>,
    /// sheet reference.
    table: Option<String>,
    /// Cell reference.
    row: CRow,
    col: CCol,
}

impl CellRef {
    /// New CellRef with all possible parameters.
    pub fn new_all(
        iri: Option<String>,
        table: Option<String>,
        row_abs: bool,
        row: u32,
        col_abs: bool,
        col: u32,
    ) -> Self {
        Self {
            iri,
            table,
            row: CRow { row_abs, row },
            col: CCol { col_abs, col },
        }
    }

    /// New defaults.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a cellref within the same table.
    pub fn local(row: u32, col: u32) -> Self {
        Self {
            iri: None,
            table: None,
            row: CRow {
                row_abs: false,
                row,
            },
            col: CCol {
                col_abs: false,
                col,
            },
        }
    }

    /// Creates a cellref that references another table.
    pub fn remote<S: Into<String>>(table: S, row: u32, col: u32) -> Self {
        Self {
            iri: None,
            table: Some(table.into()),
            row: CRow {
                row_abs: false,
                row,
            },
            col: CCol {
                col_abs: false,
                col,
            },
        }
    }

    /// External file
    pub fn set_iri<S: Into<String>>(&mut self, iri: S) {
        self.iri = Some(iri.into());
    }

    /// External file
    pub fn iri(&self) -> Option<&String> {
        self.iri.as_ref()
    }

    /// Table name for references into other tables.
    pub fn set_table<S: Into<String>>(&mut self, table: S) {
        self.table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn table(&self) -> Option<&String> {
        self.table.as_ref()
    }

    /// Row
    pub fn set_row(&mut self, row: u32) {
        self.row.row = row;
    }

    /// Row
    pub fn row(&self) -> u32 {
        self.row.row
    }

    /// "$" row reference
    pub fn set_row_abs(&mut self, abs: bool) {
        self.row.row_abs = abs;
    }

    /// "$" row reference
    pub fn row_abs(&self) -> bool {
        self.row.row_abs
    }

    /// Column
    pub fn set_col(&mut self, col: u32) {
        self.col.col = col;
    }

    /// Column
    pub fn col(&self) -> u32 {
        self.col.col
    }

    /// "$" column reference
    pub fn set_col_abs(&mut self, abs: bool) {
        self.col.col_abs = abs;
    }

    /// "$" column reference
    pub fn col_abs(&self) -> bool {
        self.col.col_abs
    }

    /// Returns a cell reference for a formula.
    pub fn to_formula(&self) -> String {
        let mut buf = String::new();
        buf.push('[');
        let _ = fmt_cell_ref(&mut buf, self);
        buf.push(']');

        buf
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.col.col_abs = true;
        self.row.row_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The column remains relative, the row is fixed.
    pub fn absolute_row(mut self) -> Self {
        self.row.row_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The row remains relative, the column is fixed.
    pub fn absolute_col(mut self) -> Self {
        self.col.col_abs = true;
        self
    }
}

impl TryFrom<&str> for CellRef {
    type Error = OdsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_cellref(s)
    }
}

impl Display for CellRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_cell_ref(f, self)
    }
}

/// A cell-range.
///
/// As usual for a spreadsheet this is meant as inclusive from and to.
///
/// // ```
/// // let r1 = CellRange::local(0, 0, 9, 9);
/// // let r2 = CellRange::origin_span(5, 5, (3, 3));
/// // ```
///
#[derive(Debug, Default, Clone, PartialEq, Eq, GetSize)]
pub struct CellRange {
    /// URI to an external source for this range.
    iri: Option<String>,
    /// First sheet for the range.
    from_table: Option<String>,
    /// From
    from_row: CRow,
    from_col: CCol,
    /// Second sheet for the range. Can be empty if only one sheet is involved.
    to_table: Option<String>,
    /// To
    to_row: CRow,
    to_col: CCol,
}

impl CellRange {
    /// Create a CellRange with all possible arguments.
    #[allow(clippy::too_many_arguments)]
    pub fn new_all(
        iri: Option<String>,
        from_table: Option<String>,
        from_row_abs: bool,
        from_row: u32,
        from_col_abs: bool,
        from_col: u32,
        to_table: Option<String>,
        to_row_abs: bool,
        to_row: u32,
        to_col_abs: bool,
        to_col: u32,
    ) -> Self {
        Self {
            iri,
            from_table,
            from_row: CRow {
                row_abs: from_row_abs,
                row: from_row,
            },
            from_col: CCol {
                col_abs: from_col_abs,
                col: from_col,
            },
            to_table,
            to_row: CRow {
                row_abs: to_row_abs,
                row: to_row,
            },
            to_col: CCol {
                col_abs: to_col_abs,
                col: to_col,
            },
        }
    }

    /// Empty
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates the cell range from from + to data.
    ///
    /// Panic
    ///
    /// If row > to_row or col > to_col.
    pub fn local(row: u32, col: u32, to_row: u32, to_col: u32) -> Self {
        assert!(row <= to_row);
        assert!(col <= to_col);
        Self {
            iri: None,
            from_table: None,
            from_row: CRow {
                row_abs: false,
                row,
            },
            from_col: CCol {
                col_abs: false,
                col,
            },
            to_table: None,
            to_row: CRow {
                row_abs: false,
                row: to_row,
            },
            to_col: CCol {
                col_abs: false,
                col: to_col,
            },
        }
    }

    /// Creates the cell range from from + to data.
    ///
    /// Panic
    ///
    /// If row > to_row or col > to_col.
    pub fn remote<S: Into<String>>(table: S, row: u32, col: u32, to_row: u32, to_col: u32) -> Self {
        assert!(row <= to_row);
        assert!(col <= to_col);
        Self {
            iri: None,
            from_table: Some(table.into()),
            from_row: CRow {
                row_abs: false,
                row,
            },
            from_col: CCol {
                col_abs: false,
                col,
            },
            to_table: None,
            to_row: CRow {
                row_abs: false,
                row: to_row,
            },
            to_col: CCol {
                col_abs: false,
                col: to_col,
            },
        }
    }

    /// Creates the cell range from origin + spanning data.
    ///
    /// Panic
    ///
    /// Both span values must be > 0.
    pub fn origin_span(row: u32, col: u32, span: (u32, u32)) -> Self {
        assert!(span.0 > 0);
        assert!(span.1 > 0);
        Self {
            iri: None,
            from_table: None,
            from_row: CRow {
                row_abs: false,
                row,
            },
            from_col: CCol {
                col_abs: false,
                col,
            },
            to_table: None,
            to_row: CRow {
                row_abs: false,
                row: row + span.0 - 1,
            },
            to_col: CCol {
                col_abs: false,
                col: col + span.1 - 1,
            },
        }
    }

    /// External file
    pub fn set_iri<S: Into<String>>(&mut self, iri: S) {
        self.iri = Some(iri.into());
    }

    /// External file
    pub fn iri(&self) -> Option<&String> {
        self.iri.as_ref()
    }

    /// Table name for references into other tables.
    pub fn set_table<S: Into<String>>(&mut self, table: S) {
        self.from_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn table(&self) -> Option<&String> {
        self.from_table.as_ref()
    }

    /// Row
    pub fn set_row(&mut self, row: u32) {
        self.from_row.row = row;
    }

    /// Row
    pub fn row(&self) -> u32 {
        self.from_row.row
    }

    /// "$" row reference
    pub fn set_row_abs(&mut self, abs: bool) {
        self.from_row.row_abs = abs;
    }

    /// "$" row reference
    pub fn row_abs(&self) -> bool {
        self.from_row.row_abs
    }

    /// Column
    pub fn set_col(&mut self, col: u32) {
        self.from_col.col = col;
    }

    /// Column
    pub fn col(&self) -> u32 {
        self.from_col.col
    }

    /// "$" column reference
    pub fn set_col_abs(&mut self, abs: bool) {
        self.from_col.col_abs = abs;
    }

    /// "$" column reference
    pub fn col_abs(&self) -> bool {
        self.from_col.col_abs
    }

    /// Table name for references into other tables.
    pub fn set_to_table<S: Into<String>>(&mut self, table: S) {
        self.to_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn to_table(&self) -> Option<&String> {
        self.to_table.as_ref()
    }

    /// To row
    pub fn set_to_row(&mut self, to_row: u32) {
        self.to_row.row = to_row;
    }

    /// To row
    pub fn to_row(&self) -> u32 {
        self.to_row.row
    }

    /// "$" row reference
    pub fn set_to_row_abs(&mut self, abs: bool) {
        self.to_row.row_abs = abs;
    }

    /// "$" row reference
    pub fn to_row_abs(&self) -> bool {
        self.to_row.row_abs
    }

    /// To column
    pub fn set_to_col(&mut self, to_col: u32) {
        self.to_col.col = to_col;
    }

    /// To column
    pub fn to_col(&self) -> u32 {
        self.to_col.col
    }

    /// "$" column reference
    pub fn set_to_col_abs(&mut self, abs: bool) {
        self.to_col.col_abs = abs;
    }

    /// "$" column reference
    pub fn to_col_abs(&self) -> bool {
        self.to_col.col_abs
    }

    /// Returns a range reference for a formula.
    pub fn to_formula(&self) -> String {
        let mut buf = String::new();
        buf.push('[');
        let _ = fmt_cell_range(&mut buf, self);
        buf.push(']');
        buf
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.from_col.col_abs = true;
        self.from_row.row_abs = true;
        self.to_col.col_abs = true;
        self.to_row.row_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The columns remain relative, the rows are fixed.
    pub fn absolute_rows(mut self) -> Self {
        self.from_row.row_abs = true;
        self.to_row.row_abs = true;
        self
    }

    /// Makes this CellReference into an absolute reference.
    /// The rows remain relative, the columns are fixed.
    pub fn absolute_cols(mut self) -> Self {
        self.from_col.col_abs = true;
        self.to_col.col_abs = true;
        self
    }

    /// Does the range contain the cell.
    /// This is inclusive for to_row and to_col!
    pub fn contains(&self, row: u32, col: u32) -> bool {
        row >= self.from_row.row
            && row <= self.to_row.row
            && col >= self.from_col.col
            && col <= self.to_col.col
    }

    /// Is this range any longer relevant, when looping rows first, then columns?
    pub fn out_looped(&self, row: u32, col: u32) -> bool {
        row > self.to_row.row || row == self.to_row.row && col > self.to_col.col
    }
}

impl TryFrom<&str> for CellRange {
    type Error = OdsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_cellrange(s)
    }
}

impl Display for CellRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_cell_range(f, self)
    }
}

/// A range over columns.
#[derive(Debug, Default, Clone, PartialEq, Eq, GetSize)]
pub struct ColRange {
    /// External reference.
    iri: Option<String>,
    /// Refers to another sheet.
    from_table: Option<String>,
    /// Column reference is fixed.
    from_col: CCol,
    /// Second sheet for the range. Can be empty if only one sheet is involved.
    to_table: Option<String>,
    /// Column reference is fixed.
    to_col: CCol,
}

impl ColRange {
    /// New with all possible arguments.
    pub fn new_all(
        iri: Option<String>,
        from_table: Option<String>,
        from_col_abs: bool,
        from_col: u32,
        to_table: Option<String>,
        to_col_abs: bool,
        to_col: u32,
    ) -> Self {
        Self {
            iri,
            from_table,
            from_col: CCol {
                col_abs: from_col_abs,
                col: from_col,
            },
            to_table,
            to_col: CCol {
                col_abs: to_col_abs,
                col: to_col,
            },
        }
    }

    /// New range.
    ///
    /// Panic
    ///
    /// If from_col > to_col.
    pub fn new(from_col: u32, to_col: u32) -> Self {
        assert!(from_col <= to_col);
        Self {
            iri: None,
            from_table: None,
            from_col: CCol {
                col_abs: false,
                col: from_col,
            },
            to_table: None,
            to_col: CCol {
                col_abs: false,
                col: to_col,
            },
        }
    }

    /// External file
    pub fn set_iri<S: Into<String>>(&mut self, iri: S) {
        self.iri = Some(iri.into());
    }

    /// External file
    pub fn iri(&self) -> Option<&String> {
        self.iri.as_ref()
    }

    /// Table name for references into other tables.
    pub fn set_table<S: Into<String>>(&mut self, table: S) {
        self.from_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn table(&self) -> Option<&String> {
        self.from_table.as_ref()
    }

    /// Column
    pub fn set_col(&mut self, col: u32) {
        self.from_col.col = col;
    }

    /// Column
    pub fn col(&self) -> u32 {
        self.from_col.col
    }

    /// "$" column reference
    pub fn set_col_abs(&mut self, abs: bool) {
        self.from_col.col_abs = abs;
    }

    /// "$" column reference
    pub fn col_abs(&self) -> bool {
        self.from_col.col_abs
    }

    /// Table name for references into other tables.
    pub fn set_to_table<S: Into<String>>(&mut self, table: S) {
        self.to_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn to_table(&self) -> Option<&String> {
        self.to_table.as_ref()
    }

    /// To column
    pub fn set_to_col(&mut self, to_col: u32) {
        self.to_col.col = to_col;
    }

    /// To column
    pub fn to_col(&self) -> u32 {
        self.to_col.col
    }

    /// "$" column reference
    pub fn set_to_col_abs(&mut self, abs: bool) {
        self.to_col.col_abs = abs;
    }

    /// "$" column reference
    pub fn to_col_abs(&self) -> bool {
        self.to_col.col_abs
    }

    /// Returns a range reference for a formula.
    pub fn to_formula(&self) -> String {
        let mut buf = String::new();
        buf.push('[');
        let _ = fmt_col_range(&mut buf, self);
        buf.push(']');
        buf
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.from_col.col_abs = true;
        self.to_col.col_abs = true;
        self
    }

    /// Is the column in this range.
    /// The range is inclusive with the to_col.
    pub fn contains(&self, col: u32) -> bool {
        col >= self.from_col.col && col <= self.to_col.col
    }
}

impl TryFrom<&str> for ColRange {
    type Error = OdsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_colrange(s)
    }
}

impl Display for ColRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_col_range(f, self)
    }
}

/// A range over rows.
#[derive(Debug, Default, Clone, PartialEq, Eq, GetSize)]
pub struct RowRange {
    /// External reference
    iri: Option<String>,
    /// Reference to another sheet.
    from_table: Option<String>,
    /// Row.
    from_row: CRow,
    /// Reference to a second sheet. Only needed if it's different than the
    /// first one.
    to_table: Option<String>,
    /// Row.
    to_row: CRow,
}

impl RowRange {
    /// New with all possible parameters.
    pub fn new_all(
        iri: Option<String>,
        from_table: Option<String>,
        from_row_abs: bool,
        from_row: u32,
        to_table: Option<String>,
        to_row_abs: bool,
        to_row: u32,
    ) -> Self {
        Self {
            iri,
            from_table,
            from_row: CRow {
                row_abs: from_row_abs,
                row: from_row,
            },
            to_table,
            to_row: CRow {
                row_abs: to_row_abs,
                row: to_row,
            },
        }
    }

    /// New range.
    pub fn new(from_row: u32, to_row: u32) -> Self {
        assert!(from_row <= to_row);
        Self {
            iri: None,
            from_table: None,
            from_row: CRow {
                row_abs: false,
                row: from_row,
            },
            to_table: None,
            to_row: CRow {
                row_abs: false,
                row: to_row,
            },
        }
    }

    /// External file
    pub fn set_iri<S: Into<String>>(&mut self, iri: S) {
        self.iri = Some(iri.into());
    }

    /// External file
    pub fn iri(&self) -> Option<&String> {
        self.iri.as_ref()
    }

    /// Table name for references into other tables.
    pub fn set_table<S: Into<String>>(&mut self, table: S) {
        self.from_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn table(&self) -> Option<&String> {
        self.from_table.as_ref()
    }

    /// Row
    pub fn row(&self) -> u32 {
        self.from_row.row
    }

    /// Row
    pub fn set_row(&mut self, row: u32) {
        self.from_row.row = row;
    }

    /// "$" row reference
    pub fn set_row_abs(&mut self, abs: bool) {
        self.from_row.row_abs = abs;
    }

    /// "$" row reference
    pub fn row_abs(&self) -> bool {
        self.from_row.row_abs
    }

    /// Table name for references into other tables.
    pub fn set_to_table<S: Into<String>>(&mut self, table: S) {
        self.to_table = Some(table.into());
    }

    /// Table name for references into other tables.
    pub fn to_table(&self) -> Option<&String> {
        self.to_table.as_ref()
    }

    /// To row
    pub fn to_row(&self) -> u32 {
        self.to_row.row
    }

    /// To row
    pub fn set_to_row(&mut self, row: u32) {
        self.to_row.row = row;
    }

    /// "$" row reference
    pub fn set_to_row_abs(&mut self, abs: bool) {
        self.to_row.row_abs = abs;
    }

    /// "$" row reference
    pub fn to_row_abs(&self) -> bool {
        self.to_row.row_abs
    }

    /// Returns a range reference for a formula.
    pub fn to_formula(&self) -> String {
        let mut buf = String::new();
        buf.push('[');
        let _ = fmt_row_range(&mut buf, self);
        buf.push(']');
        buf
    }

    /// Makes this CellReference into an absolute reference.
    pub fn absolute(mut self) -> Self {
        self.from_row.row_abs = true;
        self.to_row.row_abs = true;
        self
    }

    /// Is the row in this range.
    /// The range is inclusive with the to_row.
    pub fn contains(&self, row: u32) -> bool {
        row >= self.from_row.row && row <= self.to_row.row
    }
}

impl TryFrom<&str> for RowRange {
    type Error = OdsError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse_rowrange(s)
    }
}

impl Display for RowRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_row_range(f, self)
    }
}

mod format_refs {
    use crate::refs::format::{fmt_abs, fmt_col_name, fmt_row_name};
    use crate::refs::{CCol, CRow};
    use crate::{CellRange, CellRef, ColRange, RowRange};
    use std::fmt;

    /// Appends a simple cell reference.
    pub(crate) fn fmt_row(f: &mut impl fmt::Write, row: &CRow) -> fmt::Result {
        fmt_abs(f, row.row_abs())?;
        fmt_row_name(f, row.row())?;
        Ok(())
    }

    /// Appends a simple cell reference.
    pub(crate) fn fmt_col(f: &mut impl fmt::Write, col: &CCol) -> fmt::Result {
        fmt_abs(f, col.col_abs())?;
        fmt_col_name(f, col.col())?;
        Ok(())
    }

    /// Appends the cell reference
    pub(crate) fn fmt_cell_ref(f: &mut impl fmt::Write, cell_ref: &CellRef) -> fmt::Result {
        fmt_iri(f, cell_ref.iri())?;
        if let Some(sheet) = cell_ref.table().as_ref() {
            fmt_table_name(
                f,
                sheet,
                cell_ref.iri().is_some() || cell_ref.col_abs() || cell_ref.row_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_col(f, &cell_ref.col)?;
        fmt_row(f, &cell_ref.row)?;
        Ok(())
    }

    /// Appends the range reference
    pub(crate) fn fmt_cell_range(f: &mut impl fmt::Write, cell_range: &CellRange) -> fmt::Result {
        fmt_iri(f, cell_range.iri())?;
        if let Some(table) = cell_range.table().as_ref() {
            fmt_table_name(
                f,
                table,
                cell_range.iri().is_some()
                    || cell_range.from_row.row_abs()
                    || cell_range.from_col.col_abs()
                    || cell_range.to_row.row_abs()
                    || cell_range.to_col.col_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_col(f, &cell_range.from_col)?;
        fmt_row(f, &cell_range.from_row)?;
        write!(f, ":")?;
        if let Some(to_table) = cell_range.to_table().as_ref() {
            fmt_table_name(
                f,
                to_table,
                cell_range.iri().is_some()
                    || cell_range.from_row.row_abs()
                    || cell_range.from_col.col_abs()
                    || cell_range.to_row.row_abs()
                    || cell_range.to_col.col_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_col(f, &cell_range.to_col)?;
        fmt_row(f, &cell_range.to_row)?;
        Ok(())
    }

    /// Appends the cell reference
    pub(crate) fn fmt_col_range(f: &mut impl fmt::Write, col_range: &ColRange) -> fmt::Result {
        fmt_iri(f, col_range.iri())?;
        if let Some(sheet) = col_range.table().as_ref() {
            fmt_table_name(
                f,
                sheet,
                col_range.iri().is_some() || col_range.col_abs() || col_range.to_col_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_col(f, &col_range.from_col)?;
        write!(f, ":")?;
        if let Some(to_sheet) = col_range.to_table().as_ref() {
            fmt_table_name(
                f,
                to_sheet,
                col_range.iri().is_some() || col_range.col_abs() || col_range.to_col_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_col(f, &col_range.to_col)?;
        Ok(())
    }

    /// Appends the cell reference
    pub(crate) fn fmt_row_range(f: &mut impl fmt::Write, row_range: &RowRange) -> fmt::Result {
        fmt_iri(f, row_range.iri())?;
        if let Some(table) = row_range.table().as_ref() {
            fmt_table_name(
                f,
                table,
                row_range.iri().is_some() || row_range.row_abs() || row_range.to_row_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_row(f, &row_range.from_row)?;
        write!(f, ":")?;
        if let Some(to_table) = row_range.to_table().as_ref() {
            fmt_table_name(
                f,
                to_table,
                row_range.iri().is_some() || row_range.row_abs() || row_range.to_row_abs(),
            )?;
        }
        write!(f, ".")?;
        fmt_row(f, &row_range.to_row)?;
        Ok(())
    }

    /// Appends the IRI
    pub(crate) fn fmt_iri(f: &mut impl fmt::Write, iri: Option<&String>) -> fmt::Result {
        if let Some(iri) = iri {
            write!(f, "'")?;
            write!(f, "{}", &iri.replace('\'', "''"))?;
            write!(f, "'")?;
            write!(f, "#")?;
        }

        Ok(())
    }

    /// Appends the table-name
    pub(crate) fn fmt_table_name(
        f: &mut impl fmt::Write,
        table_name: &str,
        abs: bool,
    ) -> fmt::Result {
        fmt_abs(f, abs)?;
        if table_name.contains(|c| c == '\'' || c == ' ' || c == '.') {
            write!(f, "'")?;
            write!(f, "{}", &table_name.replace('\'', "''"))?;
            write!(f, "'")?;
        } else {
            write!(f, "{}", table_name)?;
        }
        Ok(())
    }
}

/// Parse a cell reference.
pub fn parse_cellref(buf: &str) -> Result<CellRef, OdsError> {
    let trk: StdTracker<CRCode, _> = Track::new_tracker();
    let span = Track::new_span(&trk, buf);

    let (rest, tok) = parser::parse_cell_ref(span)?;
    if rest.len() > 0 {
        Err(nom::Err::Error(KTokenizerError::new(CRCellRef, rest)))?
    } else {
        Ok(tok)
    }
}

/// Parse a cell reference.
pub fn parse_cellrange(buf: &str) -> Result<CellRange, OdsError> {
    let trk: StdTracker<CRCode, _> = Track::new_tracker();
    let span = Track::new_span(&trk, buf);

    let (rest, tok) = parser::parse_cell_range(span)?;
    if rest.len() > 0 {
        Err(nom::Err::Error(KTokenizerError::new(CRCellRange, rest)))?
    } else {
        Ok(tok)
    }
}

/// Parse a cell reference.
pub fn parse_colrange(buf: &str) -> Result<ColRange, OdsError> {
    let trk: StdTracker<CRCode, _> = Track::new_tracker();
    let span = Track::new_span(&trk, buf);

    let (rest, tok) = parser::parse_col_range(span)?;
    if rest.len() > 0 {
        Err(nom::Err::Error(KTokenizerError::new(CRColRange, rest)))?
    } else {
        Ok(tok)
    }
}

/// Parse a cell reference.
pub fn parse_rowrange(buf: &str) -> Result<RowRange, OdsError> {
    let trk: StdTracker<CRCode, _> = Track::new_tracker();
    let span = Track::new_span(&trk, buf);

    let (rest, tok) = parser::parse_row_range(span)?;
    if rest.len() > 0 {
        Err(nom::Err::Error(KTokenizerError::new(CRRowRange, rest)))?
    } else {
        Ok(tok)
    }
}

/// Parse a list of range refs
pub fn parse_cellranges(buf: &str) -> Result<Option<Vec<CellRange>>, OdsError> {
    let trk: StdTracker<CRCode, _> = Track::new_tracker();
    let span = Track::new_span(&trk, buf);

    match parser::parse_cell_range_list(span) {
        Ok((_, ranges)) => Ok(ranges),
        Err(err) => Err(err.into()),
    }
}

pub(crate) fn format_cellranges(v: &[CellRange]) -> impl Display + '_ {
    struct Tmp<'f>(&'f [CellRange]);

    impl<'f> Display for Tmp<'f> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for (i, range) in self.0.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", range)?;
            }
            Ok(())
        }
    }

    Tmp(v)
}
