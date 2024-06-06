Implements reading and writing of ODS Files.

```rust
use chrono::NaiveDate;
use icu_locid::locale;
use spreadsheet_ods::color::Rgb;
use spreadsheet_ods::formula;
use spreadsheet_ods::mm;
use spreadsheet_ods::style::units::{Border, Length, TextRelief};
use spreadsheet_ods::style::CellStyle;
use spreadsheet_ods::{format, OdsResult};
use spreadsheet_ods::{Sheet, Value, WorkBook};
use std::fs;

  fs::create_dir_all("test_out").expect("create_dir");

  let path = std::path::Path::new("test_out/lib_example.ods");
  let mut wb = if path.exists() {
    spreadsheet_ods::read_ods(path).unwrap()
  } else {
    WorkBook::new(locale!("en_US"))
  };

  if wb.num_sheets() == 0 {
    let mut sheet = Sheet::new("one");
    sheet.set_value(0, 0, true);
    wb.push_sheet(sheet);
  }

  let sheet = wb.sheet(0);
  let _n = sheet.value(0, 0).as_f64_or(0f64);
  if let Value::Boolean(v) = sheet.value(1, 1) {
    if *v {
      println!("was true");
    }
  }

  if wb.num_sheets() == 1 {
    wb.push_sheet(Sheet::new("two"));
  }

  let date_format = format::create_date_dmy_format("date_format");
  let date_format = wb.add_datetime_format(date_format);

  let mut date_style = CellStyle::new("nice_date_style", &date_format);
  date_style.set_font_bold();
  date_style.set_font_relief(TextRelief::Engraved);
  date_style.set_border(mm!(0.2), Border::Dashed, Rgb::new(192, 72, 72));
  let date_style_ref = wb.add_cellstyle(date_style);

  let sheet = wb.sheet_mut(1);
  sheet.set_value(0, 0, 21.4f32);
  sheet.set_value(0, 1, "foo");
  sheet.set_styled_value(0, 2, NaiveDate::from_ymd_opt(2020, 03, 01), &date_style_ref);
  sheet.set_formula(0, 3, format!("of:={}+1", formula::fcellref(0, 0)));

  let mut sheet = Sheet::new("sample");
  sheet.set_value(5, 5, "sample");
  wb.push_sheet(sheet);

  spreadsheet_ods::write_ods(&mut wb, "test_out/lib_example.ods").expect("write_ods")

```

This does not cover the entire ODS spec.

What is supported:

* Spread-sheets
    * Handles all datatypes
        * Uses time::Duration
        * Uses chrono::NaiveDate and NaiveDateTime
    * Column/Row/Cell styles
    * Formulas
        * Only as strings, but support functions for cell/range references.
    * Row/Column spans
    * Header rows/columns, print ranges
    * Formatted text as xml text.

* Formulas
    * Only as strings.
    * Utilities for cell/range references.

* Styles
    * Default styles per data type.
    * Preserves all style attributes.
    * Table, row, column, cell, paragraph and text styles.
    * Stylemaps (basic support)
    * Support for *setting* most style attributes.

* Value formatting
    * The whole set is available.
    * Utility functions for common formats.
    * Basic localization support.

* Content validation

* Fonts
    * Preserves all font attributes.
    * Basic support for setting this stuff.

* Page layouts
    * Style attributes
    * Header/footer content as XML text.

* Cell/range references
    * Parsing and formatting

What might be problematic:

* The text content of each cell is not formatted according to the given
  ValueFormat,
  but instead is a simple to_string() of the data type. This data is not
  necessary
  to read the contents correctly. LibreOffice seems to ignore this completely
  and display everything correctly.

Next on the TO-DO list:

* Calculation settings.
* Named expressions.

There are a number of features that are not parsed to a structure,
but which are stored as a XML. This might work as long as
these features don't refer to data that is no longer valid after
some modification. But they are written back to the ods.

Anyway those are:

* tracked-changes
* variable-decls
* sequence-decls
* user-field-decls
* dde-connection-decls
* calculation-settings
* label-ranges
* named-expressions
* database-ranges
* data-pilot-tables
* consolidation
* dde-links
* table:desc
* table-source
* dde-source
* scenario
* forms
* shapes
* calcext:conditional-formats

When storing a previously read ODS file, all the contained files
are copied to the new file. The files content.xml, styles.xml,
settings.xml and manifest.xml are written from the data.

