//! Text is stored as a simple String whenever possible.
//! When there is a more complex structure, a TextTag is constructed
//! which mirrors the Xml tree structure.
//!
//! For construction of a new TextTag structure a few helper structs are
//! defined.
//!
//! ```
//! use spreadsheet_ods::text::{TextP, TextTag, MetaAuthorName, MetaCreationDate, TextS};
//! use spreadsheet_ods::style::ParagraphStyleRef;
//!
//! let p1_ref = ParagraphStyleRef::from("p1");
//!
//! let txt = TextP::new()
//!             .style_name(&p1_ref)
//!             .text("some text")
//!             .tag(MetaAuthorName::new())
//!             .tag(TextS::new())
//!             .tag(MetaCreationDate::new())
//!             .tag(TextS::new())
//!             .text("whatever");
//! println!("{}", txt.into_xmltag());
//! ```
//!

use crate::style::{ParagraphStyleRef, TextStyleRef};
use crate::xmltree::{XmlContent, XmlTag};
use std::fmt::{Display, Formatter};

/// TextTags are just XmlTags.
pub type TextTag = XmlTag;
/// Content of a TextTag is just some XmlContent.
pub type TextContent = XmlContent;

text_tag!(TextH, "text:h");

// ok text:class-names 19.770.2,
// ok text:cond-style-name 19.776,
// ok text:id 19.809.6,
// ok text:is-list-header 19.816,
// ok text:outline-level 19.844.4,
// ok text:restart-numbering 19.857,
// ok text:start-value 19.868.2,
// ok text:style-name 19.874.7,
// ignore xhtml:about 19.905,
// ignore xhtml:content 19.906,
// ignore xhtml:datatype 19.907,
// ignore xhtml:property 19.908
// ok xml:id 19.914.
impl TextH {
    /// Sets class names aka paragraph styles as formatting.
    pub fn class_names(mut self, class_names: &[&ParagraphStyleRef]) -> Self {
        let mut buf = String::new();
        for n in class_names {
            buf.push_str(n.as_str());
            buf.push(' ');
        }
        self.xml.set_attr("text:class-names", buf);
        self
    }

    /// Sets a conditional style.
    pub fn condstyle_name(mut self, name: &ParagraphStyleRef) -> Self {
        self.xml
            .set_attr("text:condstyle-name", name.as_str().to_string());
        self
    }

    /// Identifier for a text passage.
    pub fn id(mut self, id: &str) -> Self {
        self.xml.set_attr("text:id", id);
        self
    }

    /// Styled as list header.
    pub fn list_header(mut self, lh: bool) -> Self {
        self.xml.set_attr("text:is-list-header", lh.to_string());
        self
    }

    /// Level of the heading.
    pub fn outline_level(mut self, l: u8) -> Self {
        self.xml.set_attr("text:outlinelevel", l.to_string());
        self
    }

    /// Numbering reset.
    pub fn restart_numbering(mut self, r: bool) -> Self {
        self.xml.set_attr("text:restart-numbering", r.to_string());
        self
    }

    /// Numbering start value.
    pub fn start_value(mut self, l: u8) -> Self {
        self.xml.set_attr("text:start-value", l.to_string());
        self
    }

    /// Style
    pub fn style_name(mut self, name: &ParagraphStyleRef) -> Self {
        self.xml
            .set_attr("text:style-name", name.as_str().to_string());
        self
    }

    /// xml-id
    pub fn xml_id(mut self, id: &str) -> Self {
        self.xml.set_attr("xml:id", id);
        self
    }
}

text_tag!(TextP, "text:p");

// ok text:class-names 19.770.3,
// ok text:cond-style-name 19.776,
// ok text:id 19.809.8,
// ok text:style-name 19.874.29,
// ignore xhtml:about 19.905,
// ignore xhtml:content 19.906,
// ignore xhtml:datatype 19.907,
// ignore xhtml:property 19.908
// ok xml:id 19.914.
impl TextP {
    /// Sets class names aka paragraph styles as formatting.
    pub fn class_names(mut self, class_names: &[&ParagraphStyleRef]) -> Self {
        let mut buf = String::new();
        for n in class_names {
            buf.push_str(n.as_str());
            buf.push(' ');
        }
        self.xml.set_attr("text:class-names", buf);
        self
    }

    /// Sets a conditional style.
    pub fn condstyle_name(mut self, name: &ParagraphStyleRef) -> Self {
        self.xml
            .set_attr("text:condstyle-name", name.as_str().to_string());
        self
    }

    /// Text id for a text passage.
    pub fn id(mut self, id: &str) -> Self {
        self.xml.set_attr("text:id", id);
        self
    }

    /// Style for this paragraph.
    pub fn style_name(mut self, name: &ParagraphStyleRef) -> Self {
        self.xml
            .set_attr("text:style-name", name.as_str().to_string());
        self
    }

    /// xml-id
    pub fn xml_id(mut self, id: &str) -> Self {
        self.xml.set_attr("xml:id", id);
        self
    }
}

// The <text:span> element represents the application of a style to the character data of a portion
// of text. The content of this element is the text which uses that text style.
//
// The <text:span> element can be nested.
//
// White space characters contained in this element are collapsed.
text_tag!(TextSpan, "text:span");

// text:class-names 19.770.4 and
// text:style-name 19.874.33.
impl TextSpan {
    /// A text:class-names attribute specifies a white space separated list of text style names.
    pub fn class_names(mut self, class_names: &[&TextStyleRef]) -> Self {
        let mut buf = String::new();
        for n in class_names {
            buf.push_str(n.as_str());
            buf.push(' ');
        }
        self.xml.set_attr("text:class-names", buf);
        self
    }

    /// The text:style-name attribute specifies style for span which shall be a style with family of
    /// text.
    /// If both text:style-name and text:class-names are present, the style referenced by the
    /// text:style-name attribute is treated as the first style in the list in text:class-names.
    /// Consumers should support the text:class-names attribute and also should preserve it while
    /// editing.
    pub fn style_name(mut self, name: &TextStyleRef) -> Self {
        self.xml
            .set_attr("text:style-name", name.as_str().to_string());
        self
    }
}

// The <text:a> element represents a hyperlink.
//
// The anchor of a hyperlink is composed of the character data contained by the <text:a> element
// and any of its descendant elements which constitute the character data of the paragraph which
// contains the <text:a> element. 6.1.1
text_tag!(TextA, "text:a");

// obsolete office:name 19.376.9,
// ??? office:target-frame-name 19.381,
// ??? office:title 19.383,
// ok text:style-name 19.874.2,
// ok text:visited-style-name 19.901,
// ??? xlink:actuate 19.909,
// ok xlink:href 19.910.33,
// ??? xlink:show 19.911 and
// ??? xlink:type 19.913.
impl TextA {
    /// The text:style-name attribute specifies a text style for an unvisited hyperlink.
    pub fn style_name(mut self, style: &TextStyleRef) -> Self {
        self.xml
            .set_attr("text:style-name", style.as_str().to_string());
        self
    }

    /// The text:visited-style-name attribute specifies a style for a hyperlink that has been visited.
    pub fn visited_style_name(mut self, style: &TextStyleRef) -> Self {
        self.xml
            .set_attr("text:visited-style-name", style.as_str().to_string());
        self
    }

    /// href for a link.
    pub fn href<S: Into<String>>(mut self, uri: S) -> Self {
        self.xml.set_attr("xlink:href", uri.into());
        self
    }
}

// The <text:s> element is used to represent the [UNICODE] character “ “ (U+0020, SPACE).
// This element shall be used to represent the second and all following “ “ (U+0020, SPACE)
// characters in a sequence of “ “ (U+0020, SPACE) characters.
//
// Note: It is not an error if the character preceding the element is not a white space character, but it
// is good practice to use this element only for the second and all following “ “ (U+0020, SPACE)
// characters in a sequence.
text_tag!(TextS, "text:s");

// text:c 19.763.
impl TextS {
    /// The text:c attribute specifies the number of “ “ (U+0020, SPACE) characters that a <text:s>
    /// element represents. A missing text:c attribute is interpreted as a single “ “ (U+0020, SPACE)
    /// character.
    pub fn count(mut self, count: u32) -> Self {
        self.xml.set_attr("text:c", count.to_string());
        self
    }
}

// The <text:tab> element represents the [UNICODE] tab character (HORIZONTAL
// TABULATION, U+0009).
//
// A <text:tab> element specifies that content immediately following it
// should begin at the next tab stop.
text_tag!(TextTab, "text:tab");

impl TextTab {
    /// The text:tab-ref attribute contains the number of the tab-stop to which a tab character refers.
    /// The position 0 marks the start margin of a paragraph.
    ///
    /// Note: The text:tab-ref attribute is only a hint to help non-layout oriented consumers to
    /// determine the tab/tab-stop association. Layout oriented consumers should determine the tab
    /// positions based on the style information.
    pub fn tab_ref(mut self, tab_ref: u32) -> Self {
        self.xml.set_attr("text:tab-ref", tab_ref.to_string());
        self
    }
}

// The <text:soft-page-break> element represents a soft page break within or between
// paragraph elements. As a child element of a <table:table> element it represents a soft page break between two
// table rows. It may appear in front of a <table:table-row> element.
text_tag!(SoftPageBreak, "text:soft-page-break");

// The <text:date> element displays a date, by default this is the current date. The date can be
// adjusted to display a date other than the current date.
text_tag!(MetaDate, "text:date");
text_tag!(MetaTime, "text:time");
// text:page-continuation
text_tag!(MetaPageNumber, "text:page-number");
// text:sender-firstname
// text:sender-lastname
// text:sender-initials
// text:sender-title
// text:sender-position
// text:sender-email
// text:sender-phone-private
// text:sender-fax
// text:sender-company
// text:sender-phone-work
// text:sender-street
// text:sender-city
// text:sender-postal-code
// text:sender-country
// text:sender-state-or-province
// The <text:author-name> element represents the full name of the author of a document.
text_tag!(MetaAuthorName, "text:author-name");
// The <text:author-initials> element represents the initials of the author of a document.
text_tag!(MetaAuthorInitials, "text:author-initials");
// text:chapter
text_tag!(MetaFileName, "text:file-name");
// text:template-name
text_tag!(MetaSheetName, "text:sheet-name");
text_tag!(MetaInitialCreator, "text:initial-creator");
text_tag!(MetaCreationDate, "text:creation-date");
text_tag!(MetaCreationTime, "text:creation-time");
text_tag!(MetaDescription, "text:description");
// text:user-defined
text_tag!(MetaPrintTime, "text:print-time");
text_tag!(MetaPrintDate, "text:print-date");
text_tag!(MetaPrintedBy, "text:printed-by");
text_tag!(MetaTitle, "text:title");
text_tag!(MetaSubject, "text:subject");
text_tag!(MetaKeywords, "text:keywords");
text_tag!(MetaEditingCycles, "text:editing-cycles");
text_tag!(MetaEditingDuration, "text:editing-duration");
text_tag!(MetaModificationTime, "text:modification-time");
text_tag!(MetaModificationDate, "text:modification-date");
text_tag!(MetaCreator, "text:creator");
text_tag!(MetaPageCount, "text:page-count");
// text:paragraph-count
// text:word-count
text_tag!(MetaCharacterCount, "text:character-count");
// text:table-count
// text:image-count
// text:object-count
// text:meta-field
