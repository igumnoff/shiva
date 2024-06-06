mod lib_test;

use color::Rgb;
use lib_test::*;
use spreadsheet_ods::condition::Condition;
use spreadsheet_ods::style::stylemap::StyleMap;
use spreadsheet_ods::style::units::{
    Angle, Border, CellAlignVertical, FontFamilyGeneric, FontPitch, FontWeight, Length, PageBreak,
    ParaAlignVertical, RotationAlign, TextAlignSource, TextKeep, TextPosition, TextRelief,
    TextTransform, WrapOption, WritingMode,
};
use spreadsheet_ods::style::{
    CellStyle, ColStyle, FontFaceDecl, PageStyle, RowStyle, StyleOrigin, StyleUse, TableStyle,
};
use spreadsheet_ods::{cm, deg, mm, pt, CellRef, OdsError, Sheet, WorkBook};

#[test]
fn test_attr1() {
    let mut p0 = PageStyle::new("ps1");

    p0.set_background_color(Rgb::new(12, 33, 46));
    assert_eq!(p0.style().attr("fo:background-color"), Some("#0c212e"));

    p0.set_border(pt!(1), Border::Groove, Rgb::new(99, 0, 0));
    assert_eq!(p0.style().attr("fo:border"), Some("1pt groove #630000"));

    p0.set_border_line_width(pt!(1), pt!(2), pt!(3));
    assert_eq!(
        p0.style().attr("style:border-line-width"),
        Some("1pt 2pt 3pt")
    );

    p0.set_margin(Length::Pt(3.2).into());
    assert_eq!(p0.style().attr("fo:margin"), Some("3.2pt"));

    p0.set_margin(pt!(3.2));
    assert_eq!(p0.style().attr("fo:margin"), Some("3.2pt"));

    p0.set_padding(pt!(3.3));
    assert_eq!(p0.style().attr("fo:padding"), Some("3.3pt"));

    p0.set_dynamic_spacing(true);
    assert_eq!(p0.style().attr("style:dynamic-spacing"), Some("true"));

    p0.set_shadow(mm!(3), mm!(4), None, Rgb::new(16, 16, 16));
    assert_eq!(p0.style().attr("style:shadow"), Some("#101010 3mm 4mm"));

    p0.headerstyle_mut().set_height(cm!(7));
    assert_eq!(p0.headerstyle().style().attr("svg:height"), Some("7cm"));

    p0.headerstyle_mut().set_min_height(cm!(6));
    assert_eq!(
        p0.headerstyle_mut().style().attr("fo:min-height"),
        Some("6cm")
    );

    p0.headerstyle_mut().set_dynamic_spacing(true);
    assert_eq!(
        p0.headerstyle_mut().style().attr("style:dynamic-spacing"),
        Some("true")
    );
}

#[test]
fn test_attr2() {
    let mut ff = FontFaceDecl::new("Helvetica");

    ff.set_font_family("Helvetica");
    assert_eq!(ff.attrmap().attr("svg:font-family"), Some("Helvetica"));

    ff.set_font_family_generic(FontFamilyGeneric::System);
    assert_eq!(
        ff.attrmap().attr("style:font-family-generic"),
        Some("system")
    );

    ff.set_font_pitch(FontPitch::Fixed);
    assert_eq!(ff.attrmap().attr("style:font-pitch"), Some("fixed"));
}

#[test]
fn test_attr3() {
    let mut st = TableStyle::new("c00");

    st.set_break_before(PageBreak::Page);
    assert_eq!(st.tablestyle().attr("fo:break-before"), Some("page"));

    st.set_break_after(PageBreak::Page);
    assert_eq!(st.tablestyle().attr("fo:break-after"), Some("page"));

    st.set_keep_with_next(TextKeep::Auto);
    assert_eq!(st.tablestyle().attr("fo:keep-with-next"), Some("auto"));

    st.set_writing_mode(WritingMode::TbLr);
    assert_eq!(st.tablestyle().attr("style:writing-mode"), Some("tb-lr"));

    let mut st = ColStyle::new("c01");

    st.set_use_optimal_col_width(true);
    assert_eq!(
        st.colstyle().attr("style:use-optimal-column-width"),
        Some("true")
    );

    st.set_rel_col_width(33.0);
    assert_eq!(st.colstyle().attr("style:rel-column-width"), Some("33*"));

    st.set_col_width(cm!(17));
    assert_eq!(st.colstyle().attr("style:column-width"), Some("17cm"));

    let mut st = RowStyle::new("r02");

    st.set_use_optimal_row_height(true);
    assert_eq!(
        st.rowstyle().attr("style:use-optimal-row-height"),
        Some("true")
    );

    st.set_min_row_height(pt!(77));
    assert_eq!(st.rowstyle().attr("style:min-row-height"), Some("77pt"));

    st.set_row_height(pt!(77));
    assert_eq!(st.rowstyle().attr("style:row-height"), Some("77pt"));
}

#[test]
fn test_attr4() {
    let mut st = CellStyle::new("c00", &"f00".into());

    st.set_diagonal_bl_tr(pt!(0.2), Border::Ridge, Rgb::new(0, 127, 0));
    assert_eq!(
        st.cellstyle().attr("style:diagonal-bl-tr"),
        Some("0.2pt ridge #007f00")
    );

    st.set_diagonal_tl_br(pt!(0.2), Border::Ridge, Rgb::new(0, 127, 0));
    assert_eq!(
        st.cellstyle().attr("style:diagonal-bl-tr"),
        Some("0.2pt ridge #007f00")
    );

    st.set_wrap_option(WrapOption::Wrap);
    assert_eq!(st.cellstyle().attr("fo:wrap-option"), Some("wrap"));

    st.set_print_content(true);
    assert_eq!(st.cellstyle().attr("style:print-content"), Some("true"));

    st.set_repeat_content(true);
    assert_eq!(st.cellstyle().attr("style:repeat-content"), Some("true"));

    st.set_rotation_align(RotationAlign::Center);
    assert_eq!(st.cellstyle().attr("style:rotation-align"), Some("center"));

    st.set_rotation_angle(deg!(42));
    assert_eq!(st.cellstyle().attr("style:rotation-angle"), Some("42deg"));

    st.set_shrink_to_fit(true);
    assert_eq!(st.cellstyle().attr("style:shrink-to-fit"), Some("true"));

    st.set_vertical_align(CellAlignVertical::Top);
    assert_eq!(st.cellstyle().attr("style:vertical-align"), Some("top"));
}

#[test]
fn test_attr5() {
    let mut st = CellStyle::new("c00", &"f00".into());

    st.set_vertical_align_para(ParaAlignVertical::Baseline);
    assert_eq!(
        st.paragraphstyle().attr("style:vertical-align"),
        Some("baseline")
    );

    st.set_line_spacing(pt!(4));
    assert_eq!(st.paragraphstyle().attr("style:line-spacing"), Some("4pt"));

    st.set_number_lines(true);
    assert_eq!(st.paragraphstyle().attr("text:number-lines"), Some("true"));

    st.set_text_align_source(TextAlignSource::ValueType);
    assert_eq!(
        st.cellstyle().attr("style:text-align-source"),
        Some("value-type")
    );

    st.set_text_indent(mm!(4.2));
    assert_eq!(st.paragraphstyle().attr("fo:text-indent"), Some("4.2mm"));
}

#[test]
fn test_attr6() {
    let mut st = CellStyle::new("c00", &"f00".into());

    st.set_font_bold();
    assert_eq!(st.textstyle().attr("fo:font-weight"), Some("bold"));

    st.set_font_weight(FontWeight::W700);
    assert_eq!(st.textstyle().attr("fo:font-weight"), Some("700"));

    st.set_font_size(pt!(13));
    assert_eq!(st.textstyle().attr("fo:font-size"), Some("13pt"));

    st.set_color(Rgb::new(0, 0, 128));
    assert_eq!(st.textstyle().attr("fo:color"), Some("#000080"));

    st.set_font_italic();
    assert_eq!(st.textstyle().attr("fo:font-style"), Some("italic"));

    st.set_font_name("Boing");
    assert_eq!(st.textstyle().attr("style:font-name"), Some("Boing"));

    st.set_font_relief(TextRelief::Engraved);
    assert_eq!(st.textstyle().attr("style:font-relief"), Some("engraved"));

    st.set_letter_spacing(pt!(0.2));
    assert_eq!(st.textstyle().attr("fo:letter-spacing"), Some("0.2pt"));

    st.set_text_position(TextPosition::Sub, None);
    assert_eq!(st.textstyle().attr("style:text-position"), Some("sub"));

    st.set_text_transform(TextTransform::Lowercase);
    assert_eq!(st.textstyle().attr("fo:text-transform"), Some("lowercase"));
}

#[test]
fn testtablestyle() {
    let mut s = TableStyle::new("fine");
    s.set_background_color(Rgb::new(0, 0, 0));
}

#[test]
fn test_stylemap() -> Result<(), OdsError> {
    let mut wb = WorkBook::new_empty();

    let mut ce12 = CellStyle::new("ce12", &"num2".into());
    ce12.set_origin(StyleOrigin::Styles);
    ce12.set_styleuse(StyleUse::Named);
    ce12.set_display_name("CC12");
    ce12.set_color(Rgb::new(192, 128, 0));
    wb.add_cellstyle(ce12);

    let mut ce11 = CellStyle::new("ce11", &"num2".into());
    ce11.set_origin(StyleOrigin::Styles);
    ce11.set_styleuse(StyleUse::Named);
    ce11.set_display_name("CC11");
    ce11.set_color(Rgb::new(0, 192, 128));
    wb.add_cellstyle(ce11);

    let mut ce13 = CellStyle::new("ce13", &"num4".into());
    ce13.push_stylemap(StyleMap::new(
        Condition::content_eq("BB"),
        "ce12".into(),
        Some(CellRef::remote("s0", 4, 3)),
    ));
    ce13.push_stylemap(StyleMap::new(
        Condition::content_eq("CC"),
        "ce11".into(),
        Some(CellRef::remote("s0", 4, 3)),
    ));
    let ce13 = wb.add_cellstyle(ce13);

    let mut sh = Sheet::new("s0");
    sh.set_styled_value(4, 3, "AA", &ce13);
    sh.set_styled_value(5, 3, "BB", &ce13);
    sh.set_styled_value(6, 3, "CC", &ce13);

    wb.push_sheet(sh);

    test_write_ods(&mut wb, "test_out/test_style_attr.ods")?;

    Ok(())
}
