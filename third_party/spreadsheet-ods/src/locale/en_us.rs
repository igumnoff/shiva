use crate::defaultstyles::DefaultFormat;
use crate::format::FormatNumberStyle;
use crate::locale::LocalizedValueFormat;
use crate::{
    ValueFormatBoolean, ValueFormatCurrency, ValueFormatDateTime, ValueFormatNumber,
    ValueFormatPercentage, ValueFormatTimeDuration,
};
use icu_locid::{locale, Locale};

pub(crate) struct LocaleEnUs {}

pub(crate) static LOCALE_EN_US: LocaleEnUs = LocaleEnUs {};

impl LocaleEnUs {
    const LOCALE: Locale = locale!("en_US");
}

impl LocalizedValueFormat for LocaleEnUs {
    fn locale(&self) -> Locale {
        LocaleEnUs::LOCALE
    }

    fn boolean_format(&self) -> ValueFormatBoolean {
        let mut v = ValueFormatBoolean::new_localized(DefaultFormat::bool(), Self::LOCALE);
        v.part_boolean().build();
        v
    }

    fn number_format(&self) -> ValueFormatNumber {
        let mut v = ValueFormatNumber::new_localized(DefaultFormat::number(), Self::LOCALE);
        v.part_number()
            .min_integer_digits(1)
            .decimal_places(2)
            .build();
        v
    }

    fn percentage_format(&self) -> ValueFormatPercentage {
        let mut v = ValueFormatPercentage::new_localized(DefaultFormat::percent(), Self::LOCALE);
        v.part_number()
            .min_integer_digits(1)
            .decimal_places(2)
            .build();
        v.part_text("%").build();
        v
    }

    fn currency_format(&self) -> ValueFormatCurrency {
        let mut v = ValueFormatCurrency::new_localized(DefaultFormat::currency(), Self::LOCALE);
        v.part_currency().locale(Self::LOCALE).symbol("$").build();
        v.part_text(" ").build();
        v.part_number()
            .min_integer_digits(1)
            .decimal_places(2)
            .min_decimal_places(2)
            .grouping()
            .build();
        v
    }

    fn date_format(&self) -> ValueFormatDateTime {
        let mut v = ValueFormatDateTime::new_localized(DefaultFormat::date(), Self::LOCALE);
        v.part_month().style(FormatNumberStyle::Long).build();
        v.part_text("/").build();
        v.part_day().style(FormatNumberStyle::Long).build();
        v.part_text("/").build();
        v.part_year().style(FormatNumberStyle::Long).build();
        v
    }

    fn datetime_format(&self) -> ValueFormatDateTime {
        let mut v = ValueFormatDateTime::new_localized(DefaultFormat::datetime(), Self::LOCALE);
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

    fn time_of_day_format(&self) -> ValueFormatDateTime {
        let mut v = ValueFormatDateTime::new_localized(DefaultFormat::time_of_day(), Self::LOCALE);
        v.part_hours().style(FormatNumberStyle::Long).build();
        v.part_text(":").build();
        v.part_minutes().style(FormatNumberStyle::Long).build();
        v.part_text(":").build();
        v.part_seconds().style(FormatNumberStyle::Long).build();
        v.part_text(" ").build();
        v.part_am_pm().build();
        v
    }

    fn time_interval_format(&self) -> ValueFormatTimeDuration {
        let mut v =
            ValueFormatTimeDuration::new_localized(DefaultFormat::time_interval(), Self::LOCALE);
        v.set_truncate_on_overflow(false);

        v.part_hours().style(FormatNumberStyle::Long).build();
        v.part_text(":").build();
        v.part_minutes().style(FormatNumberStyle::Long).build();
        v.part_text(":").build();
        v.part_seconds().style(FormatNumberStyle::Long).build();
        v
    }
}
