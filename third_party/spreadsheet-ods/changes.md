# 0.22.2

- Update dependencies
    - zip 0.6.6 -> 1.3
    - base64 0.21 -> 0.22

# 0.22.1

- fix

# 0.22.0

- memory optimizations
    - BREAKING: Change &String in all APIs with &str.
    - BREAKING: change currency string in Value from String to Box<str>. This shrinks Currency
      enough to fit in with the other variants.
    - BREAKING: change annotation to Box<Annotation>. Big reduction for CellDataExt. Overall minor wins.
    - change AttrMap2 from HashMap to Vec<Key> + Vec<Value>. Time is roughly equivalent, memory -7% averaged.
    - breaking: change Sheet::header_rows() and Sheet::header_cols() to Header instead of RowRange/ColRange.

# 0.21.0

- add some examples.

## Breaking changes

- small: ```Pagestyle.set_page_usage()``` changed.
  There has been an unusual Option<> parameter. This is replaced with a direct set+clear.
- medium: switch apis for xxxRef fields from String to the correct Ref type.
  The conversions from/to strings are still in place.
- medium: rename the metadata xml-tags to have a common prefix "Meta". This helps with
  code completion, there were conflicts with some enums.
- large: Refined OdsOptions.
    - remove ```use_repeat_for_empty()``` - this is almost unusable as it's
      very difficult to avoid overlapping cells. Especially with empty cells you didn't
      know where there now overlapping a newly added data-cell.
      ```ignore_empty_cells()``` might be useful instead.
    - rename ```use_clone_for_repeat()``` to ```use_clone_for_cells()```
    - add ```ignore_empty_cells()``` and ```read_empty_cells()```
    - add ```read_styles()``` as opposite to ```content_only()```
- small: rename misleading ```used_cols()```/```used_rows()``` to
  ```col_header_max()```/```row_header_max()```.
  To get the fill-state of the sheet there has always been used_grid_size().

## Features

- add ```set_styled()``` as short name for ```set_styled_value()```. Ease of use addition.
- add ```CellContentRef::to_owned()``` to get a CellContent instance.
- add ```iter_cols()``` and ```iter_rows()``` to Sheet. Implements column-wise/row-wise iteration
  over a given Range.
- new functions ```add_left()```, ```add_center()```, ```add_right()```, ```add_content()``` for HeaderFooter.
  Ease of use addition.
- add ```OdsResult<T>```
- add ```get-size```

## Performance/memory usage

- Store the row/column header data in compact form pos+span to save some memory.
    - column headers are deduplicated when reading/before writing on top of that.
- The cell duplication code has been moved to a separate stage.
  This makes the task a lot easier and allows extra reductions.
- The repeat value for a whole row is now used to duplicated the rows on reading.
  This has not been done so far.
- The last two rows often use insane repeat values. If the repeat-value for these rows is greater 1000
  it's simply ignored.
- The last column of a row often only has a style and a big repeat value. These usually are only
  editing artifacts and not consciously added. These columns are now ignored and dropped.
  If a style for such columns is needed, setting a default-cell-style for the column should handle
  this case.
- These changes have reduced memory usage for my test-set by a factor of 28.

## Bugfix

- fix: ValueFormatXXX now use the correct default values for StyleOrigin (Content) and
  StyleUse (Automatic).
- fix: create_number_format_fixed: must set integer-digits too, otherwise this doesn't work.
- fix: If "mimetype" or "META-INF/manifest.xml" occurs in the manifest they were duplicated
  in the output. which creates an invalid zip archive.

## Experiments

Tried three different implementations for xxxStyleRef.

- Current String implementation.
- Using smol_str:
    - Memory usage goes down only 1-2%. Time is in the same range. Not worth the effort.
- Using handles with u32:
    - Memory usage goes down 10%. Time is in the same range.
    - This can't get rid of the style-name completely, so there would be two ways of
      indirection. Which only leads to insanity.

Conclusion 1: It's not worth the effort.
Conclusion 2: Using xxStyleRef instead of String helps readability. Implementing
Borrow<str> and AsRef<str> for xxStyleRef might be helpful.

# 0.20.2

fix #47: method unescape_value for struct quick_xml::events::attributes::Attribute is only
available if "encoding" feature is disabled

# 0.20.1

- Change OdsOptions.use_repeat_for_empty to default to false. The result of the
  former default of true was too surprising when editing spreadsheets.
- But add the behaviour to read to ignore any cell-data that contains no data
  except a repeat count. This accounts for most of the uses the flag was indented
  for without the repercussions.

- Make CRow and CCol public and add the necessary functionality.

# 0.20.0

- breaking: Split lib.rs in workbook.rs, sheet.rs, cell.rs and value.rs
    * mod error removed, only crate::ODSError is reachable.
    * new mod workbook: EventListener, Script and WorkBookConfig moved here.
    * new mod sheet: CellIter, Grouped, Range, SheetConfig, SplitMode and
      Visibility moved here.
    * new mod cell: CellSpan moved here.

- fix: 0.18.0 removed header_rows and header_cols.
  Mistook these with some useless feature. Reinstated now.

- cleanup: Remove mktemp. Not used anymore.
- cleanup: Remove indexmap. Only used for a specific task, generally use std::
  HashMap.

# 0.19.3

one more dbg!()

# 0.19.2

dbg!() removed.

# 0.19.1

Move cell! macro from spreadsheet-ods-formula crate to this one.

# 0.19.0

BREAKING

- The default for reading is changed to use the repeat counter for empty cells.
  While this should be fine for most uses, it still might break something.
  But the speedup for many cases is too impressive to not change this.
    - This can be reverted to the previous default with OdsOptions::
      use_clone_for_repeat().

NEW

- Add OdsOptions for read options.
    - content_only() - Parses only the data content, no styles etc.
      WorkBooks that are read this way should rather be treated as read-only,
      writing them back will loose all the meta-data, styles, ..
    - use_repeat_for_cells() - Sets the repeat-counter for a cell instead
      of cloning the cell.
    - use_repeat_for_empty() - Sets the repeat-counter for empty cells, but
      not for cells containing data or formulas.
    - use_clone_for_repeat() - Clone always.
      Rows are never spread out like this, they always use the repeat count.
- Cell annotations.
- Matrix span for cells.
- Read images directly linked to a table cell. (There are images that are
  tied to the sheet. Not covered yet.)

CHANGED

- Split the attribute macros along the xml prefix into separate files.
- Internal CellData struct looses some data and gains a boxed extra field.
  Which accommodates seldom used data. CellContent and CellContentRef are
  simply extended with the new fields.
- Cleanup the code for reading table-cells.
- GraphicStyle now keeps paragraph and text attributes.
- WorkBook::default() now uses a default locale to init basic cell styles.

# 0.18.1

Published with xml-tag checks active. Corrected now.

# 0.18.0

NEW

- Read/Write Flat-ODS (.fods) files.
    - read/write functions duplicated for fods-files.
- Add support for row and column groupings.
- Add basic support for scripts and event-listeners.
- Namespaces are now copied too.

CHANGED

- Removed support for header-rows/header-columns. This is only used for Writer
  not for Spreadsheet.
- Datetime values can have a trailing "Z".
- Basic support for ruby-styles.
- Add missing iterators for WorkBook content.
- Currency values without currency symbol don't produce an error.
- Empty config-items are ignored.
- Cleanup of read/write functionality. Mostly to get a more unified style.

# 0.17.0

NEW

- Allow access to meta.xml data.
- Allow access to manifest.xml.

CHANGED

- Rewrote the XMLWriter to cause less allocations. Mixed results, but nicer API.

# 0.16.1

- Add PartialEq for Value and dependencies.
- Add WorkBook::iter_sheets(), iter_row_styles(), iter_col_styles(),
  iter_cell_styles().
- Bump dependencies.
- Reexport color-rs crate as spreadsheet_ods::color. It seems this is often with
  the defunct "color" crate.
- Fixed a compile-error PR#44

# 0.16.0

- New ValueStyleMap for use in ValueFormat*.
- base_cell is optional even for CellStyle stylemaps.
- ValueCondition has to use 'value()'

- read_ods_from() and write_ods_to() for Read/Write traits.

# 0.15.0

- It was an error to assume that currency values use an ISO code for
  the currency string. Removed the optimization and use a String again.

- number-rows-repeated a million times. Can be found for the last or the
  second to last row. If the row is overwritten with actual data and
  opened in LibreOffice this results to a real memory stress test.
  Any repeat count of more than 1000 for the last two rows are now ignored.

- Sheet::split_col_header() and split_row_header() now split after
  the given row/column.
- Add as_*64, as_*16, as_*8 conversions for Value.

- Bug: Default number-format should set min-integer-digits to 1. Fixed.
- Bug: LibreOffice uses dates like 0000-00-00. Fixed.
- Bug: embedded-text in format broke the parser. Removed that part for now
  and ignore this tag.
- Bug: Parsing sheet-names failed with the new reference parser. Fixed.

- Update dependencies

# 0.14.0

- Undo spreadsheet-ods-cellref. Was a reasonable start, but didn't work out
  as expected.
- Instead use a splinter of a parser for OpenFormula I'm working on separately
  for cellref parsing.
- This means
    - Cell-references now can contain external references via an IRI.
    - Cell-ranges can span more than one table.
    - Colranges and Rowranges have IRI, from-table and to-table now too.

# 0.13.0

- Upgrade mktemp to latest.
- Extracted cell references to a separate crate spreadsheet-ods-cellref.
    - The parser has been rewritten with nom.
    - The fmt* functions are new too.
- CellRef
    - Add an IRI for references to external files.
- CellRange
    - Add an IRI for references to external files.
    - Add a to_table to allow ranges that span multiple sheets.
- ColRange, RowRange
    - Add an IRI for references to external files.
    - Add from_table and to_table.
    - Add from_col_abs, to_col_abs for fixed columns in ColRange.
    - Add from_row_abs, to_row_abs for fixed rows in RowRange.

# 0.12.1

- Upgrade icu_locid and quick_xml to latest.

# 0.12.0

BREAKING:

- ValueFormat is gone. Many, many functions had an annotation
  "can only be used when ...", which is not a good sign.
  So I split it up in one struct per ValueType (ValueFormatBoolean,
  ValueFormatNumber, ...). This allows for a clearer communication
  what is possible with each of them.

  Changing should be straightforward:

  Before:
  ```rust
    let mut v1 = ValueFormat::new_named("f1", ValueType::Number);
    v1.part_scientific().decimal_places(4).build();
    let v1 = wb.add_format(v1);
  ```

  After:
  ```rust
    let mut v1 = ValueFormatNumber::new_named("f1");
    v1.part_scientific().decimal_places(4).build();
    let v1 = wb.add_number_format(v1);
  ```

  The good news: I think I am happy now how ValueFormatXXX and XXXStyle work.
  I will keep them stable from now on.

CHANGES:

- create_loc_number_format_fixed, create_loc_time_interval_format where missing.
- HeaderFooter can contain multiple paragraphs of text. Works now.
- TextTag/XmlTag: Add functionality to work with Vec<XmlTag>.

# 0.11.1

- Minor fixes.

# 0.11.0

BREAKING:

Localization has been added via icu_locid. This leads to a few but central
breaks in the api.

- WorkBook::new() now needs a Locale. This obsoletes the call to
  create_default_styles()
  which never was really satisfying. The old behaviour can be had with
  WorkBook::new_empty()

- ValueFormat: set_country(), set_language(), set_script() were replaced with
  set_locale().
- ValueFormat: all the format_xxx() functions were a train-wreck and have been
  removed. They were only ever used to write the cell-content in a nicer way. A
  value
  that is immediately thrown away when the spreadsheet is openend. So I now
  write the
  same format that is used for the xxx-value attribute anyway.
- FormatPart: all new_xxx functions removed.

CHANGES:

- Overhauled ValueFormat.
    - All the ValueFormat::push_xxx were broken and missing attributes.
      As most of these attributes are optional these functions were replaced
      with new ValueFormat::part_xxx which return a builder for each pattern.

- Add icu_locid to the dependencies. Used where language/country/script
  attributes exist.
- Add locale module that contains localized default formats.
    - Available locales are behind feature-gates.
    - Needs ca 60 loc for a new locale.
    - Fallback available.
    - create_default_styles replaced with WorkBook::init_defaults and WorkBook::
      new_localized.

- Sheet::new() now always needs a name for the sheet.

- All the style attributes are crosschecked with the specification, and a lot
  of missing ones where added. I only excluded obviously obsolete ones and
  things that are out of scope.

- TableStyle::set_master_page_name() -> set_master_page()
- FontFaceDecl::new_with_name()

# 0.10.0

- Upgraded to edition 2021.
- Updated dependencies:
    - rust_decimal to 1.24
    - color_rs to 0.7
    - time to 0.3
    - zip to 0.6
    - removed criterion as dev-dependency.
- Parsing values implemented with nom and changed from str to &[u8] to safe on
  unnecessary utf8 conversion.
- Needed a lot of read buffers for each xml hierarchy level. Keep them around
  and reuse them.
- set_row_repeat must not be 0. Panics if so. This doesn't solve all
  problems with set_row_repeat, there is still some spurious repeat on the
  last row.
- Content validation was broken.

# 0.9.0

- Throw away SCell. This was used for internal storage and as part of the API.
  Split this into the internal CellData and the API CellContent for a copy
  of the cell data and CellContentRef for references to the data.
  This allows for a possible future rearrangement of the internal storage.

  cell_mut was removed, cell, add_cell, remove_cell, work with CellContent now.
  iter() and range() use CellContentRef.

- Throw away ucell. Uses u32 instead.

- Implement IntoIterator, iter(), range()

- Add CellSpan for ease of use.

- Changed layout of Value::Currency. The currency string is 3 bytes of ASCII,
  so a String is not necessary.

- read_table_cell and read_text_or_tag rewritten to use fewer copies of String
  data. Parsing cell-values works directly with the buffer data.

# 0.8.2

- Checks that formulas start with "of:="
- New f*ref variants for formulas. These create a diverse array of absolute $
  references.
- Value can extract a NaiveDate value.

WorkBook:

- Add sheet_idx to find a sheet by name.
- Add used_cols, used_rows to find the number of row/column-headers.
- fixed a missing namespace.

Sheet:

- clear_formula, clear_cell_style, clear_validation

# 0.8.1

- fix for #24. Excel doesn't like the empty content-validations tag.
- complex text was missing the value-type
- Value conversions from ref types.

# 0.8.0

- Value::TextXml changed to Vec<TextTag>. Multiline, styled text can occur as
  multiple direct text:p in a table:cell.

- Repeat rows where not correctly considered when occuring in the middle of the
  table.

  There is a problem that still remains: If data exists in the repeat range,
  these values are written out too. The effect is that the rest of the rows is
  shifted down. This might destroy cell references. This doesn't happen when an
  existing ODS is read, but care has to be taken when generating new stuff.

- write_ods_buf and read_ods_buf implemented.

- add some missing namespaces.
- fix bug in Condition with string values.
- add range() to sheet.

# 0.7.0

- Add content validations.
    - Condition and ValueCondition allow for composition.
    - Validation supports all features except macros.
    - StyleMap uses ValueCondition now.
    - Validation can be set on SCell.
- CellRange, CellRef get absolute_xx() functions.

# 0.6.3

- Renamed the split functions to better match their functionality.
- Clippy brought up some issues.
- Moved Detached to the ds module. It's not a core concept.

# 0.6.2

- Rewriting an existing ODS was broken after the change to directly writing the
  ZIP.

# 0.6.1

- F'd up some tests with 0.6.0, and the new configuration features broke a lot.
  Fixed now.
- Reworked Config
    - Now the order of the configuration entries is kept between load/stores.
      Makes comparisons easier.
    - Added a few more configuration flags to be visible.
    - Added functions to sheet to set table-splits.

# 0.6.0

Breaking:

- Storing the WorkBook now needs a mutable reference.
- set_col_width doesn't need the WorkBook any longer.
- set_row_height doesn't need the WorkBook any longer.

Changes:

- Add basic row repeat functionality to Sheet. The subsequent row index will not
  be altered for now, and references etc must be updated manually. After writing
  and reading again the row indexes *will* be changed. So for now it's mostly
  useful to use this for the last row in the Sheet.
- The ODS content will no longer be written to a temp directory first and zipped
  later. Was a workaround a weird bug I couldn't locate. Testing now shows no
  problems.
- Styles without name get a name assigned when adding to the WorkBook.
- Length got an extra value Default.
- Sheet.set_col_width and Sheet.set_row_height now can work without the WorkBook
  Parameter. The necessary style modifications are applied when storing the
  workbook.
  *Storing the WorkBook now needs a mutable reference to make this possible.*
- Sheets can now be detached from the workbook and leave a placeholder behind to
  be reattached later. This makes it easier to modify the WorkBook and a Sheet
  at the same time.
- AttrMap2Trait removed, not helpful after the style reorg.
- settings.xml is now parsed. A subset of the Settings can be accessed via
  WorkBook.config() and Sheet.config().

# 0.5.2

- Add useability for XmlTag and TextTag.
- Implement a few standard TextTag Wrappers for common text elements.

# 0.5.1

- Split up the unwieldy PageLayout into PageStyle and MasterPage, and add an
  example how to use those bastards.

# 0.5.0

- Major reorg of styles. Replaced Styles with separate CellStyle, ColStyle,
  RowStyle etc.
- Create CellStyleRef, ColStyleRef, etc to be used when relating to styles.
  Should add a little bit of safety here.
- Introduced submodules for style: stylemap, tabstop, units

# 0.4.2

- Allow the ODS version to be specified. This adds support for ODS 1.3. --
  Default version set to 1.3.

# 0.4.1

- Refine usage of Style::cell(), cell_mut(), table(), table_mut(), col(),
  col_mut(),
  row(), row_mut(). Assert the correct style-family for access to these
  Attributes.

- Bug when writing empty lines, used wrong row-style.

- No row/column styles are written if they are beyond the range of the maximum
  used cell. This is a desired behaviour. To make it easier there is a
  Value-Conversion from '()' to an empty cell-value.

