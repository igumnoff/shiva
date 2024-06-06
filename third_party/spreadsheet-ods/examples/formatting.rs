//!
//!
//!

use std::fs;

use icu_locid::locale;
use spreadsheet_ods::format::create_number_format_fixed;
use spreadsheet_ods::{
    defaultstyles::{DefaultFormat, DefaultStyle},
    style::{units::TextAlign, StyleOrigin, StyleUse},
    write_ods, CellStyle, OdsResult, Sheet, ValueFormatNumber, WorkBook,
};

///
pub fn main() -> OdsResult<()> {
    let _ = fs::create_dir_all("examples_out");

    default_format()?;
    cell_style()?;
    number_format()?;

    Ok(())
}

/// Use predefined formats.
fn default_format() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    let mut sheet = Sheet::new("one");

    // use predefined styles.
    // see: WorkBook::locale_settings() for details.
    sheet.set_styled_value(0, 0, 1234, &DefaultStyle::number());
    sheet.set_styled_value(1, 0, 1234, &DefaultStyle::bool());
    sheet.set_styled_value(2, 0, 1234, &DefaultStyle::date());
    sheet.set_styled_value(3, 0, 1234, &DefaultStyle::time_of_day());
    sheet.set_styled_value(4, 0, 1234, &DefaultStyle::currency());

    wb.push_sheet(sheet);

    write_ods(&mut wb, "examples_out/default_format.ods")
}

/// Apply a cell-style.
fn cell_style() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    let mut style_bold = CellStyle::new("s_bold", &DefaultFormat::number());
    // origin + styleuse to show the style in the style-chooser.
    style_bold.set_origin(StyleOrigin::Styles);
    style_bold.set_styleuse(StyleUse::Named);
    // some attributes ...
    style_bold.set_font_bold();
    style_bold.set_text_align(TextAlign::Right);
    let style_bold = wb.add_cellstyle(style_bold);

    let mut sheet = Sheet::new("one");

    sheet.set_styled_value(0, 0, 1234, &style_bold);

    wb.push_sheet(sheet);

    write_ods(&mut wb, "examples_out/cell_style.ods")
}

/// Use number formatting.
fn number_format() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    // use a helper function
    let f_0 = create_number_format_fixed("numeric_0", 0, false);
    let f_0 = wb.add_number_format(f_0);

    // define from scratch, use the part-builders.
    let mut f_2 = ValueFormatNumber::new_named("numeric_2");
    f_2.part_text("!!").build();
    f_2.part_number()
        .fixed_decimal_places(2)
        .if_then(true, |f| f.grouping())
        .build();
    let f_2 = wb.add_number_format(f_2);

    // automatic name
    let mut f_3 = ValueFormatNumber::new_empty();
    f_3.part_fill_character().fill_char('*').build();
    f_3.part_number()
        .min_integer_digits(1)
        .decimal_places(3)
        .build();
    let f_3 = wb.add_number_format(f_3);

    // create cellstyles to use the formats

    let s_0 = CellStyle::new("numeric_0", &f_0);
    let s_0 = wb.add_cellstyle(s_0);

    let s_3 = CellStyle::new("", &f_3);
    let s_3 = wb.add_cellstyle(s_3);

    let s_2 = CellStyle::new("numeric_2", &f_2);
    let s_2 = wb.add_cellstyle(s_2);

    let mut sheet = Sheet::new("one");
    sheet.set_styled_value(0, 0, 12.34, &s_0);
    sheet.set_styled_value(1, 0, 12.3456, &s_3);
    sheet.set_styled_value(2, 0, 12.3456, &s_2);

    wb.push_sheet(sheet);

    write_ods(&mut wb, "examples_out/number_format.ods")
}
