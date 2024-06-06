mod lib_test;

use lib_test::*;
use spreadsheet_ods::condition::Condition;
use spreadsheet_ods::text::TextP;
use spreadsheet_ods::validation::{Validation, ValidationError, ValidationHelp};
use spreadsheet_ods::{CellRange, OdsError, Sheet, WorkBook};

#[test]
fn test_validation0() -> Result<(), OdsError> {
    let mut book = WorkBook::new_empty();

    let mut sheet = Sheet::new("One");

    let cc: u32 = 0;
    sheet.set_value(0, cc, "Content Length");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_text_length_lt(5));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 1;
    sheet.set_value(0, cc, "Between");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_text_length_is_between(4, 10));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 2;
    sheet.set_value(0, cc, "Not Between");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_text_length_is_not_between(4, 10));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 3;
    sheet.set_value(0, cc, "In List");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_in_list::<u32>(&[
        1, 3, 5, 7, 11, 13, 17, 19,
    ]));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 4;
    sheet.set_value(0, cc, "Decimal");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_decimal_number_and(
        Condition::content_gt(0),
    ));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 5;
    sheet.set_value(0, cc, "Whole");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_whole_number_and(
        Condition::content_eq(0),
    ));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 6;
    sheet.set_value(0, cc, "Date");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_date_and(Condition::content_eq(0)));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 7;
    sheet.set_value(0, cc, "Time");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_time_and(Condition::content_eq(0)));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 8;
    sheet.set_value(0, cc, "Ref");
    sheet.set_value(1, cc + 1, "A");
    sheet.set_value(2, cc + 1, "B");
    sheet.set_value(3, cc + 1, "C");
    sheet.set_value(4, cc + 1, "D");
    sheet.set_value(5, cc + 1, "E");
    sheet.set_value(6, cc + 1, "F");
    sheet.set_value(7, cc + 1, "G");
    sheet.set_value(8, cc + 1, "H");
    sheet.set_value(9, cc + 1, "I");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_in_cellrange(
        CellRange::local(1, cc + 1, 9, cc + 1).absolute(),
    ));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    let cc: u32 = 10;
    sheet.set_value(0, cc, "In List");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_is_in_list(&["a", "b", "c"]));
    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    book.push_sheet(sheet);

    test_write_ods(&mut book, "test_out/test_validation_1.ods")?;

    Ok(())
}

#[test]
fn test_validation1() -> Result<(), OdsError> {
    let mut book = WorkBook::new_empty();

    let mut sheet = Sheet::new("One");

    let cc: u32 = 0;
    sheet.set_value(0, cc, "Content Length");
    let mut valid = Validation::new();
    valid.set_condition(Condition::content_text_length_lt(5));
    let mut help = ValidationHelp::new();
    help.set_text(Some(TextP::new().text("may he help you!").into_xmltag()));
    valid.set_help(Some(help));
    let mut err = ValidationError::new();
    err.set_title(Some("Function disappeared".to_string()));
    err.set_text(Some(
        TextP::new()
            .text("who knows where it's gone?")
            .into_xmltag(),
    ));
    valid.set_err(Some(err));

    let valid = book.add_validation(valid);
    sheet.set_validation(1, cc, &valid);

    book.push_sheet(sheet);

    test_write_ods(&mut book, "test_out/test_validation_2.ods")?;

    Ok(())
}
