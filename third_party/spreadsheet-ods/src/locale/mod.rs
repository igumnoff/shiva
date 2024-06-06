//!
//! Defines localized versions for all default formats.
//!

mod default;

#[cfg(feature = "locale_de_AT")]
mod de_at;
#[cfg(feature = "locale_en_US")]
mod en_us;

use crate::HashMap;
use crate::{
    ValueFormatBoolean, ValueFormatCurrency, ValueFormatDateTime, ValueFormatNumber,
    ValueFormatPercentage, ValueFormatTimeDuration,
};
use icu_locid::Locale;
use lazy_static::lazy_static;

/// Defines functions that generate the standard formats for various
/// value types.
#[allow(dead_code)]
pub(crate) trait LocalizedValueFormat: Sync {
    fn locale(&self) -> Locale;
    /// Default boolean format.
    fn boolean_format(&self) -> ValueFormatBoolean;
    /// Default number format.
    fn number_format(&self) -> ValueFormatNumber;
    /// Default percentage format.
    fn percentage_format(&self) -> ValueFormatPercentage;
    /// Default currency format.
    fn currency_format(&self) -> ValueFormatCurrency;
    /// Default date format.
    fn date_format(&self) -> ValueFormatDateTime;
    /// Default date/time format.
    fn datetime_format(&self) -> ValueFormatDateTime;
    /// Default time of day format.
    fn time_of_day_format(&self) -> ValueFormatDateTime;
    /// Default time interval format.
    fn time_interval_format(&self) -> ValueFormatTimeDuration;
}

lazy_static! {
    static ref LOCALE_DATA: HashMap<Locale, &'static dyn LocalizedValueFormat> = {
        #[allow(unused_mut)]
        let mut lm: HashMap<Locale, &'static dyn LocalizedValueFormat> = HashMap::new();

        lm.insert(icu_locid::locale!("en"), &default::LOCALE_DEFAULT);
        #[cfg(feature = "locale_de_AT")]
        {
            lm.insert(icu_locid::locale!("de_AT"), &de_at::LOCALE_DE_AT);
        }
        #[cfg(feature = "locale_en_US")]
        {
            lm.insert(icu_locid::locale!("en_US"), &en_us::LOCALE_EN_US);
        }
        lm
    };
}

/// Returns the localized format or a fallback.
pub(crate) fn localized_format(locale: Locale) -> Option<&'static dyn LocalizedValueFormat> {
    LOCALE_DATA.get(&locale).copied()
}
