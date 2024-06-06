use spreadsheet_ods::{CellStyleRef, Sheet, ValueType, WorkBook};

#[test]
fn test_workbook() {
    let mut wb = WorkBook::new_empty();

    let sh = Sheet::new("1");
    // println!("sizeof Sheet {}", size_of_val(&sh));
    wb.push_sheet(sh);
    assert_eq!(wb.num_sheets(), 1);
    wb.push_sheet(Sheet::new("b"));
    wb.push_sheet(Sheet::new("c"));
    assert_eq!(wb.sheet(1).name(), "b");
    wb.insert_sheet(1, Sheet::new("x"));
    assert_eq!(wb.sheet(1).name(), "x");
    let sh = wb.remove_sheet(1);
    assert_eq!(sh.name(), "x");
    assert_eq!(wb.num_sheets(), 3);
}

#[test]
fn test_def_style() {
    let mut wb = WorkBook::new_empty();

    wb.add_def_style(ValueType::Number, CellStyleRef::from("val0"));
    assert_eq!(
        wb.def_style(ValueType::Number),
        Some(&CellStyleRef::from("val0"))
    );
    assert!(wb.def_style(ValueType::Text).is_none());
}
