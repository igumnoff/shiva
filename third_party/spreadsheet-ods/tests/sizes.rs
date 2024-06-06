use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;
use std::sync::Arc;

use chrono::{Duration, NaiveDateTime};

use spreadsheet_ods::metadata::Metadata;
use spreadsheet_ods::sheet::{Grouped, SheetConfig};
use spreadsheet_ods::style::{TableStyle, TableStyleRef};
use spreadsheet_ods::text::TextTag;
use spreadsheet_ods::{CellRange, ColRange, RowRange, Sheet, Value, WorkBook};

#[test]
pub fn sizes() {
    println!("WorkBook {}", size_of::<WorkBook>());
    println!("Sheet {}", size_of::<Sheet>());
    println!("Metadata {}", size_of::<Metadata>());

    println!("(ucell,ucell) {}", size_of::<(u32, u32)>());

    println!("Value {}", size_of::<Value>());

    println!("bool {}", size_of::<bool>());
    println!("f64 {}", size_of::<f64>());
    println!("f64, Box<str> {}", size_of::<(f64, Box<str>)>());
    println!("String {}", size_of::<String>());
    println!("Vec<TextTag> {}", size_of::<Vec<TextTag>>());
    println!(
        "HashMap<String, TableStyle> {}",
        size_of::<HashMap<String, TableStyle>>()
    );
    println!("NaiveDateTime {}", size_of::<NaiveDateTime>());
    println!("Duration {}", size_of::<Duration>());
}

#[test]
pub fn sizes2() {
    println!("Sheet {}", size_of::<Sheet>());
    println!("TableStyleRef {}", size_of::<TableStyleRef>());
    // println!("CellData {}", size_of::<CellData>());
    // println!("CellDataExt {}", size_of::<CellDataExt>());
    // println!("ColHeader {}", size_of::<ColHeader>());
    // println!("RowHeader {}", size_of::<RowHeader>());
    println!("RowRange {}", size_of::<RowRange>());
    println!("ColRange {}", size_of::<ColRange>());
    println!("CellRange {}", size_of::<CellRange>());
    println!("Grouped {}", size_of::<Grouped>());
    println!("SheetConfig {}", size_of::<SheetConfig>());

    println!("String {}", size_of::<String>());
    println!("Cow<str> {}", size_of::<Cow<'_, str>>());
    println!("Rc<str> {}", size_of::<Rc<str>>());
    println!("Arc<str> {}", size_of::<Arc<str>>());
    println!("Box<str> {}", size_of::<Box<str>>());
}
