macro_rules! fo_background_color {
    ($acc:ident) => {
        /// The fo:background-color attribute specifies a background color for characters, paragraphs,
        /// text sections, frames, page bodies, headers, footers, table cells, table rows and tables. This can
        /// be transparent or a color. If the value is set to transparent, it switches off any background
        /// image that is specified by a <style:background-image> 17.3.
        ///
        /// If a value for a draw:fill attribute is provided in a style, any background image that is specified
        /// by a <style:background-image> element and any background color that is specified with the
        /// fo:background-color attribute are switched off.
        pub fn set_background_color(&mut self, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:background-color", color_string(color));
        }
    };
}

macro_rules! fo_border {
    ($acc:ident) => {
        /// Border style all four sides. See §7.29.3 of XSL.
        pub fn set_border(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:border", border_string(width, border, color));
        }

        /// Border style. See §7.29.4 of XSL
        pub fn set_border_bottom(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:border-bottom", border_string(width, border, color));
        }

        /// Border style. See §7.29.6 of XSL.
        pub fn set_border_left(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:border-left", border_string(width, border, color));
        }

        /// Border style. See §7.29.7 of XSL.
        pub fn set_border_right(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:border-right", border_string(width, border, color));
        }

        /// Border style. See §7.29.10 of XSL.
        pub fn set_border_top(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("fo:border-top", border_string(width, border, color));
        }
    };
}

macro_rules! fo_padding {
    ($acc:ident) => {
        /// Padding for all sides. See §7.29.15 of XSL.
        ///
        /// The fo:padding attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tablecell-properties 17.18.
        pub fn set_padding(&mut self, padding: Length) {
            assert!(padding.is_positive());
            self.$acc.set_attr("fo:padding", padding.to_string());
        }

        /// Padding. See §7.7.36 of XSL.
        ///
        /// The fo:padding-bottom attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tablecell-properties 17.18.
        pub fn set_padding_bottom(&mut self, padding: Length) {
            assert!(padding.is_positive());
            self.$acc.set_attr("fo:padding-bottom", padding.to_string());
        }

        /// Padding. See §7.7.37 of XSL.
        ///
        /// The fo:padding-left attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tablecell-properties 17.18.
        pub fn set_padding_left(&mut self, padding: Length) {
            assert!(padding.is_positive());
            self.$acc.set_attr("fo:padding-left", padding.to_string());
        }

        /// Padding. See §7.7.38 of XSL.
        ///
        /// The fo:padding-right attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tablecell-properties 17.18.
        pub fn set_padding_right(&mut self, padding: Length) {
            assert!(padding.is_positive());
            self.$acc.set_attr("fo:padding-right", padding.to_string());
        }

        /// Padding. See §7.7.35 of XSL.
        ///
        /// The fo:padding-top attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tablecell-properties 17.18.
        pub fn set_padding_top(&mut self, padding: Length) {
            assert!(padding.is_positive());
            self.$acc.set_attr("fo:padding-top", padding.to_string());
        }
    };
}

macro_rules! fo_wrap_option {
    ($acc:ident) => {
        // fo:wrap-option 20.230,
        /// See §7.15.13 of XSL.
        /// If wrapping is disabled, it is implementation-defined whether the overflow text is visible or hidden.
        /// If the text is hidden consumers may support a scrolling to access the text.
        pub fn set_wrap_option(&mut self, wrap: WrapOption) {
            self.$acc.set_attr("fo:wrap-option", wrap.to_string());
        }
    };
}

macro_rules! fo_border_line_width {
    ($acc:ident) => {
        /// The style:border-line-width attribute specifies the widths of borders defined by the FO
        /// border properties (see 20.183) for borders where the value of these properties is double.
        /// The value of the style:border-line-width attribute is a list of three white space-separated
        /// lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specifies the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// style:graphic-properties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:page-layout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:table-cell-properties 17.18.
        pub fn set_border_line_width(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc.set_attr(
                "style:border-line-width",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-bottom attribute specifies the widths of the bottom border
        /// for borders defined by the FO border properties (see 20.183) if the property for the bottom border
        /// has the value double.
        /// The value of the style:border-line-width-bottom attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specifies the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// style:graphic-properties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:page-layout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:table-cell-properties 17.18.
        pub fn set_border_line_width_bottom(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc.set_attr(
                "style:border-line-width-bottom",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-left attribute specifies the widths of the left border for
        /// borders defined by the FO border properties (see 20.183) if the property for the left border has the
        /// value double.
        /// The value of the style:border-line-width-left attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// style:graphic-properties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:page-layout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:table-cell-properties 17.18.
        pub fn set_border_line_width_left(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc.set_attr(
                "style:border-line-width-left",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-right attribute specifies the widths of the right border for
        /// borders defined by the FO border properties (see 20.183) if the property for the right border has
        /// the value double.
        /// The value of the style:border-line-width-right attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// style:graphic-properties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:page-layout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:table-cell-properties 17.18.
        pub fn set_border_line_width_right(
            &mut self,
            inner: Length,
            spacing: Length,
            outer: Length,
        ) {
            self.$acc.set_attr(
                "style:border-line-width-right",
                border_line_width_string(inner, spacing, outer),
            );
        }

        /// The style:border-line-width-top attribute specifies the widths of the top border for
        /// borders defined by the FO border properties (see 20.183) if the property for the top border has the
        /// value double.
        /// The value of the style:border-line-width-top attribute is a list of three white spaceseparated lengths, as follows:
        /// * The first value specifies the width of the inner line
        /// * The second value specified the distance between the two lines
        /// * The third value specifies the width of the outer line
        ///
        /// The style:border-line-width attribute is usable with the following elements:
        /// style:graphic-properties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:page-layout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:table-cell-properties 17.18.
        pub fn set_border_line_width_top(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc.set_attr(
                "style:border-line-width-top",
                border_line_width_string(inner, spacing, outer),
            );
        }
    };
}

macro_rules! fo_break {
    ($acc:ident) => {
        /// See §7.19.2 of XSL. The values odd-page and even-page are not supported.
        /// This attribute shall not be used at the same time as fo:break-after.
        /// In the OpenDocument XSL-compatible namespace, the fo:break-before attribute does not
        /// support even-page, inherit and odd-page values.
        pub fn set_break_before(&mut self, pagebreak: PageBreak) {
            self.$acc.set_attr("fo:break-before", pagebreak.to_string());
        }

        /// See §7.19.1 of XSL. The values odd-page and even-page are not supported.
        /// This attribute shall not be used at the same time as fo:break-before.
        /// In the OpenDocument XSL-compatible namespace, the fo:break-after attribute does not
        /// support even-page, inherit and odd-page values.
        pub fn set_break_after(&mut self, pagebreak: PageBreak) {
            self.$acc.set_attr("fo:break-after", pagebreak.to_string());
        }
    };
}

macro_rules! fo_hyphenation {
    ($acc:ident) => {
        /// See §7.15.1 of XSL.
        pub fn set_hyphenation_keep(&mut self, hyphenation: Hyphenation) {
            self.$acc
                .set_attr("fo:hyphenation-keep", hyphenation.to_string());
        }

        /// See §7.15.2 of XSL.
        /// The defined values for the fo:hyphenation-ladder-count attribute are:
        /// * no-limit:
        /// * a value of type positiveInteger
        pub fn set_hyphenation_ladder_count(&mut self, hyphenation: HyphenationLadderCount) {
            self.$acc
                .set_attr("fo:hyphenation-ladder-count", hyphenation.to_string());
        }
    };
}

macro_rules! fo_keep_together {
    ($acc:ident) => {
        /// See §7.19.3 of XSL.
        /// In the OpenDocument XSL-compatible namespace, the fo:keep-together attribute does not
        /// support the integer value.
        ///
        /// The fo:keep-together attribute is usable with the following elements:
        /// style:paragraphproperties 17.6 and
        /// style:table-row-properties 17.17.
        pub fn set_keep_together(&mut self, keep_together: TextKeep) {
            self.$acc
                .set_attr("fo:keep-together", keep_together.to_string());
        }
    };
}

macro_rules! fo_keep_with_next {
    ($acc:ident) => {
        /// See §7.19.4 of XSL.
        /// In the OpenDocument XSL-compatible namespace, the fo:keep-with-next attribute does not
        /// support the integer value.
        pub fn set_keep_with_next(&mut self, keep_with_next: TextKeep) {
            self.$acc
                .set_attr("fo:keep-with-next", keep_with_next.to_string());
        }
    };
}

macro_rules! fo_line_height {
    ($acc:ident) => {
        /// See §7.15.4 of XSL.
        /// The value normal activates the default line height calculation. The value of this attribute can be a
        /// length, a percentage, normal.
        ///
        /// In the OpenDocument XSL-compatible namespace, the fo:line-height attribute does not
        /// support the inherit, number, and space values.
        /// The defined values for the fo:line-height attribute are:
        /// * a value of type nonNegativeLength 18.3.20
        /// * normal: disables the effects of style:line-height-at-least 20.317 and
        /// style:line-spacing 20.318.
        /// * a value of type percent 18.3.23
        ///
        /// The fo:line-height attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        pub fn set_line_height(&mut self, line_height: LineHeight) {
            assert!(line_height.is_positive());
            self.$acc
                .set_attr("fo:line-height", line_height.to_string());
        }
    };
}

macro_rules! fo_margin {
    ($acc:ident) => {
        /// See §7.29.14 of XSL.
        /// In the OpenDocument XSL-compatible namespace, the fo:margin attribute does not support
        /// auto and inherit values.
        ///
        /// The fo:margin attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tableproperties 17.15.
        pub fn set_margin(&mut self, margin: Margin) {
            assert!(margin.is_positive());
            self.$acc.set_attr("fo:margin", margin.to_string());
        }

        /// See §7.10.2 of XSL.
        /// If this attribute is contained in a style:paragraph-properties 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// In the OpenDocument XSL-compatible namespace, the fo:margin-bottom attribute does not
        /// support the auto and inherit values.
        ///
        /// The fo:margin-bottom attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tableproperties 17.15.
        pub fn set_margin_bottom(&mut self, margin: Margin) {
            assert!(margin.is_positive());
            self.$acc.set_attr("fo:margin-bottom", margin.to_string());
        }

        /// See §7.10.3 of XSL.
        /// If this attribute is contained in a style:paragraph-properties 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// Tables that align to the left or to the center ignore right margins, and tables that align to the right
        /// or to the center ignore left margins.
        ///
        /// The fo:margin-left attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6,
        /// style:sectionproperties 17.11 and
        /// style:table-properties 17.15.
        pub fn set_margin_left(&mut self, margin: Margin) {
            assert!(margin.is_positive());
            self.$acc.set_attr("fo:margin-left", margin.to_string());
        }

        /// See §7.10.4 of XSL.
        /// If this attribute is contained in a style:paragraph-properties 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// Tables that align to the left or to the center ignore right margins, and tables that align to the right
        /// or to the center ignore left margins.
        ///
        /// The fo:margin-right attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6,
        /// style:sectionproperties 17.11 and
        /// style:table-properties 17.15.
        pub fn set_margin_right(&mut self, margin: Margin) {
            assert!(margin.is_positive());
            self.$acc.set_attr("fo:margin-right", margin.to_string());
        }

        /// See §7.10.1 of XSL.
        /// If this attribute is contained in a style:paragraph-properties 17.6 element, its value may
        /// be a percentage that refers to the corresponding margin of a parent style.
        /// In the OpenDocument XSL-compatible namespace, the fo:margin-top attribute does not
        /// support the inherit value.
        ///
        /// The fo:margin-top attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6 and
        /// style:tableproperties 17.15.
        pub fn set_margin_top(&mut self, margin: Margin) {
            assert!(margin.is_positive());
            self.$acc.set_attr("fo:margin-top", margin.to_string());
        }
    };
}

macro_rules! fo_orphans {
    ($acc:ident) => {
        /// See §7.19.6 of XSL.
        ///
        /// The fo:orphans attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        pub fn set_orphans(&mut self, orphans: u32) {
            self.$acc.set_attr("fo:orphans", orphans.to_string());
        }
    };
}

macro_rules! fo_text_align {
    ($acc:ident) => {
        /// See §7.15.9 of XSL.
        /// If there are no values specified for the fo:text-align and style:justify-single-word
        /// 20.301 attributes within the same formatting properties element, the values of those attributes is
        /// set to start and false respectively.
        ///
        /// In the OpenDocument XSL-compatible namespace, the fo:text-align attribute does not
        /// support the inherit, inside, outside, or string values.
        ///
        /// The fo:text-align attribute is usable with the following elements:
        /// style:list-levelproperties 17.19 and
        /// style:paragraph-properties 17.6.
        pub fn set_text_align(&mut self, align: TextAlign) {
            self.$acc.set_attr("fo:text-align", align.to_string());
        }
    };
}

macro_rules! fo_text_align_last {
    ($acc:ident) => {
        /// See §7.15.10 of XSL.
        /// This attribute is ignored if it not accompanied by an fo:text-align 20.223 attribute.
        /// If no value is specified for this attribute, the value is set to start.
        ///
        /// The fo:text-align-last attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_text_align_last(&mut self, align: TextAlignLast) {
            self.$acc.set_attr("fo:text-align-last", align.to_string());
        }
    };
}

macro_rules! fo_text_indent {
    ($acc:ident) => {
        /// The fo:text-indent attribute specifies a positive or negative indent for the first line of a
        /// paragraph. See §7.15.11 of XSL. The attribute value is a length. If the attribute is contained in a
        /// common style, the attribute value may be also a percentage that refers to the corresponding text
        /// indent of a parent style.
        ///
        /// The fo:text-indent attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        ///
        /// The values of the fo:text-indent attribute are a value of type length 18.3.18 or a value of
        /// type percent 18.3.23.
        pub fn set_text_indent(&mut self, indent: Indent) {
            self.$acc.set_attr("fo:text-indent", indent.to_string());
        }
    };
}

macro_rules! fo_widows {
    ($acc:ident) => {
        /// See §7.19.7 of XSL.
        /// The fo:widows attribute specifies the minimum number of lines that shall be displayed at the top
        /// of a page to avoid paragraph widows.
        /// In the OpenDocument XSL-compatible namespace, the fo:widows attribute does not support
        /// the inherit value.
        ///
        /// The fo:widows attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        ///
        /// The fo:widows attribute has the data type nonNegativeInteger 18.2
        pub fn set_widows(&mut self, num: u32) {
            self.$acc.set_attr("fo:widows", num.to_string());
        }
    };
}

macro_rules! fo_color {
    ($acc:ident) => {
        /// See §7.17.1 of XSL.
        /// In the OpenDocument XSL-compatible namespace, the fo:color attribute does not support the
        /// inherit value.
        pub fn set_color(&mut self, color: Rgb<u8>) {
            self.$acc.set_attr("fo:color", color_string(color));
        }
    };
}

macro_rules! fo_locale {
    ($acc:ident) => {
        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any UNICODE characters whose script type is latin.
        pub fn set_locale(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.$acc
                    .set_attr("fo:language", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.$acc.set_attr("fo:country", region.to_string());
                } else {
                    self.$acc.clear_attr("fo:country");
                }
                if let Some(script) = locale.id.script {
                    self.$acc.set_attr("fo:script", script.to_string());
                } else {
                    self.$acc.clear_attr("fo:script");
                }
            } else {
                self.$acc.clear_attr("fo:language");
                self.$acc.clear_attr("fo:country");
                self.$acc.clear_attr("fo:script");
            }
        }
    };
}

macro_rules! fo_font_size {
    ($acc:ident) => {
        /// See §7.8.4 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// XSL. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// UNICODE characters whose type is asian. The style:font-size-complex attribute (20.285)
        /// is evaluated for UNICODE characters whose type is complex.
        pub fn set_font_size(&mut self, size: FontSize) {
            assert!(size.is_positive());
            self.$acc.set_attr("fo:font-size", size.to_string());
        }
    };
}
macro_rules! fo_font_size_rel {
    ($acc:ident) => {
        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel(&mut self, size: FontSize) {
            self.$acc.set_attr("fo:font-size-rel", size.to_string());
        }
    };
}

macro_rules! fo_font_style {
    ($acc:ident) => {
        /// See §7.8.7 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
        pub fn set_font_style(&mut self, style: FontStyle) {
            self.$acc.set_attr("fo:font-style", style.to_string());
        }

        /// Set the font-style to italic.
        pub fn set_font_italic(&mut self) {
            self.$acc.set_attr("fo:font-style", "italic".to_string());
        }
    };
}

macro_rules! fo_font_weight {
    ($acc:ident) => {
        /// See §7.8.9 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
        pub fn set_font_weight(&mut self, weight: FontWeight) {
            self.$acc.set_attr("fo:font-weight", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold(&mut self) {
            self.$acc
                .set_attr("fo:font-weight", FontWeight::Bold.to_string());
        }
    };
}

macro_rules! fo_font_variant {
    ($acc:ident) => {
        /// See §7.8.8 of XSL.
        pub fn set_font_variant(&mut self, var: FontVariant) {
            self.$acc.set_attr("fo:font-variant", var.to_string());
        }
    };
}

macro_rules! fo_font_attr {
    ($acc:ident) => {
        /// Combined font attributes.
        pub fn set_font_attr(&mut self, size: FontSize, bold: bool, italic: bool) {
            self.set_font_size(size);
            if bold {
                self.set_font_bold();
            }
            if italic {
                self.set_font_italic();
            }
        }
    };
}

macro_rules! fo_hyphenate {
    ($acc:ident) => {
        /// See §7.9.4 of XSL.
        pub fn set_hyphenate(&mut self, hyphenate: bool) {
            self.$acc.set_attr("fo:hyphenate", hyphenate.to_string());
        }
    };
}

macro_rules! fo_hyphenation_push_char_count {
    ($acc:ident) => {
        /// See §7.10.6 of XSL
        pub fn set_hyphenation_push_char_count(&mut self, count: u32) {
            self.$acc
                .set_attr("fo:hyphenation-push-char-count", count.to_string());
        }
    };
}

macro_rules! fo_hyphenation_remain_char_count {
    ($acc:ident) => {
        /// See §7.10.7 of XSL
        pub fn set_hyphenation_remain_char_count(&mut self, count: u32) {
            self.$acc
                .set_attr("fo:hyphenation-remain-char-count", count.to_string());
        }
    };
}

macro_rules! fo_letter_spacing {
    ($acc:ident) => {
        /// See §7.16.2 of XSL.
        /// Sets the letter spacing.
        pub fn set_letter_spacing(&mut self, spacing: LetterSpacing) {
            self.$acc.set_attr("fo:letter-spacing", spacing.to_string());
        }
    };
}

macro_rules! fo_text_shadow {
    ($acc:ident) => {
        /// The fo:text-shadow attribute specifies the text shadow style to use.
        pub fn set_text_shadow(
            &mut self,
            x_offset: Length,
            y_offset: Length,
            blur: Option<Length>,
            color: Rgb<u8>,
        ) {
            self.$acc.set_attr(
                "fo:text-shadow",
                shadow_string(x_offset, y_offset, blur, color),
            );
        }
    };
}

macro_rules! fo_text_transform {
    ($acc:ident) => {
        /// See §7.16.6 of XSL.
        /// If fo:text-transform and fo:font-variant 20.192 attributes are used simultaneously and
        /// have different values than normal and none, the result is undefined.
        /// Note: In consumers, the fo:text-transform and fo:font-variant
        /// attributes are mutually exclusive
        pub fn set_text_transform(&mut self, trans: TextTransform) {
            self.$acc.set_attr("fo:text-transform", trans.to_string());
        }
    };
}

macro_rules! fo_min_height {
    ($acc:ident) => {
        /// Minimum height.
        pub fn set_min_height(&mut self, height: LengthPercent) {
            self.$acc.set_attr("fo:min-height", height.to_string());
        }
    };
}

macro_rules! fo_page_height {
    ($acc:ident) => {
        /// Page Height
        pub fn set_page_height(&mut self, height: Length) {
            self.style_mut()
                .set_attr("fo:page-height", height.to_string());
        }
    };
}

macro_rules! fo_page_width {
    ($acc:ident) => {
        /// Page Width
        pub fn set_page_width(&mut self, width: Length) {
            self.style_mut()
                .set_attr("fo:page-width", width.to_string());
        }
    };
}
