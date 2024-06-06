use spreadsheet_ods::style::ParagraphStyleRef;
use spreadsheet_ods::text::{MetaAuthorName, MetaCreationDate, TextH, TextP, TextS, TextTag};

#[test]
fn test_text() {
    let txt = TextTag::new("text:p")
        .tag(MetaAuthorName::new())
        .tag(TextH::new().style_name(&"style0".into()).text("wablawa"));

    assert_eq!(
        txt.to_string(),
        r#"<text:p>
<text:author-name/>
<text:h text:style-name="style0">
wablawa
</text:h>
</text:p>
"#
    );
}

#[test]
fn test_text2() {
    let p1_ref = ParagraphStyleRef::from("p1");

    let txt = TextP::new()
        .style_name(&p1_ref)
        .text("some text")
        .tag(MetaAuthorName::new())
        .tag(TextS::new())
        .tag(MetaCreationDate::new())
        .tag(TextS::new())
        .text("whatever");
    assert_eq!(
        txt.to_string(),
        r#"<text:p text:style-name="p1">
some text
<text:author-name/>
<text:s/>
<text:creation-date/>
<text:s/>
whatever
</text:p>
"#
    )
}
