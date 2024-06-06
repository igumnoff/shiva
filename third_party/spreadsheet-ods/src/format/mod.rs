//!
//! Defines one ValueFormatXX per ValueType for textual formatting of those values.
//!
//! ```
//! use spreadsheet_ods::{ValueType};
//! use spreadsheet_ods::format::{FormatCalendarStyle, FormatNumberStyle, ValueFormatDateTime, ValueFormatNumber};
//!
//! let mut v = ValueFormatDateTime::new_named("dt0");
//! v.part_day().long_style().build();
//! v.part_text(".").build();
//! v.part_month().long_style().build();
//! v.part_text(".").build();
//! v.part_year().long_style().build();
//! v.part_text(" ").build();
//! v.part_hours().long_style().build();
//! v.part_text(":").build();
//! v.part_minutes().long_style().build();
//! v.part_text(":").build();
//! v.part_seconds().long_style().build();
//!
//! let mut v = ValueFormatNumber::new_named("n3");
//! v.part_number().decimal_places(3).build();
//! ```
//!

// This cares about the following parts of office:styles and office:automatic-styles.
//
// <number:boolean-style> 16.29.24,
// <number:currency-style> 16.29.8,
// <number:date-style> 16.29.11,
// <number:number-style> 16.29.2,
// <number:percentage-style> 16.29.10,
// <number:text-style> 16.29.26,
// <number:time-style> 16.29.19,
//

mod builder;
mod create;
mod stylemap;

pub use builder::*;
pub use create::*;
pub use stylemap::*;

use crate::attrmap2::AttrMap2;
use crate::color::Rgb;
use crate::style::units::{
    Angle, FontSize, FontStyle, FontVariant, FontWeight, FormatSource, Length, LetterSpacing,
    LineMode, LineStyle, LineType, LineWidth, Percent, RotationScale, TextCombine, TextCondition,
    TextDisplay, TextEmphasize, TextEmphasizePosition, TextPosition, TextRelief, TextTransform,
    TransliterationStyle,
};
use crate::style::AnyStyleRef;
use crate::style::ParseStyleAttr;
use crate::style::{
    color_string, shadow_string, text_position, StyleOrigin, StyleUse, TextStyleRef,
};
use crate::{OdsError, ValueType};
use core::borrow::Borrow;
use get_size::GetSize;
use get_size_derive::GetSize;
use icu_locid::subtags::{Language, Region, Script};
use icu_locid::{LanguageIdentifier, Locale};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

style_ref2!(ValueFormatRef);

/// Trait used by the builder types.
pub trait ValueFormatTrait {
    /// Returns a reference name for this value format.
    fn format_ref(&self) -> ValueFormatRef;

    /// The style:name attribute specifies names that reference style mechanisms.
    fn set_name<S: Into<String>>(&mut self, name: S);

    /// The style:name attribute specifies names that reference style mechanisms.
    fn name(&self) -> &String;

    /// Returns the value type.
    fn value_type(&self) -> ValueType;

    /// Sets the storage location for this ValueFormat. Either content.xml
    /// or styles.xml.
    fn set_origin(&mut self, origin: StyleOrigin);

    /// Returns the storage location.
    fn origin(&self) -> StyleOrigin;

    /// How is the style used in the document.
    fn set_styleuse(&mut self, styleuse: StyleUse);

    /// How is the style used in the document.
    fn styleuse(&self) -> StyleUse;

    /// All direct attributes of the number:xxx-style tag.
    fn attrmap(&self) -> &AttrMap2;

    /// All direct attributes of the number:xxx-style tag.
    fn attrmap_mut(&mut self) -> &mut AttrMap2;

    /// Text style attributes.
    fn textstyle(&self) -> &AttrMap2;

    /// Text style attributes.
    fn textstyle_mut(&mut self) -> &mut AttrMap2;

    /// Adds a format part.
    fn push_part(&mut self, part: FormatPart);

    /// Adds all format parts.
    fn push_parts(&mut self, partvec: &mut Vec<FormatPart>);

    /// Returns the parts.
    fn parts(&self) -> &Vec<FormatPart>;

    /// Returns the mutable parts.
    fn parts_mut(&mut self) -> &mut Vec<FormatPart>;

    /// Adds a stylemap.
    fn push_stylemap(&mut self, stylemap: ValueStyleMap);

    /// Returns the stylemaps
    fn stylemaps(&self) -> Option<&Vec<ValueStyleMap>>;

    /// Returns the mutable stylemap.
    fn stylemaps_mut(&mut self) -> &mut Vec<ValueStyleMap>;
}

valueformat!(ValueFormatBoolean, ValueType::Boolean);

// 16.29.24 <number:boolean-style>
impl ValueFormatBoolean {
    part_boolean!();

    push_boolean!();
}

// 16.29.2 <number:number-style>
valueformat!(ValueFormatNumber, ValueType::Number);

impl ValueFormatNumber {
    part_fill_character!();
    part_fraction!();
    part_number!();
    part_scientific!();
    part_text!();

    push_fraction!();
    push_number!();
    push_number_fix!();
    push_scientific!();
    push_text!();
}

// 16.29.10 <number:percentage-style>
valueformat!(ValueFormatPercentage, ValueType::Percentage);

impl ValueFormatPercentage {
    part_fill_character!();
    part_number!();
    part_text!();

    push_number!();
    push_number_fix!();
    push_text!();
}

// 16.29.8 <number:currency-style>
valueformat!(ValueFormatCurrency, ValueType::Currency);

impl ValueFormatCurrency {
    number_automatic_order!(attr);

    part_currency!();
    part_fill_character!();
    part_number!();
    part_text!();

    push_currency_symbol!();
    push_number!();
    push_number_fix!();
    push_text!();
}

// 16.29.26 <number:text-style>
valueformat!(ValueFormatText, ValueType::Text);

impl ValueFormatText {
    part_fill_character!();
    part_text!();
    part_text_content!();

    push_text!();
    push_text_content!();
}

// 16.29.11 <number:date-style>
valueformat!(ValueFormatDateTime, ValueType::DateTime);

impl ValueFormatDateTime {
    number_automatic_order!(attr);
    number_format_source!(attr);

    part_am_pm!();
    part_day!();
    part_day_of_week!();
    part_era!();
    part_fill_character!();
    part_hours!();
    part_minutes!();
    part_month!();
    part_quarter!();
    part_seconds!();
    part_text!();
    part_week_of_year!();
    part_year!();

    push_am_pm!();
    push_day!();
    push_day_of_week!();
    push_era!();
    push_hours!();
    push_minutes!();
    push_month!();
    push_quarter!();
    push_seconds!();
    push_text!();
    push_week_of_year!();
    push_year!();
}

// 16.29.19 <number:time-style>
valueformat!(ValueFormatTimeDuration, ValueType::TimeDuration);

impl ValueFormatTimeDuration {
    number_format_source!(attr);
    number_truncate_on_overflow!(attr);

    part_am_pm!();
    part_fill_character!();
    part_hours!();
    part_minutes!();
    part_seconds!();
    part_text!();

    push_am_pm!();
    push_hours!();
    push_minutes!();
    push_seconds!();
    push_text!();
}

/// Identifies the structural parts of a value format.
#[derive(Debug, Clone, Copy, Eq, PartialEq, GetSize)]
#[allow(missing_docs)]
pub enum FormatPartType {
    Number,
    FillCharacter,
    ScientificNumber,
    Fraction,
    CurrencySymbol,
    Day,
    Month,
    Year,
    Era,
    DayOfWeek,
    WeekOfYear,
    Quarter,
    Hours,
    Minutes,
    Seconds,
    AmPm,
    Boolean,
    Text,
    TextContent,
}

/// One structural part of a value format.
#[derive(Debug, Clone, GetSize)]
pub struct FormatPart {
    /// What kind of format part is this?
    part_type: FormatPartType,
    /// Properties of this part.
    attr: AttrMap2,
    /// Textposition for embedded text when acting as a number format part.
    ///
    /// The number:position attribute specifies the position where text appears.
    /// The index of a position starts with 1 and is counted by digits from right to left in the integer part of
    /// a number, starting left from a decimal separator if one exists, or from the last digit of the number.
    /// Text is inserted before the digit at the specified position. If the value of number:position
    /// attribute is greater than the value of number:min-integer-digits and greater than
    /// the number of integer digits in the number, text is prepended to the number.
    position: Option<i32>,
    /// Some content.
    content: Option<String>,
}

/// Flag for several PartTypes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FormatNumberStyle {
    Short,
    Long,
}

impl Display for FormatNumberStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FormatNumberStyle::Short => write!(f, "short"),
            FormatNumberStyle::Long => write!(f, "long"),
        }
    }
}

/// Calendar types.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FormatCalendarStyle {
    Gregorian,
    Gengou,
    Roc,
    Hanja,
    Hijri,
    Jewish,
    Buddhist,
}

impl Display for FormatCalendarStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FormatCalendarStyle::Gregorian => write!(f, "gregorian"),
            FormatCalendarStyle::Gengou => write!(f, "gengou"),
            FormatCalendarStyle::Roc => write!(f, "ROC"),
            FormatCalendarStyle::Hanja => write!(f, "hanja"),
            FormatCalendarStyle::Hijri => write!(f, "hijri"),
            FormatCalendarStyle::Jewish => write!(f, "jewish"),
            FormatCalendarStyle::Buddhist => write!(f, "buddhist"),
        }
    }
}

impl FormatPart {
    /// New, empty
    pub fn new(ftype: FormatPartType) -> Self {
        FormatPart {
            part_type: ftype,
            attr: Default::default(),
            position: None,
            content: None,
        }
    }

    /// Sets the kind of the part.
    pub fn set_part_type(&mut self, p_type: FormatPartType) {
        self.part_type = p_type;
    }

    /// What kind of part?
    pub fn part_type(&self) -> FormatPartType {
        self.part_type
    }

    /// General attributes.
    pub(crate) fn attrmap(&self) -> &AttrMap2 {
        &self.attr
    }

    /// General attributes.
    pub(crate) fn attrmap_mut(&mut self) -> &mut AttrMap2 {
        &mut self.attr
    }

    /// Adds an attribute.
    pub fn set_attr(&mut self, name: &str, value: String) {
        self.attr.set_attr(name, value);
    }

    /// Returns a property or a default.
    pub fn attr_def<'a, 'b, S>(&'a self, name: &'b str, default: S) -> &'a str
    where
        S: Into<&'a str>,
    {
        self.attr.attr_def(name, default)
    }

    /// Sets the position for embedded text in a number format part.
    pub fn set_position(&mut self, pos: i32) {
        self.position = Some(pos);
    }

    /// Clear the position for embedded text in a number format part.
    pub fn clear_position(&mut self) {
        self.position = None;
    }

    /// The position for embedded text in a number format part.
    pub fn position(&self) -> Option<i32> {
        self.position
    }

    /// Sets a textual content for this part. This is only used
    /// for text and currency-symbol.
    pub fn set_content<S: Into<String>>(&mut self, content: S) {
        self.content = Some(content.into());
    }

    /// Clear the textual content for this part. This is only used
    /// for text and currency-symbol.
    pub fn clear_content(&mut self) {
        self.content = None;
    }

    /// Returns the text content.
    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }
}
