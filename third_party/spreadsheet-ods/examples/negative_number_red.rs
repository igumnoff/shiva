//!
//!
//!

use color::Rgb;
use icu_locid::locale;
use spreadsheet_ods::condition::ValueCondition;
use spreadsheet_ods::format::{ValueFormatTrait, ValueStyleMap};
use spreadsheet_ods::*;
use std::fs;

///
pub fn main() -> Result<(), OdsError> {
    let _ = fs::create_dir_all("examples_out");

    negative_numbers_red()?;

    Ok(())
}

fn negative_numbers_red() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    // positive format
    let mut f_number = ValueFormatNumber::new_localized("num0", locale!("de_AT"));
    f_number
        .part_number()
        .decimal_places(2)
        .min_integer_digits(1)
        .grouping()
        .build();
    let f_number = wb.add_number_format(f_number);

    // negative format
    let mut f_number_red = ValueFormatNumber::new_localized("num0red", locale!("de_AT"));
    f_number_red.part_text("-").build();
    f_number_red
        .part_number()
        .decimal_places(2)
        .min_integer_digits(1)
        .grouping()
        .build();
    f_number_red.set_color(Rgb::new(255, 0, 0));
    f_number_red.push_stylemap(ValueStyleMap::new(ValueCondition::value_ge(0), f_number));
    let f_number_red = wb.add_number_format(f_number_red);

    // cellstyle for this number format
    let s_number = CellStyle::new("num_red", &f_number_red);
    let s_number = wb.add_cellstyle(s_number);

    // ...

    let mut sheet = Sheet::new("sample");

    sheet.set_styled_value(0, 0, 723, &s_number);
    sheet.set_styled_value(1, 0, -723, &s_number);

    wb.push_sheet(sheet);

    write_ods(&mut wb, "examples_out/neg.ods")?;

    Ok(())
}
