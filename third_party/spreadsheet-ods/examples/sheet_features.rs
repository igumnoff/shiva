//! show some features of sheets
use color::Rgb;
use icu_locid::locale;
use spreadsheet_ods::style::units::{Border, Margin, PrintCentering};
use spreadsheet_ods::style::{HeaderFooter, MasterPage, PageStyle, TableStyle};
use spreadsheet_ods::text::{
    MetaAuthorName, MetaDate, MetaPageCount, MetaPageNumber, MetaTime, TextP, TextS,
};
use spreadsheet_ods::{cm, pt, write_ods, CellRange, Length, OdsResult, Sheet, WorkBook};

///
pub fn main() -> OdsResult<()> {
    printing()?;
    cell_grid()?;

    Ok(())
}

// visual stuff with a sheet
fn cell_grid() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    let mut sh = Sheet::new("grouping");
    sheet_data(&mut sh);

    // widths
    sh.set_col_width(0, cm!(0.55));
    sh.set_col_width(1, cm!(0.90));
    sh.set_col_width(2, cm!(1.48));
    sh.set_col_width(3, cm!(2.44));
    sh.set_col_width(4, cm!(4));

    // hide grid
    sh.config_mut().show_grid = false;

    // groups
    sh.add_col_group(1, 2);
    sh.add_col_group(3, 4);

    // split header
    sh.split_row_header(1);

    // spans
    sh.set_col_span(0, 0, 5);

    wb.push_sheet(sh);

    write_ods(&mut wb, "examples_out/visual.ods")?;

    Ok(())
}

// print ranges & headers & footers
fn printing() -> OdsResult<()> {
    let mut wb = WorkBook::new(locale!("de_AT"));

    // page styling
    let mut s_page = PageStyle::new("one");
    s_page.set_table_centering(PrintCentering::Both);
    s_page.set_margin(Margin::Length(cm!(1)));
    s_page.set_border(pt!(1), Border::Dotted, Rgb::new(0, 0, 0));

    let s_footer = s_page.footerstyle_mut();
    s_footer.set_background_color(Rgb::new(248, 128, 128));
    // seems to have no effect
    s_footer.set_dynamic_spacing(false);
    // works!
    s_footer.set_min_height(cm!(0.7));
    // has no effect
    // s_footer.set_height(cm!(2));
    s_footer.set_margin(Margin::Length(cm!(0)));
    let s_page = wb.add_pagestyle(s_page);

    // header/footer data
    let mut m_page = MasterPage::new("one");
    m_page.set_pagestyle(&s_page);

    let mut m_footer = HeaderFooter::new();
    m_footer.add_left(TextP::new().tag(MetaAuthorName::new()).into_xmltag());
    m_footer.add_center(
        TextP::new()
            .tag(MetaPageNumber::new())
            .tag(TextS::new())
            .text("/")
            .tag(TextS::new())
            .tag(MetaPageCount::new())
            .into_xmltag(),
    );
    m_footer.add_right(
        TextP::new()
            .tag(MetaDate::new())
            .tag(TextS::new())
            .tag(MetaTime::new())
            .into_xmltag(),
    );
    m_page.set_footer(m_footer);
    let m_page = wb.add_masterpage(m_page);

    // per table page-settings.
    let mut s_table = TableStyle::new("one");
    s_table.set_master_page(&m_page);
    let s_table = wb.add_tablestyle(s_table);

    let mut sh = Sheet::new("one");
    sh.set_style(&s_table);
    sheet_data(&mut sh);

    // define print header
    sh.set_header_rows(0, 1);
    // restriction on the print-data.
    sh.add_print_range(CellRange::local(1, 0, 1001, 3));

    wb.push_sheet(sh);

    write_ods(&mut wb, "examples_out/printing.ods")?;

    Ok(())
}

fn sheet_data(sh: &mut Sheet) {
    sh.set_value(0, 0, "Heading");
    let heading = ["A", "B", "C", "D", "E"];
    for c in 0..5 {
        sh.set_value(1, c, heading[c as usize]);
    }
    for c in 0..5 {
        for r in 0..1000 {
            sh.set_value(r + 2, c, r * c);
        }
    }
}
