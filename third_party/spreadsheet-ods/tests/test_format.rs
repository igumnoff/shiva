mod lib_test;

use chrono::NaiveDateTime;
use icu_locid::locale;
use lib_test::*;
use spreadsheet_ods::format::{FormatCalendarStyle, FormatNumberStyle};
use spreadsheet_ods::style::CellStyle;
use spreadsheet_ods::{
    OdsError, Sheet, ValueFormatBoolean, ValueFormatCurrency, ValueFormatDateTime,
    ValueFormatNumber, ValueFormatPercentage, WorkBook,
};

#[test]
fn write_format() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut v1 = ValueFormatNumber::new_named("f1");
    v1.part_scientific().decimal_places(4).build();
    let v1 = wb.add_number_format(v1);

    let mut v2 = ValueFormatNumber::new_named("f2");
    v2.part_number().fixed_decimal_places(2).build();
    let v2 = wb.add_number_format(v2);

    let mut v3 = ValueFormatNumber::new_named("f3");
    v3.part_number().decimal_places(2).build();
    let v3 = wb.add_number_format(v3);

    let mut v31 = ValueFormatNumber::new_named("f31");
    v31.part_fraction()
        .denominator(13)
        .min_denominator_digits(1)
        .min_integer_digits(1)
        .min_numerator_digits(1)
        .build();
    let v31 = wb.add_number_format(v31);

    let mut v4 = ValueFormatCurrency::new_named("f4");
    v4.part_currency()
        .locale(locale!("de_AT"))
        .symbol("€")
        .build();
    v4.part_text(" ").build();
    v4.part_number().decimal_places(2).build();
    let v4 = wb.add_currency_format(v4);

    let mut v5 = ValueFormatPercentage::new_named("f5");
    v5.part_number().decimal_places(2).build();
    v5.part_text("/ct").build();
    let v5 = wb.add_percentage_format(v5);

    let mut v6 = ValueFormatBoolean::new_named("f6");
    v6.part_boolean().build();
    let v6 = wb.add_boolean_format(v6);

    let mut v7 = ValueFormatDateTime::new_named("f7");
    v7.part_era()
        .style(FormatNumberStyle::Long)
        .calendar(FormatCalendarStyle::Gregorian)
        .build();
    v7.part_text(" ").build();
    v7.part_year().style(FormatNumberStyle::Long).build();
    v7.part_text(" ").build();
    v7.part_month().style(FormatNumberStyle::Long).build();
    v7.part_text(" ").build();
    v7.part_day().style(FormatNumberStyle::Long).build();
    v7.part_text(" ").build();
    v7.part_day_of_week()
        .style(FormatNumberStyle::Long)
        .calendar(FormatCalendarStyle::Gregorian)
        .build();
    v7.part_text(" ").build();
    v7.part_week_of_year()
        .calendar(FormatCalendarStyle::Gregorian)
        .build();
    v7.part_text(" ").build();
    v7.part_quarter()
        .style(FormatNumberStyle::Long)
        .calendar(FormatCalendarStyle::Gregorian)
        .build();
    let v7 = wb.add_datetime_format(v7);

    let mut v8 = ValueFormatNumber::new_named("v8");
    v8.part_number()
        .min_integer_digits(4)
        .decimal_places(2)
        .embedded_text("RR", 2)
        .build();
    let v8 = wb.add_number_format(v8);

    let f1 = wb.add_cellstyle(CellStyle::new("f1", &v1));
    let f2 = wb.add_cellstyle(CellStyle::new("f2", &v2));
    let f3 = wb.add_cellstyle(CellStyle::new("f3", &v3));
    let f31 = wb.add_cellstyle(CellStyle::new("f31", &v31));
    let f4 = wb.add_cellstyle(CellStyle::new("f4", &v4));
    let f5 = wb.add_cellstyle(CellStyle::new("f5", &v5));
    let f6 = wb.add_cellstyle(CellStyle::new("f6", &v6));
    let f7 = wb.add_cellstyle(CellStyle::new("f7", &v7));
    let f8 = wb.add_cellstyle(CellStyle::new("f8", &v8));

    let mut sh = Sheet::new("1");
    sh.set_styled_value(0, 0, 1.234567f64, &f1);
    sh.set_styled_value(1, 0, 1.234567f64, &f2);
    sh.set_styled_value(2, 0, 1.234567f64, &f3);
    sh.set_styled_value(2, 1, 1.234567f64, &f31);
    sh.set_styled_value(3, 0, 1.234567f64, &f4);
    sh.set_styled_value(4, 0, 1.234567f64, &f5);
    sh.set_styled_value(6, 0, 1.234567f64, &f6);
    sh.set_styled_value(
        7,
        0,
        NaiveDateTime::from_timestamp_opt(1_223_222_222, 22992).unwrap(),
        &f7,
    );
    sh.set_styled_value(8, 0, 1.234567f64, &f8);

    wb.push_sheet(sh);
    let path = std::path::Path::new("test_out/test_format.ods");
    if path.exists() {
        test_write_ods(&mut wb, path)
    } else {
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::File::create(path)?;
        test_write_ods(&mut wb, path)
    }
}
