use crate::style::pagestyle::PageStyleRef;
use crate::style::AnyStyleRef;
use crate::text::TextTag;
use get_size::GetSize;
use get_size_derive::GetSize;
use std::borrow::Borrow;

style_ref2!(MasterPageRef);

/// Defines the structure and content for a page.
/// Refers to a PageStyle for layout information.
/// It must be attached to a Sheet to be used.
///
/// ```
/// use spreadsheet_ods::{pt, Length, WorkBook, Sheet};
/// use spreadsheet_ods::style::{PageStyle, MasterPage, TableStyle};
/// use spreadsheet_ods::style::units::Border;
/// use spreadsheet_ods::xmltree::XmlVec;
/// use spreadsheet_ods::color::Rgb;
/// use icu_locid::locale;
///
/// let mut wb = WorkBook::new(locale!("en_US"));
///
/// let mut ps = PageStyle::new("ps1");
/// ps.set_border(pt!(0.5), Border::Groove, Rgb::new(128,128,128));
/// ps.headerstyle_mut().set_background_color(Rgb::new(92,92,92));
/// let ps_ref = wb.add_pagestyle(ps);
///
/// let mut mp1 = MasterPage::new("mp1");
/// mp1.set_pagestyle(&ps_ref);
/// mp1.header_mut().center_mut().add_text("center");
/// mp1.footer_mut().right_mut().add_text("right");
/// let mp1_ref = wb.add_masterpage(mp1);
///
/// let mut ts = TableStyle::new("ts1");
/// ts.set_master_page(&mp1_ref);
/// let ts_ref = wb.add_tablestyle(ts);
///
/// let mut sheet = Sheet::new("sheet 1");
/// sheet.set_style(&ts_ref);
/// ```
#[derive(Clone, Debug, Default, GetSize)]
pub struct MasterPage {
    name: String,
    display_name: String,
    pagestyle: Option<PageStyleRef>,
    next_style_name: Option<MasterPageRef>,

    header: HeaderFooter,
    header_first: HeaderFooter,
    header_left: HeaderFooter,

    footer: HeaderFooter,
    footer_first: HeaderFooter,
    footer_left: HeaderFooter,
}

impl MasterPage {
    /// Empty.
    pub fn new_empty() -> Self {
        Self {
            name: Default::default(),
            display_name: Default::default(),
            pagestyle: Default::default(),
            next_style_name: Default::default(),
            header: Default::default(),
            header_first: Default::default(),
            header_left: Default::default(),
            footer: Default::default(),
            footer_first: Default::default(),
            footer_left: Default::default(),
        }
    }

    /// New MasterPage
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
            display_name: Default::default(),
            pagestyle: Default::default(),
            next_style_name: Default::default(),
            header: Default::default(),
            header_first: Default::default(),
            header_left: Default::default(),
            footer: Default::default(),
            footer_first: Default::default(),
            footer_left: Default::default(),
        }
    }

    /// Style reference.
    pub fn masterpage_ref(&self) -> MasterPageRef {
        MasterPageRef::from(self.name())
    }

    /// Name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Name.
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    /// Name.
    pub fn display_name(&self) -> &String {
        &self.display_name
    }

    /// Reference to a page-style.
    pub fn set_pagestyle(&mut self, name: &PageStyleRef) {
        self.pagestyle = Some(name.clone());
    }

    /// Reference to a page-style.
    pub fn pagestyle(&self) -> Option<&PageStyleRef> {
        self.pagestyle.as_ref()
    }

    /// The style:next-style-name attribute specifies the name of the master page that is used for
    /// the next page if the current page is entirely filled. If the next style name is not specified, the
    /// current master page is used for the next page. The value of this attribute shall be the name of a
    /// <style:master-page> element.
    pub fn set_next_masterpage(&mut self, master: &MasterPageRef) {
        self.next_style_name = Some(master.clone());
    }

    ///
    pub fn next_masterpage(&self) -> Option<&MasterPageRef> {
        self.next_style_name.as_ref()
    }

    /// Left side header.
    pub fn set_header(&mut self, header: HeaderFooter) {
        self.header = header;
    }

    /// Left side header.
    pub fn header(&self) -> &HeaderFooter {
        &self.header
    }

    /// Header.
    pub fn header_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header
    }

    /// First page header.
    pub fn set_header_first(&mut self, header: HeaderFooter) {
        self.header_first = header;
    }

    /// First page header.
    pub fn header_first(&self) -> &HeaderFooter {
        &self.header_first
    }

    /// First page header.
    pub fn header_first_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_first
    }

    /// Left side header.
    pub fn set_header_left(&mut self, header: HeaderFooter) {
        self.header_left = header;
    }

    /// Left side header.
    pub fn header_left(&self) -> &HeaderFooter {
        &self.header_left
    }

    /// Left side header.
    pub fn header_left_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_left
    }

    /// Footer.
    pub fn set_footer(&mut self, footer: HeaderFooter) {
        self.footer = footer;
    }

    /// Footer.
    pub fn footer(&self) -> &HeaderFooter {
        &self.footer
    }

    /// Footer.
    pub fn footer_mut(&mut self) -> &mut HeaderFooter {
        &mut self.footer
    }

    /// First page footer.
    pub fn set_footer_first(&mut self, footer: HeaderFooter) {
        self.footer_first = footer;
    }

    /// First page footer.
    pub fn footer_first(&self) -> &HeaderFooter {
        &self.footer_first
    }

    /// First page footer.
    pub fn footer_first_mut(&mut self) -> &mut HeaderFooter {
        &mut self.footer_first
    }

    /// Left side footer.
    pub fn set_footer_left(&mut self, footer: HeaderFooter) {
        self.footer_left = footer;
    }

    /// Left side footer.
    pub fn footer_left(&self) -> &HeaderFooter {
        &self.footer_left
    }

    /// Left side footer.
    pub fn footer_left_mut(&mut self) -> &mut HeaderFooter {
        &mut self.footer_left
    }
}

/// Header/Footer data.
/// Can be seen as three regions left/center/right or as one region.
/// In the first case region* contains the data, in the second it's content.
/// Each is a TextTag of parsed XML-tags.
#[derive(Clone, Debug, Default, GetSize)]
pub struct HeaderFooter {
    display: bool,

    region_left: Vec<TextTag>,
    region_center: Vec<TextTag>,
    region_right: Vec<TextTag>,

    content: Vec<TextTag>,
}

impl HeaderFooter {
    /// Create
    pub fn new() -> Self {
        Self {
            display: true,
            region_left: Default::default(),
            region_center: Default::default(),
            region_right: Default::default(),
            content: Default::default(),
        }
    }

    /// Is the header displayed. Used to deactivate left side headers.
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Display
    pub fn display(&self) -> bool {
        self.display
    }

    /// true if all regions of the header/footer are empty.
    pub fn is_empty(&self) -> bool {
        self.region_left.is_empty()
            && self.region_center.is_empty()
            && self.region_right.is_empty()
            && self.content.is_empty()
    }

    /// Set the content of the left region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn set_left(&mut self, txt: Vec<TextTag>) {
        self.region_left = txt;
    }

    /// Adds to the content of the left region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn add_left(&mut self, txt: TextTag) {
        self.region_left.push(txt);
    }

    /// Clear left region.
    pub fn clear_left(&mut self) {
        self.region_left = Vec::new();
    }

    /// Left region.
    pub fn left(&self) -> &Vec<TextTag> {
        self.region_left.as_ref()
    }

    /// Left region.
    pub fn left_mut(&mut self) -> &mut Vec<TextTag> {
        self.region_left.as_mut()
    }

    /// Set the content of the center region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn set_center(&mut self, txt: Vec<TextTag>) {
        self.region_center = txt;
    }

    /// Adds to the content of the center region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn add_center(&mut self, txt: TextTag) {
        self.region_center.push(txt);
    }

    /// Center region.
    pub fn clear_center(&mut self) {
        self.region_center = Vec::new();
    }

    /// Center region.
    pub fn center(&self) -> &Vec<TextTag> {
        self.region_center.as_ref()
    }

    /// Center region.
    pub fn center_mut(&mut self) -> &mut Vec<TextTag> {
        self.region_center.as_mut()
    }

    /// Set the content of the right region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn set_right(&mut self, txt: Vec<TextTag>) {
        self.region_right = txt;
    }

    /// Adds to the content of the right region of the header.
    ///
    /// Attention:
    /// This tag must be a text:p otherwise its ignored.
    pub fn add_right(&mut self, txt: TextTag) {
        self.region_right.push(txt);
    }

    /// Right region.
    pub fn clear_right(&mut self) {
        self.region_right = Vec::new();
    }

    /// Right region.
    pub fn right(&self) -> &Vec<TextTag> {
        self.region_right.as_ref()
    }

    /// Right region.
    pub fn right_mut(&mut self) -> &mut Vec<TextTag> {
        self.region_right.as_mut()
    }

    /// Header content, if there are no regions.
    pub fn set_content(&mut self, txt: Vec<TextTag>) {
        self.content = txt;
    }

    /// Adds header content, if there are no regions.
    pub fn add_content(&mut self, txt: TextTag) {
        self.content.push(txt);
    }

    /// Header content, if there are no regions.
    pub fn content(&self) -> &Vec<TextTag> {
        &self.content
    }

    /// Header content, if there are no regions.
    pub fn content_mut(&mut self) -> &mut Vec<TextTag> {
        &mut self.content
    }
}
