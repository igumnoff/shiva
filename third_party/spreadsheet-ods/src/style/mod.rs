//! Styles define a large number of attributes.
//!
//! They are split along style:family into separate structs like CellStyle,
//! ParagraphStyle etc. These are the main building blocks that will be used
//! to set the properties of each style.
//!
//! Such a style has to be added to the workbook, which returns a reference
//! to the added style (CellStyle -> CellStyleRef). These provide only a loose
//! coupling to the style itself, but allow a better differentiation of the
//! different style-families. Wherever a style can be used a reference
//! of the correct type is expected.
//!
//! ```
//! use spreadsheet_ods::{CellRef, Sheet, WorkBook};
//! use spreadsheet_ods::style::{StyleOrigin, StyleUse, CellStyle};
//! use spreadsheet_ods::color::Rgb;
//! use icu_locid::locale;
//! use spreadsheet_ods::style::stylemap::StyleMap;
//! use spreadsheet_ods::condition::{Condition};
//!
//! let mut wb = WorkBook::new(locale!("en_US"));
//!
//! let mut cs1 = CellStyle::new("ce12", &"num2".into());
//! cs1.set_color(Rgb::new(192, 128, 0));
//! cs1.set_font_bold();
//! let cs1 = wb.add_cellstyle(cs1);
//!
//! let mut cs2 = CellStyle::new("ce11", &"num2".into());
//! cs2.set_color(Rgb::new(0, 192, 128));
//! cs2.set_font_bold();
//! let cs2 = wb.add_cellstyle(cs2);
//!
//! let mut cs3 = CellStyle::new("ce13", &"num4".into());
//! cs3.push_stylemap(StyleMap::new(Condition::content_eq("BB"), "ce12".into(), Some(CellRef::remote("sheet0", 4, 3))));
//! cs3.push_stylemap(StyleMap::new(Condition::content_eq("CC"), "ce11".into(), Some(CellRef::remote("sheet0", 4, 3))));
//! let cs3 = wb.add_cellstyle(cs3);
//!
//!
//!
//! let mut sheet = Sheet::new("sample");
//! sheet.set_styled_value(0, 0, 1234, &cs1);
//! sheet.set_styled_value(0, 1, 5678, &cs2);
//!
//! ```
//!
//! From the specification:
//!
//! The style:style element represents styles.
//!
//! Styles defined by the style:style element use a hierarchical style model. The
//! style:style element supports inheritance of formatting properties by a style from its parent
//! style. A parent style is specified by the style:parent-style-name attribute on a
//! style:style element.
//!
//! The determination of the value of a formatting property begins with any style that is specified by
//! an element. If the formatting property is present in that style, its value is used.
//! If that style does not specify a value for that formatting property and it has a parent style, the value
//! of the formatting element is taken from the parent style, if present.
//! If the parent style does not have a value for the formatting property, the search for the formatting
//! property value continues up parent styles until either the formatting property has been found or a
//! style is found with no parent style.
//! If a search of the parent styles of a style does not result in a value for a formatting property, the
//! determination of its value depends on the style family and the element to which a style is applied.
//! For styles with family text which are applied to elements which are contained in another element
//! that specifies a style with family text, the search continues within the text style that is applied
//! to the nearest ancestor element that specifies a style with family text, and continues in its parent
//! styles.
//!
//! For styles with family text which are applied to elements which are contained in a paragraph
//! element 6.1.1, the search continues within the paragraph style that is applied to the paragraph
//! element, and continues in its parent styles.
//! For styles with family paragraph which are applied to paragraph elements which are contained
//! in a drawing shape or a chart element, the search continues within the graphic, presentation
//! or chart style that is applied to the drawing object or chart element, and continues in its parent
//! styles.
//! For styles with family paragraph which are applied to paragraph elements which are contained
//! in a table cell, the search continues within the table-cell style that is applied to the table-cell,
//! and continues in its parent styles. If a value for the formatting property has not been found, the
//! search continues as defined for styles with family table-cell.
//!
//! For styles with family table-cell which are applied to a table cell, the search continues with the
//! style specified by the table:default-cell-style-name attribute 19.619 of the table cell's
//! table:table-row parent element, if present, and then with the style specified by the
//! table:default-cell-style-name attribute of the table:table-column element
//! associated with the table cell.
//!
//! In all other cases, or if a value for the formatting property has not been found by any of the family
//! specific rules, a default style 16.4 that has the same family as the style initially declared sets the
//! value. If a value has not been found by these steps, but this specification defines a default value,
//! then this default value is used. In all remaining cases an implementation-dependent value is used.

use crate::color::Rgb;
use crate::style::units::{Border, Length, Percent, TextPosition};
use crate::OdsError;
use get_size::GetSize;
use get_size_derive::GetSize;
use std::borrow::Borrow;
use std::str::FromStr;

pub use cellstyle::*;
pub use colstyle::*;
pub use fontface::*;
pub use graphicstyle::*;
pub use masterpage::*;
pub use pagestyle::*;
pub use paragraphstyle::*;
pub use rowstyle::*;
pub use ruby::*;
pub use tablestyle::*;
pub use textstyle::*;

pub mod stylemap;
pub mod tabstop;
pub mod units;

mod cellstyle;
mod colstyle;
mod fontface;
mod graphicstyle;
mod masterpage;
mod pagestyle;
mod paragraphstyle;
mod rowstyle;
mod ruby;
mod tablestyle;
mod textstyle;

// The <style:style> element has the following attributes:
// ok: style:auto-update 19.467,
// ok: style:class 19.470,
// only for table-cell: style:data-style-name 19.473,
// only for paragraph: style:default-outlinelevel 19.474,
// ok: style:display-name 19.476,
// mapped as separate XxxStyle structs: style:family 19.480,
// not used: style:list-level 19.499,
// not used: style:list-style-name 19.500,
// only for table and paragraph: style:master-page-name 19.501,
// ok: style:name 19.502,
// only for paragraph: style:next-style-name 19.503,
// ok: style:parent-style-name 19.510 and
// only for chart: style:percentage-data-style-name 19.511

/// Origin of a style. Content.xml or Styles.xml.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, GetSize)]
pub enum StyleOrigin {
    /// Style comes from Content.xml
    #[default]
    Content,
    /// Style comes from Styles.xml
    Styles,
}

/// Placement of a style. office:styles or office:automatic-styles
/// Defines the usage pattern for the style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, GetSize)]
pub enum StyleUse {
    /// The style:default-style element represents default styles. A default style specifies
    /// default formatting properties for a style family. These defaults are used if a formatting property is
    /// neither specified by an automatic nor a common style. Default styles exist for all style families that
    /// are represented by the style:style element specified by the style:family attribute
    /// 19.480.
    /// An OpenDocument document should contain the default styles of the style families for which are
    /// used in common or automatic styles in the document.
    Default,
    /// The office:styles element contains common styles used in a document. A common style
    /// is a style chosen by a user for a document or portion thereof.
    Named,
    /// The office:automatic-styles element contains automatic styles used in a document.
    /// An automatic style is a set of formatting properties treated as properties of the object to which the
    /// style is assigned.
    ///
    /// Note: Common and automatic styles behave differently in OpenDocument editing
    /// consumers. Common styles present to a user as a named set of formatting
    /// properties. The formatting properties of an automatic style present to a user as
    /// properties of the object to which the style is applied.
    #[default]
    Automatic,
}

// General style reference.
style_ref2_base!(AnyStyleRef);

/// Parses an attribute string to a value type.
pub(crate) trait ParseStyleAttr<T> {
    fn parse_attr(attr: Option<&str>) -> Result<Option<T>, OdsError>;

    fn parse_attr_def(attr: Option<&str>, default: T) -> Result<T, OdsError> {
        match Self::parse_attr(attr)? {
            None => Ok(default),
            Some(v) => Ok(v),
        }
    }
}

impl ParseStyleAttr<bool> for bool {
    fn parse_attr(attr: Option<&str>) -> Result<Option<bool>, OdsError> {
        if let Some(s) = attr {
            Ok(Some(bool::from_str(s)?))
        } else {
            Ok(None)
        }
    }
}

pub(crate) fn color_string(color: Rgb<u8>) -> String {
    format!("#{:02x}{:02x}{:02x}", color.r, color.g, color.b)
}

pub(crate) fn shadow_string(
    x_offset: Length,
    y_offset: Length,
    blur: Option<Length>,
    color: Rgb<u8>,
) -> String {
    if let Some(blur) = blur {
        format!("{} {} {} {}", color_string(color), x_offset, y_offset, blur)
    } else {
        format!("{} {} {}", color_string(color), x_offset, y_offset)
    }
}

pub(crate) fn rel_width_string(value: f64) -> String {
    format!("{}*", value)
}

pub(crate) fn border_string(width: Length, border: Border, color: Rgb<u8>) -> String {
    format!(
        "{} {} #{:02x}{:02x}{:02x}",
        width, border, color.r, color.g, color.b
    )
}

pub(crate) fn border_line_width_string(inner: Length, space: Length, outer: Length) -> String {
    format!("{} {} {}", inner, space, outer)
}

pub(crate) fn text_position(pos: TextPosition, scale: Option<Percent>) -> String {
    if let Some(scale) = scale {
        format!("{} {}", pos, scale)
    } else {
        format!("{}", pos)
    }
}
