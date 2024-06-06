#![doc = include_str!("../crate.md")]

pub use color;

pub use crate::cell_::{CellContent, CellContentRef};
pub use crate::error::{OdsError, OdsResult};
pub use crate::format::{
    ValueFormatBoolean, ValueFormatCurrency, ValueFormatDateTime, ValueFormatNumber,
    ValueFormatPercentage, ValueFormatRef, ValueFormatText, ValueFormatTimeDuration,
};
pub use crate::io::read::{
    read_fods, read_fods_buf, read_fods_from, read_ods, read_ods_buf, read_ods_from, OdsOptions,
};
pub use crate::io::write::{
    write_fods, write_fods_buf, write_fods_to, write_ods, write_ods_buf,
    write_ods_buf_uncompressed, write_ods_to,
};
pub use crate::refs::{CCol, CRow, CellRange, CellRef, ColRange, RowRange};
pub use crate::sheet_::Sheet;
pub use crate::style::units::{Angle, Length};
pub use crate::style::{CellStyle, CellStyleRef};
pub use crate::value_::{Value, ValueType};
// pub mod value {
// }
pub use crate::workbook_::WorkBook;

#[macro_use]
mod macro_attr_draw;
#[macro_use]
mod macro_attr_style;
#[macro_use]
mod macro_attr_fo;
#[macro_use]
mod macro_attr_svg;
#[macro_use]
mod macro_attr_text;
#[macro_use]
mod macro_attr_number;
#[macro_use]
mod macro_attr_table;
#[macro_use]
mod macro_attr_xlink;
#[macro_use]
mod macro_units;
#[macro_use]
mod macro_format;
#[macro_use]
mod macro_style;
#[macro_use]
mod macro_text;

mod attrmap2;
mod cell_;
mod config;
mod ds;
mod error;
mod io;
mod locale;
mod sheet_;
#[macro_use]
mod value_;
mod workbook_;

pub mod cell {
    //! Detail structs for a Cell.
    pub use crate::cell_::CellSpan;
}
pub mod condition;
pub mod defaultstyles;
pub mod draw;
pub mod format;
#[macro_use]
pub mod formula;
pub mod manifest;
pub mod metadata;
pub mod refs;
pub mod sheet {
    //! Detail structs for a Sheet.
    pub use crate::sheet_::{CellIter, Grouped, Range, SheetConfig, SplitMode, Visibility};
}
pub mod style;
pub mod text;
pub mod validation;
pub mod workbook {
    //! Detail structs for the WorkBook.
    pub use crate::workbook_::{EventListener, Script, WorkBookConfig};
}
pub mod xlink;
pub mod xmltree;

// Use the IndexMap for debugging, makes diffing much easier.
// Otherwise the std::HashMap is good.
// pub(crate) type HashMap<K, V> = indexmap::IndexMap<K, V>;
// pub(crate) type HashMapIter<'a, K, V> = indexmap::map::Iter<'a, K, V>;
pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V>;
// pub(crate) type HashMapIter<'a, K, V> = std::collections::hash_map::Iter<'a, K, V>;
