use crate::format::FormatNumberStyle;
use crate::{
    ValueFormatBoolean, ValueFormatCurrency, ValueFormatDateTime, ValueFormatNumber,
    ValueFormatPercentage, ValueFormatTimeDuration,
};
use icu_locid::Locale;

/// Creates a new number format.
pub fn create_loc_boolean_format<S: AsRef<str>>(name: S, locale: Locale) -> ValueFormatBoolean {
    let mut v = ValueFormatBoolean::new_localized(name, locale);
    v.part_boolean().build();
    v
}

/// Creates a new number format.
pub fn create_loc_number_format<S: AsRef<str>>(
    name: S,
    locale: Locale,
    decimal: u8,
    grouping: bool,
) -> ValueFormatNumber {
    let mut v = ValueFormatNumber::new_localized(name, locale);
    v.part_number()
        .decimal_places(decimal)
        .if_then(grouping, |p| p.grouping())
        .build();
    v
}

/// Creates a new number format with a fixed number of decimal places.
pub fn create_loc_number_format_fixed<S: AsRef<str>>(
    name: S,
    locale: Locale,
    decimal: u8,
    grouping: bool,
) -> ValueFormatNumber {
    let mut v = ValueFormatNumber::new_localized(name, locale);
    v.part_number()
        .min_integer_digits(1)
        .fixed_decimal_places(decimal)
        .if_then(grouping, |p| p.grouping())
        .build();
    v
}

/// Creates a new percentage format.
pub fn create_loc_percentage_format<S: AsRef<str>>(
    name: S,
    locale: Locale,
    decimal: u8,
) -> ValueFormatPercentage {
    let mut v = ValueFormatPercentage::new_localized(name, locale);
    v.part_number().decimal_places(decimal).build();
    v.part_text("%").build();
    v
}

/// Creates a new currency format.
pub fn create_loc_currency_prefix<S1, S2>(
    name: S1,
    locale: Locale,
    symbol_locale: Locale,
    symbol: S2,
) -> ValueFormatCurrency
where
    S1: AsRef<str>,
    S2: Into<String>,
{
    let mut v = ValueFormatCurrency::new_localized(name, locale);
    v.part_currency()
        .locale(symbol_locale)
        .symbol(symbol)
        .build();
    v.part_text(" ").build();
    v.part_number()
        .decimal_places(2)
        .min_decimal_places(2)
        .grouping()
        .build();
    v.part_number()
        .decimal_places(2)
        .min_decimal_places(2)
        .grouping()
        .build();
    v
}

/// Creates a new currency format.
pub fn create_loc_currency_suffix<S1, S2>(
    name: S1,
    locale: Locale,
    symbol_locale: Locale,
    symbol: S2,
) -> ValueFormatCurrency
where
    S1: AsRef<str>,
    S2: Into<String>,
{
    let mut v = ValueFormatCurrency::new_localized(name, locale);
    v.part_number()
        .decimal_places(2)
        .min_decimal_places(2)
        .grouping()
        .build();
    v.part_text(" ").build();
    v.part_currency()
        .locale(symbol_locale)
        .symbol(symbol)
        .build();
    v
}

/// Creates a new date format D.M.Y
pub fn create_loc_date_dmy_format<S: AsRef<str>>(name: S, locale: Locale) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_localized(name, locale);
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_year().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new date format M/D/Y
pub fn create_loc_date_mdy_format<S: AsRef<str>>(name: S, locale: Locale) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_localized(name, locale);
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text("/").build();
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text("/").build();
    v.part_year().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a datetime format Y-M-D H:M:S
pub fn create_loc_datetime_format<S: AsRef<str>>(name: S, locale: Locale) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_localized(name, locale);
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_year().style(FormatNumberStyle::Long).build();
    v.part_text(" ").build();
    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new time-Duration format H:M:S
pub fn create_loc_time_format<S: AsRef<str>>(name: S, locale: Locale) -> ValueFormatTimeDuration {
    let mut v = ValueFormatTimeDuration::new_localized(name, locale);
    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new time-Duration format H:M:S
pub fn create_loc_time_interval_format<S: AsRef<str>>(
    name: S,
    locale: Locale,
) -> ValueFormatTimeDuration {
    let mut v = ValueFormatTimeDuration::new_localized(name, locale);
    v.set_truncate_on_overflow(false);

    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new number format.
pub fn create_boolean_format<S: AsRef<str>>(name: S) -> ValueFormatBoolean {
    let mut v = ValueFormatBoolean::new_named(name);
    v.part_boolean().build();
    v
}

/// Creates a new number format.
pub fn create_number_format<S: AsRef<str>>(
    name: S,
    decimal: u8,
    grouping: bool,
) -> ValueFormatNumber {
    let mut v = ValueFormatNumber::new_named(name);
    v.part_number()
        .decimal_places(decimal)
        .if_then(grouping, |p| p.grouping())
        .build();
    v
}

/// Creates a new number format with a fixed number of decimal places.
pub fn create_number_format_fixed<S: AsRef<str>>(
    name: S,
    decimal: u8,
    grouping: bool,
) -> ValueFormatNumber {
    let mut v = ValueFormatNumber::new_named(name);
    v.part_number()
        .min_integer_digits(1)
        .fixed_decimal_places(decimal)
        .if_then(grouping, |p| p.grouping())
        .build();
    v
}

/// Creates a new percentage format.
pub fn create_percentage_format<S: AsRef<str>>(name: S, decimal: u8) -> ValueFormatPercentage {
    let mut v = ValueFormatPercentage::new_named(name);
    v.part_number().fixed_decimal_places(decimal).build();
    v.part_text("%").build();
    v
}

/// Creates a new currency format.
pub fn create_currency_prefix<S1, S2>(
    name: S1,
    symbol_locale: Locale,
    symbol: S2,
) -> ValueFormatCurrency
where
    S1: AsRef<str>,
    S2: Into<String>,
{
    let mut v = ValueFormatCurrency::new_named(name);
    v.part_currency()
        .locale(symbol_locale)
        .symbol(symbol)
        .build();
    v.part_text(" ").build();
    v.part_number().fixed_decimal_places(2).grouping().build();
    v
}

/// Creates a new currency format.
pub fn create_currency_suffix<S1, S2>(
    name: S1,
    symbol_locale: Locale,
    symbol: S2,
) -> ValueFormatCurrency
where
    S1: AsRef<str>,
    S2: Into<String>,
{
    let mut v = ValueFormatCurrency::new_named(name);
    v.part_number().fixed_decimal_places(2).grouping().build();
    v.part_text(" ").build();
    v.part_currency()
        .locale(symbol_locale)
        .symbol(symbol)
        .build();
    v
}

/// Creates a new date format YYYY-MM-DD
pub fn create_date_iso_format<S: AsRef<str>>(name: S) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_named(name);
    v.part_year().style(FormatNumberStyle::Long).build();
    v.part_text("-").build();
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text("-").build();
    v.part_day().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new date format D.M.Y
pub fn create_date_dmy_format<S: AsRef<str>>(name: S) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_named(name);
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text(".").build();
    v.part_year().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new date format M/D/Y
pub fn create_date_mdy_format<S: AsRef<str>>(name: S) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_named(name);
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text("/").build();
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text("/").build();
    v.part_year().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a datetime format Y-M-D H:M:S
pub fn create_datetime_format<S: AsRef<str>>(name: S) -> ValueFormatDateTime {
    let mut v = ValueFormatDateTime::new_named(name);
    v.part_year().style(FormatNumberStyle::Long).build();
    v.part_text("-").build();
    v.part_month().style(FormatNumberStyle::Long).build();
    v.part_text("-").build();
    v.part_day().style(FormatNumberStyle::Long).build();
    v.part_text(" ").build();
    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new time format H:M:S
pub fn create_time_of_day_format<S: AsRef<str>>(name: S) -> ValueFormatTimeDuration {
    let mut v = ValueFormatTimeDuration::new_named(name);

    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}

/// Creates a new time-Duration format H:M:S
pub fn create_time_interval_format<S: AsRef<str>>(name: S) -> ValueFormatTimeDuration {
    let mut v = ValueFormatTimeDuration::new_named(name);
    v.set_truncate_on_overflow(false);

    v.part_hours().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_minutes().style(FormatNumberStyle::Long).build();
    v.part_text(":").build();
    v.part_seconds().style(FormatNumberStyle::Long).build();
    v
}
