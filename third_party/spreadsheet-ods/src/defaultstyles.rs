//!
//! Creates default formats for a new Workbook.
//!

use crate::format::ValueFormatRef;
use crate::style::CellStyle;
use crate::{format, CellStyleRef, ValueType, WorkBook};
use icu_locid::locale;

///
/// Allows access to the value-format names for the default formats
/// as created by create_default_styles.
///
#[derive(Debug)]
pub struct DefaultFormat {}

impl DefaultFormat {
    /// Default format.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> ValueFormatRef {
        ValueFormatRef::from("")
    }

    /// Default boolean format.
    pub fn bool() -> ValueFormatRef {
        ValueFormatRef::from("bool1")
    }

    /// Default number format.
    pub fn number() -> ValueFormatRef {
        ValueFormatRef::from("num1")
    }

    /// Default percentage format.
    pub fn percent() -> ValueFormatRef {
        ValueFormatRef::from("percent1")
    }

    /// Default currency format.
    pub fn currency() -> ValueFormatRef {
        ValueFormatRef::from("currency1")
    }

    /// Default date format.
    pub fn date() -> ValueFormatRef {
        ValueFormatRef::from("date1")
    }

    /// Default datetime format.
    pub fn datetime() -> ValueFormatRef {
        ValueFormatRef::from("datetime1")
    }

    /// Default time format.
    pub fn time_of_day() -> ValueFormatRef {
        ValueFormatRef::from("time1")
    }

    /// Default time format.
    pub fn time_interval() -> ValueFormatRef {
        ValueFormatRef::from("interval1")
    }
}

///
/// Allows access to the names of the default styles as created by
/// create_default_styles.
///
#[derive(Debug)]
pub struct DefaultStyle {}

impl DefaultStyle {
    /// Default bool style.
    pub fn bool() -> CellStyleRef {
        CellStyleRef::from("default-bool")
    }

    /// Default number style.
    pub fn number() -> CellStyleRef {
        CellStyleRef::from("default-num")
    }

    /// Default percent style.
    pub fn percent() -> CellStyleRef {
        CellStyleRef::from("default-percent")
    }

    /// Default currency style.
    pub fn currency() -> CellStyleRef {
        CellStyleRef::from("default-currency")
    }

    /// Default date style.
    pub fn date() -> CellStyleRef {
        CellStyleRef::from("default-date")
    }

    /// Default datetime style.
    pub fn datetime() -> CellStyleRef {
        CellStyleRef::from("default-datetime")
    }

    /// Default time style.
    pub fn time_of_day() -> CellStyleRef {
        CellStyleRef::from("default-time")
    }

    /// Default time style.
    pub fn time_interval() -> CellStyleRef {
        CellStyleRef::from("default-interval")
    }
}

/// Replaced with WorkBook::locale_settings() or WorkBook::new(l: Locale).
#[deprecated]
pub fn create_default_styles(book: &mut WorkBook) {
    book.add_boolean_format(format::create_boolean_format(DefaultFormat::bool()));
    book.add_number_format(format::create_number_format(
        DefaultFormat::number(),
        2,
        false,
    ));
    book.add_percentage_format(format::create_percentage_format(
        DefaultFormat::percent(),
        2,
    ));
    book.add_currency_format(format::create_currency_prefix(
        DefaultFormat::currency(),
        locale!("de_AT"),
        "€",
    ));
    book.add_datetime_format(format::create_date_dmy_format(DefaultFormat::date()));
    book.add_datetime_format(format::create_datetime_format(DefaultFormat::datetime()));
    book.add_timeduration_format(format::create_time_of_day_format(
        DefaultFormat::time_of_day(),
    ));
    book.add_timeduration_format(format::create_time_interval_format(
        DefaultFormat::time_interval(),
    ));

    book.add_cellstyle(CellStyle::new(DefaultStyle::bool(), &DefaultFormat::bool()));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::number(),
        &DefaultFormat::number(),
    ));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::percent(),
        &DefaultFormat::percent(),
    ));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::currency(),
        &DefaultFormat::currency(),
    ));
    book.add_cellstyle(CellStyle::new(DefaultStyle::date(), &DefaultFormat::date()));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::datetime(),
        &DefaultFormat::datetime(),
    ));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::time_of_day(),
        &DefaultFormat::time_of_day(),
    ));
    book.add_cellstyle(CellStyle::new(
        DefaultStyle::time_interval(),
        &DefaultFormat::time_interval(),
    ));

    book.add_def_style(ValueType::Boolean, DefaultStyle::bool());
    book.add_def_style(ValueType::Number, DefaultStyle::number());
    book.add_def_style(ValueType::Percentage, DefaultStyle::percent());
    book.add_def_style(ValueType::Currency, DefaultStyle::currency());
    book.add_def_style(ValueType::DateTime, DefaultStyle::date());
    book.add_def_style(ValueType::TimeDuration, DefaultStyle::time_interval());
}
