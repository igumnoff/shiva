macro_rules! style_default_outline_level {
    ($acc:ident) => {
        /// The style:default-outline-level attribute specifies a default outline level for a style with
        /// the style:family 19.480 attribute value paragraph.
        ///
        /// If the style:default-outline-level attribute is present in a paragraph style, and if this
        /// paragraph style is assigned to a paragraph or heading by user action, then the consumer should
        /// replace the paragraph or heading with a heading of the specified level, which has the same
        /// content and attributes as the original paragraph or heading.
        ///
        /// Note: This attribute does not modify the behavior of <text:p> 5.1.3 or
        /// <text:h> 5.1.2 elements, but only instructs a consumer to create one or the
        /// other when assigning a paragraph style as a result of user interface action while
        /// the document is edited.
        ///
        /// The style:default-outline-level attribute value can be empty. If empty, this attribute
        /// does not inherit a list style value from a parent style.
        pub fn set_default_outline_level(&mut self, level: u32) {
            self.$acc
                .set_attr("style:default-outline-level", level.to_string());
        }
    };
}

macro_rules! style_master_page {
    ($acc:ident) => {
        /// The style:master-page-name attribute defines a master page for a paragraph or table style.
        /// This applies to automatic and common styles.
        ///
        /// If this attribute is associated with a style, a page break is inserted when the style is applied and
        /// the specified master page is applied to the resulting page.
        ///
        /// This attribute is ignored if it is associated with a paragraph style that is applied to a paragraph
        /// within a table.
        pub fn set_master_page(&mut self, masterpage: &MasterPageRef) {
            self.$acc
                .set_attr("style:master-page-name", masterpage.as_str().to_string());
        }
    };
}

macro_rules! style_next_style {
    ($acc:ident) => {
        /// Within styles for paragraphs, style:next-style-name attribute specifies the style to be used
        /// for the next paragraph if a paragraph break is inserted in the user interface. By default, the current
        /// style is used as the next style.
        pub fn set_next_style(&mut self, name: &ParagraphStyleRef) {
            self.$acc
                .set_attr("style:next-style-name", name.as_str().to_string());
        }
    };
}

macro_rules! style_cell_protect {
    ($acc:ident) => {
        /// The style:cell-protect attribute specifies how a cell is protected.
        ///
        /// This attribute is only evaluated if the current table is protected.
        ///
        /// The defined values for the style:cell-protect attribute are:
        /// * formula-hidden: if cell content is a formula, it is not displayed. It can be replaced by
        /// changing the cell content.
        /// Note: Replacement of cell content includes replacement with another formula or
        /// other cell content.
        /// * hidden-and-protected: cell content is not displayed and cannot be edited. If content is a
        /// formula, the formula result is not displayed.
        /// * none: formula responsible for cell content is neither hidden nor protected.
        /// * protected: cell content cannot be edited.
        /// * protected formula-hidden: cell content cannot be edited. If content is a formula, it is not
        /// displayed. A formula result is displayed.
        pub fn set_cell_protect(&mut self, protect: CellProtect) {
            self.$acc
                .set_attr("style:cell-protect", protect.to_string());
        }
    };
}

macro_rules! style_decimal_places {
    ($acc:ident) => {
        // style:decimal-places 20.258,
        /// The style:decimal-places attribute specifies the maximum number of decimal places that
        /// are displayed if numbers are formatted by a data style that has no setting for number of decimal
        /// places itself.
        /// This attribute is only evaluated if it is contained in a default style.
        pub fn set_decimal_places(&mut self, dec: u8) {
            self.$acc.set_attr("style:decimal-places", dec.to_string());
        }
    };
}

macro_rules! style_diagonal {
    ($acc:ident) => {
        // style:diagonal-bl-tr 20.259,
        /// The style:diagonal-bl-tr attribute specifies the style of border to use for a bottom-left to
        /// top-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_bl_tr(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("style:diagonal-bl-tr", border_string(width, border, color));
        }

        // style:diagonal-bl-tr-widths 20.260,
        /// The style:diagonal-bl-tr-widths attribute specifies the width between a double line
        /// border to use for a bottom-left to top-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_bl_tr_widths(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc.set_attr(
                "style:diagonal-bl-tr-widths",
                border_line_width_string(inner, spacing, outer),
            );
        }

        // style:diagonal-tl-br 20.261,
        /// The style:diagonal-tl-br attribute specifies the style of border to use for a left-top to
        /// bottom-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_tl_br(&mut self, width: Length, border: Border, color: Rgb<u8>) {
            self.$acc
                .set_attr("style:diagonal-tl-br", border_string(width, border, color));
        }

        // style:diagonal-tl-br-widths 20.262,
        /// The style:diagonal-tl-br-widths attribute specifies the width between a double line
        /// border to use for a top-left to bottom-right diagonal in a spreadsheet cell.
        pub fn set_diagonal_tl_br_widths(&mut self, inner: Length, spacing: Length, outer: Length) {
            self.$acc.set_attr(
                "style:diagonal-tl-br-widths",
                border_line_width_string(inner, spacing, outer),
            );
        }
    };
}

macro_rules! style_direction {
    ($acc:ident) => {
        /// The style:direction attribute specifies the direction of characters.
        /// The style:direction attribute modifies the direction of text rendering as specified by a
        /// style:writing-mode attribute. 20.404
        ///
        /// The defined values for the style:direction attribute are:
        /// * ltr – left to right, text is rendered in the direction specified by the style:writing-mode
        /// attribute
        /// * ttb – top to bottom, characters are vertically stacked but not rotated
        pub fn set_direction(&mut self, direction: WritingDirection) {
            self.$acc.set_attr("style:direction", direction.to_string());
        }
    };
}

macro_rules! style_glyph_orientation_vertical {
    ($acc:ident) => {
        /// The style:glyph-orientation-vertical attribute specifies a vertical glyph orientation.
        /// See §10.7.3 of SVG. The attribute specifies an angle or automatic mode. The only defined angle
        /// is 0 degrees, which disables this feature.
        ///
        /// Note: OpenDocument v1.1 did not support angle specifications that contain an
        /// angle unit identifier. Angle unit identifiers should be omitted for compatibility with
        /// OpenDocument v1.1.
        pub fn set_glyph_orientation_vertical(&mut self, glyph_orientation: GlyphOrientation) {
            self.$acc.set_attr(
                "style:glyph-orientation-vertical",
                glyph_orientation.to_string(),
            );
        }
    };
}

macro_rules! style_print_content {
    ($acc:ident) => {
        // style:print-content 20.331,
        /// The style:print-content attribute specifies if content is printed.
        /// The style:print-content attribute specifies if cell content is printed.
        /// The style:print-content attribute is usable with the following element:
        /// * style:tablecell-properties 17.18.
        pub fn set_print_content(&mut self, print: bool) {
            self.$acc.set_attr("style:print-content", print.to_string());
        }
    };
}

macro_rules! style_repeat_content {
    ($acc:ident) => {
        // style:repeat-content 20.342,
        /// The style:repeat-content attribute specifies whether text content of a cell is displayed as
        /// many times as there is space left in the cell's writing direction. The attribute has no effect for cell
        /// content that contains a line break.
        /// The defined values for the style:repeat-content attribute are:
        /// * false: text content of a cell should not be displayed as many times as there is space left in
        /// the cell's writing direction.
        /// * true: text content of a cell should be displayed as many times as there is space left in the
        /// cell's writing direction.
        pub fn set_repeat_content(&mut self, print: bool) {
            self.$acc
                .set_attr("style:repeat-content", print.to_string());
        }
    };
}

macro_rules! style_rotation_align {
    ($acc:ident) => {
        // style:rotationalign 20.346,
        /// The style:rotation-align attribute specifies how the edge of the text in a cell is aligned
        /// after a rotation.
        /// The values of the style:rotation-align attribute are none, bottom, top or center.
        pub fn set_rotation_align(&mut self, align: RotationAlign) {
            self.$acc
                .set_attr("style:rotation-align", align.to_string());
        }
    };
}

macro_rules! style_rotation_scale {
    ($acc:ident) => {
        // style:text-scale 20.387,
        /// The style:text-rotation-scale attribute specifies whether for rotated text the width of the
        /// text should be scaled to fit into the current line height or the width of the text should remain fixed,
        /// therefore changing the current line height.
        /// The defined values for the style:text-rotation-scale attribute are:
        /// * fixed: width of text should remain fixed.
        /// * line-height: width of text should be scaled to fit the current line height.
        pub fn set_rotation_scale(&mut self, scale: RotationScale) {
            self.$acc
                .set_attr("style:text-rotation-scale", scale.to_string());
        }
    };
}

macro_rules! style_rotation_angle {
    ($acc:ident) => {
        // style:rotation-angle 20.347,
        /// The style:rotation-angle attribute specifies the rotation angle of content.
        pub fn set_rotation_angle(&mut self, angle: Angle) {
            self.$acc
                .set_attr("style:rotation-angle", angle.to_string());
        }
    };
}

macro_rules! style_shadow {
    ($acc:ident) => {
        /// The style:shadow attribute specifies a shadow effect.
        /// The defined values for this attribute are those defined in §7.16.5 of XSL, except the value
        /// inherit.
        ///
        /// The shadow effect is not applied to the text content of an element, but depending on the element
        /// where the attribute appears, to a paragraph, a text box, a page body, a header, a footer, a table or
        /// a table cell.
        ///
        /// The style:shadow attribute is usable with the following elements:
        /// style:graphicproperties 17.21,
        /// style:header-footer-properties 17.5,
        /// style:pagelayout-properties 17.2,
        /// style:paragraph-properties 17.6,
        /// style:tablecell-properties 17.18 and
        /// style:table-properties 17.15.
        pub fn set_shadow(
            &mut self,
            x_offset: Length,
            y_offset: Length,
            blur: Option<Length>,
            color: Rgb<u8>,
        ) {
            self.$acc.set_attr(
                "style:shadow",
                shadow_string(x_offset, y_offset, blur, color),
            );
        }
    };
}

macro_rules! style_shrink_to_fit {
    ($acc:ident) => {
        // style:shrinkto-fit 20.360,
        /// The style:shrink-to-fit attribute specifies whether content is reduced in size to fit within a
        /// cell or drawing object. Shrinking means that the font size of the content is decreased to fit the
        /// content into a cell or drawing object. The attribute has no effect on cells where the cell content
        /// already fits into the cell.
        ///
        /// The defined values for the style:shrink-to-fit attribute are:
        /// * false: content should not be reduced in size to fit within a cell or drawing object.
        /// * true: content should be reduced in size to fit within a cell or drawing object
        pub fn set_shrink_to_fit(&mut self, shrink: bool) {
            self.$acc
                .set_attr("style:shrink-to-fit", shrink.to_string());
        }
    };
}

macro_rules! style_text_align_source {
    ($acc:ident) => {
        // style:text-align-source 20.364,
        /// The style:text-align-source attribute specifies the source of a text-align attribute.
        /// The defined values for the style:text-align-source attribute are:
        /// * fix: content alignment uses the value of the fo:text-align 20.223 attribute.
        /// * value-type: content alignment uses the value-type of the cell.
        ///
        /// The default alignment for a cell value-type string is left, for other value-types it is right.
        pub fn set_text_align_source(&mut self, align: TextAlignSource) {
            self.cellstyle
                .set_attr("style:text-align-source", align.to_string());
        }
    };
}

macro_rules! style_vertical_align {
    ($acc:ident) => {
        // style:vertical-align 20.396,
        /// The style:vertical-align attribute specifies the vertical position of a character. By default
        /// characters are aligned according to their baseline.
        ///
        /// The defined values for the style:vertical-align attribute are:
        /// * auto: automatically, which sets the vertical alignment to suit the text rotation. Text that is
        /// rotated 0 or 90 degrees is aligned to the baseline, while text that is rotated 270 degrees is
        /// aligned to the center of the line.
        /// * baseline: to the baseline of the character.
        /// * bottom: to the bottom of the line.
        /// * middle: to the center of the line.
        /// * top: to the top of the line.
        pub fn set_vertical_align(&mut self, align: CellAlignVertical) {
            self.cellstyle
                .set_attr("style:vertical-align", align.to_string());
        }
    };
}

macro_rules! style_auto_text_indent {
    ($acc:ident) => {
        /// The style:auto-text-indent attribute specifies that the first line of a paragraph is indented
        /// by a value that is based on the current font size.
        /// If this attribute has a value of true and is used together with a fo:text-indent 20.225
        /// attribute the fo:text-indent attribute is ignored.
        ///
        /// The style:auto-text-indent attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        ///
        /// The style:auto-text-indent attribute has the data type boolean 18.3.3.
        pub fn set_auto_text_indent(&mut self, indent: bool) {
            self.$acc
                .set_attr("style:auto-text-indent", indent.to_string());
        }
    };
}

macro_rules! style_background_transparency {
    ($acc:ident) => {
        /// The style:background-transparency attribute specifies the transparency of a paragraph's
        /// background color.
        /// The style:background-transparency attribute is usable with the following elements:
        /// style:graphic-properties> 17.21 and <style:paragraph-properties 17.6.
        pub fn set_background_transpareny(&mut self, percent: Percent) {
            self.$acc
                .set_attr("style:background-transparency", percent.to_string());
        }
    };
}

macro_rules! style_contextual_spacing {
    ($acc:ident) => {
        /// The fo:margin-bottom 20.206 attribute of a paragraph and the fo:margin-top 20.209
        /// attribute of the next paragraph are ignored, so that the space between the paragraphs is zero,
        /// if all of the following conditions hold:
        /// * The style:contextual-spacing attribute of both paragraphs has the value true.
        /// * The paragraphs belong to the same content area.
        /// * The text:style-name 19.880 attribute of the paragraphs refer to the same common
        /// paragraph style. In case a text:style-name attribute refers to an automatic style, the value
        /// of the style:parent-style-name 19.510 attribute of the automatic style is taken for the
        /// style comparison. If a paragraph has a conditional style, the value of its text:cond-stylename 19.781 attribute is taken for the style comparison.
        /// The default value for this attribute is false.
        ///
        /// The style:contextual-spacing attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_contextual_spacing(&mut self, spacing: bool) {
            self.$acc
                .set_attr("style:contextual-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_font_independent_line_spacing {
    ($acc:ident) => {
        /// The style:font-independent-line-spacing attribute specifies if font independent line
        /// spacing is used.
        ///
        /// The defined values for the style:font-independent-line-spacing attribute are:
        /// * false: font metric of the font is taken into account.
        /// * true: line height is calculated only from the font height as specified by the font size attributes
        /// fo:font-size 20.190, style:font-size-asian 20.284 and style:font-sizecomplex 20.285.
        ///
        /// The style:font-independent-line-spacing attribute is usable with the following
        /// element: style:paragraph-properties 17.6.
        pub fn set_font_independent_line_spacing(&mut self, spacing: bool) {
            self.$acc
                .set_attr("style:font-independent-line-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_join_border {
    ($acc:ident) => {
        /// The style:join-border property specifies whether a border for one paragraph is to be
        /// extended around the following paragraph.
        ///
        /// In addition to the value of this attribute, joining of borders requires meeting these conditions:
        /// 1) Values of attributes fo:border-top 20.183.6, fo:border-bottom 20.183.3,
        /// fo:border-left 20.183.4 and fo:border-right 20.183.5 are the same. These values
        /// can also be given by the fo:border 20.183.2 attribute.
        /// 2) Values of attributes style:border-line-width-top 20.252, style:border-linewidth-bottom 20.249, style:border-line-width-left 20.250 and
        /// style:border-line-width-right 20.251 are the same. These values can also be given
        /// by the style:border-line-width 20.248 attribute.
        /// 3) Values of attributes fo:padding-left 20.219 and fo:padding-right 20.220 are the
        /// same. These values can also be given by the fo:padding 20.217 attribute.
        /// 4) Values of the fo:margin-right 20.208 attributes are the same. These values can also be
        /// given by the fo:margin 20.205 attribute.
        /// 5) Values of the fo:margin-left 20.207 attribute, which can also be given by the fo:margin,
        /// and fo:text-indent 19.246 attributes, that meet one of these conditions:
        /// 1. All values are the same.
        /// 2. Values of the fo:margin-left attributes are the same and values of the fo:textindent attributes are non-negative.
        /// 3. Value of the fo:margin-left attribute of one paragraph whose value of the fo:textindent attribute is non-negative is the same as the sum of values of the fo:marginleft and fo:text-indent attributes of the other paragraph whose value of the
        /// fo:text-indent attribute is negative.
        /// 4. Both values of the fo:text-indent attributes are negative and the sums of values of the
        /// fo:margin-left and fo:text-indent attributes are equal.
        ///
        /// The default value of this attribute is true.
        ///
        /// The defined values for the style:join-border attribute are:
        /// * false: borders should not be joined.
        /// * true: borders should be joined.
        ///
        /// The style:join-border attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        pub fn set_join_border(&mut self, join: bool) {
            self.$acc.set_attr("style:join-border", join.to_string());
        }
    };
}

macro_rules! style_justify_single_word {
    ($acc:ident) => {
        /// The style:justify-single-word attribute specifies whether a single word should be justified
        /// when the last line in a paragraph is justified.
        /// Specifying a style:justify-single-word attribute without specifying a fo:text-align
        /// 20.223 and fo:text-align-last 20.224 attribute has no effect. Unspecified, both
        /// fo:textalign and fo:text-align-last have the value start.
        ///
        /// The defined values for the style:justify-single-word attribute are:
        /// * false: single word should not be justified when the last line in a paragraph is justified.
        /// * true: single word should be justified when last line in a paragraph is justified.
        ///
        /// The style:justify-single-word attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_justify_single_word(&mut self, justify: bool) {
            self.$acc
                .set_attr("style:justify-single-word", justify.to_string());
        }
    };
}

macro_rules! style_line_break {
    ($acc:ident) => {
        /// The style:line-break attribute specifies line breaking rules.
        /// The defined values for the style:line-break attribute are:
        /// * normal: line breaks may occur between any characters.
        /// * strict: line breaks shall not occur before or after implementation-defined characters.
        ///
        /// The style:line-break attribute is usable with the following element: style:paragraphproperties 17.6.
        ///
        /// The values of the style:line-break attribute are normal or strict.
        pub fn set_line_break(&mut self, linebreak: LineBreak) {
            self.$acc
                .set_attr("style:line-break", linebreak.to_string());
        }
    };
}

macro_rules! style_line_height_at_least {
    ($acc:ident) => {
        /// The style:line-height-at-least attribute specifies a minimum line height. The value of
        /// this attribute is a length.
        /// The effect of this attribute is disabled when fo:line-height 20.204 has the value of normal.
        ///
        /// The style:line-height-at-least attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_line_height_at_least(&mut self, height: Length) {
            assert!(height.is_positive());
            self.$acc
                .set_attr("style:line-height-at-least", height.to_string());
        }
    };
}

macro_rules! style_line_spacing {
    ($acc:ident) => {
        /// The style:line-spacing attribute specifies a fixed distance between two lines.
        /// The effect of this attribute is disabled when fo:line-height 20.204 has the value of normal.
        ///
        /// The style:line-spacing attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_line_spacing(&mut self, spacing: Length) {
            self.$acc
                .set_attr("style:line-spacing", spacing.to_string());
        }
    };
}

macro_rules! style_page_number {
    ($acc:ident) => {
        /// The style:page-number attribute specifies the page number that should be used for a new
        /// page when either a paragraph or table style specifies a master page that should be applied
        /// beginning from the start of a paragraph or table.
        ///
        /// The defined values for the style:page-number attribute are:
        /// * auto: a page has the page number of the previous page, incremented by one.
        /// * a value of type nonNegativeInteger 18.2: specifies a page number.
        ///
        /// The style:page-number attribute is usable with the following elements:
        /// style:paragraph-properties 17.6 and
        /// style:table-properties 17.15.
        ///
        /// The values of the style:page-number attribute are a value of type nonNegativeInteger
        /// 18.2 or auto.
        pub fn set_page_number(&mut self, page_number: PageNumber) {
            self.$acc
                .set_attr("style:page-number", page_number.to_string());
        }
    };
}

macro_rules! style_punctuation_wrap {
    ($acc:ident) => {
        /// The style:punctuation-wrap attribute specifies whether a punctuation mark, if one is
        /// present, can be hanging, that is, whether it can placed in the margin area at the end of a full line of
        /// text.
        ///
        /// The defined values for the style:punctuation-wrap attribute are:
        /// * hanging: a punctuation mark can be placed in the margin area at the end of a full line of text.
        /// * simple: a punctuation mark cannot be placed in the margin area at the end of a full line of
        /// text.
        ///
        /// The style:punctuation-wrap attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_punctuation_wrap(&mut self, wrap: PunctuationWrap) {
            self.$acc
                .set_attr("style:punctuation-wrap", wrap.to_string());
        }
    };
}

macro_rules! style_register_true {
    ($acc:ident) => {
        /// The style:register-true attribute specifies whether the lines on both sides of a printed page
        /// align. The text baselines of text in page columns or text box columns also align.
        /// The defined values for the style:register-true attribute are:
        /// * false: lines on both sides of a printed text need not align.
        /// * true: lines on both sides of a printed text should align.
        ///
        /// The style:register-true attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        ///
        /// The style:register-true attribute has the data type boolean 18.3.3
        pub fn set_register_true(&mut self, register: bool) {
            self.$acc
                .set_attr("style:register-true", register.to_string());
        }
    };
}

macro_rules! style_snap_to_layout_grid {
    ($acc:ident) => {
        /// The style:snap-to-layout-grid attribute specifies whether the layout of a paragraph
        /// should consider the layout grid settings of the page where it appears.
        ///
        /// The defined values for the style:snap-to-layout-grid attribute are:
        /// * false: layout of a paragraph should not consider the layout grid settings of the page where it
        /// appears.
        /// * true: layout of a paragraph should consider the layout grid settings of the page where it
        /// appears.
        ///
        /// The style:snap-to-layout-grid attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_snap_to_layout_grid(&mut self, snap: bool) {
            self.$acc
                .set_attr("style:snap-to-layout-grid", snap.to_string());
        }
    };
}

macro_rules! style_tab_stop_distance {
    ($acc:ident) => {
        /// The style:tab-stop-distance attribute specifies the distance between default tab stops. A
        /// default tab stop is repeated automatically after the specified distance. Default tab stops are only
        /// evaluated if they are specified within a default style.
        ///
        /// The style:tab-stop-distance attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_tab_stop_distance(&mut self, tab: Length) {
            assert!(tab.is_positive());
            self.$acc
                .set_attr("style:tab-stop-distance", tab.to_string());
        }
    };
}

macro_rules! style_text_autospace {
    ($acc:ident) => {
        /// The style:text-autospace attribute specifies whether to add space between portions of
        /// Asian, Western, and complex texts.
        ///
        /// The defined values for the style:text-autospace attribute are:
        /// * ideograph-alpha: space should be added between portions of Asian, Western and
        /// complex texts.
        /// * none: space should not be added between portions of Asian, Western and complex texts.
        ///
        /// The style:text-autospace attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_text_autospace(&mut self, space: TextAutoSpace) {
            self.$acc
                .set_attr("style:text-autospace", space.to_string());
        }
    };
}

macro_rules! style_vertical_align_para {
    ($acc:ident) => {
        /// The style:vertical-align attribute specifies the vertical position of a character. By default
        /// characters are aligned according to their baseline.
        ///
        /// The defined values for the style:vertical-align attribute are:
        /// * auto: automatically, which sets the vertical alignment to suit the text rotation. Text that is
        /// rotated 0 or 90 degrees is aligned to the baseline, while text that is rotated 270 degrees is
        /// aligned to the center of the line.
        /// * baseline: to the baseline of the character.
        /// * bottom: to the bottom of the line.
        /// * middle: to the center of the line.
        /// * top: to the top of the line.
        ///
        /// The style:vertical-align attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_vertical_align_para(&mut self, align: ParaAlignVertical) {
            self.$acc
                .set_attr("style:vertical-align", align.to_string());
        }
    };
}

macro_rules! style_writing_mode {
    ($acc:ident) => {
        /// See §7.27.7 of XSL.
        ///
        /// The defined value for the style:writing-mode attribute is page: writing mode is inherited from
        /// the page that contains the element where this attribute appears.
        pub fn set_writing_mode(&mut self, writing_mode: WritingMode) {
            self.$acc
                .set_attr("style:writing-mode", writing_mode.to_string());
        }
    };
}

macro_rules! style_writing_mode_automatic {
    ($acc:ident) => {
        /// The style:writing-mode-automatic attribute specifies whether a consumer may
        /// recalculate the writing mode of a paragraph based on its content whenever the content is edited.
        ///
        /// The writing-mode should be specified in a style:writing-mode attribute.
        ///
        /// If the fo:text-align with value start, text alignment can be adapted to the writing mode.
        /// The defined values for the style:writing-mode-automatic attribute are:
        /// * false: consumers should not recalculate writing mode of a paragraph whenever its content
        /// is edited.
        /// * true: consumers should recalculate writing mode of a paragraph whenever its content is
        /// edited.
        ///
        /// The style:writing-mode-automatic attribute is usable with the following element:
        /// style:paragraph-properties 17.6.
        pub fn set_writing_mode_automatic(&mut self, auto: bool) {
            self.$acc
                .set_attr("style:writing-mode-automatic", auto.to_string());
        }
    };
}

macro_rules! style_line_number {
    ($acc:ident) => {
        /// The text:line-number attribute specifies a new start value for line numbering, if a
        /// text:number-lines 20.434 attribute, with the value true, appears on the same
        /// style:paragraph-properties 17.6 element. Otherwise, this attribute shall be ignored.
        ///
        /// The text:line-number attribute is usable with the following element: style:paragraphproperties 17.6.
        /// The text:line-number attribute has the data type nonNegativeInteger 18.2.
        pub fn set_line_number(&mut self, line: u32) {
            self.$acc.set_attr("text:line-number", line.to_string());
        }
    };
}

macro_rules! style_number_lines {
    ($acc:ident) => {
        /// The text:number-lines attribute specifies whether lines are numbered.
        /// The defined values for the text:number-lines attribute are:
        /// * false: lines should not be numbered.
        /// * true: lines should be numbered.
        ///
        /// The text:number-lines attribute is usable with the following element:
        /// style:paragraphproperties 17.6.
        pub fn set_number_lines(&mut self, lines: bool) {
            self.$acc.set_attr("text:number-lines", lines.to_string());
        }
    };
}

macro_rules! style_font_name {
    ($acc:ident) => {
        // LATIN

        /// The style:font-name attribute specifies a font that is declared by a style:font-face
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
        pub fn set_font_name<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("style:font-name", name.into());
        }
    };
}

macro_rules! style_locale_asian {
    ($acc:ident) => {
        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any UNICODE characters whose script type is asian.
        pub fn set_locale_asian(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.$acc
                    .set_attr("style:language-asian", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.$acc
                        .set_attr("style:country-asian", region.to_string());
                } else {
                    self.$acc.clear_attr("style:country-asian");
                }
                if let Some(script) = locale.id.script {
                    self.$acc.set_attr("style:script-asian", script.to_string());
                } else {
                    self.$acc.clear_attr("style:script-asian");
                }
            } else {
                self.$acc.clear_attr("style:language-asian");
                self.$acc.clear_attr("style:country-asian");
                self.$acc.clear_attr("style:script-asian");
            }
        }
    };
}

macro_rules! style_font_name_asian {
    ($acc:ident) => {
        /// The style:font-name attribute specifies a font that is declared by a style:font-face
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any UNICODE character whose script type is asian. 20.358
        pub fn set_font_name_asian<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("style:font-name-asian", name.into());
        }
    };
}

macro_rules! style_font_size_asian {
    ($acc:ident) => {
        /// See §7.8.4 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is asian. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// XSL. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// UNICODE characters whose type is asian. The style:font-sizecomplex attribute (20.285) is evaluated for UNICODE characters whose type is
        /// complex.
        ///
        pub fn set_font_size_asian(&mut self, size: FontSize) {
            assert!(size.is_positive());
            self.$acc
                .set_attr("style:font-size-asian", size.to_string());
        }
    };
}

macro_rules! style_font_size_rel_asian {
    ($acc:ident) => {
        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any UNICODE character whose script type is asian. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel_asian(&mut self, size: FontSize) {
            self.$acc
                .set_attr("style:font-size-rel-asian", size.to_string());
        }
    };
}

macro_rules! style_font_style_asian {
    ($acc:ident) => {
        /// See §7.8.7 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is asian. 20.358
        pub fn set_font_style_asian(&mut self, style: FontStyle) {
            self.$acc
                .set_attr("style:font-style-asian", style.to_string());
        }

        /// Set the font-style to italic.
        pub fn set_font_italic_asian(&mut self) {
            self.$acc
                .set_attr("style:font-style-asian", "italic".to_string());
        }
    };
}

macro_rules! style_font_weight_asian {
    ($acc:ident) => {
        /// See §7.8.9 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is asian. 20.358
        pub fn set_font_weight_asian(&mut self, weight: FontWeight) {
            self.$acc
                .set_attr("style:font-weight-asian", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold_asian(&mut self) {
            self.$acc
                .set_attr("style:font-weight-asian", FontWeight::Bold.to_string());
        }
    };
}

macro_rules! style_font_attr_asian {
    ($acc:ident) => {
        /// Combined font attributes.
        pub fn set_font_attr_asian(&mut self, size: FontSize, bold: bool, italic: bool) {
            self.set_font_size_asian(size);
            if bold {
                self.set_font_bold_asian();
            }
            if italic {
                self.set_font_italic_asian();
            }
        }
    };
}

macro_rules! style_locale_complex {
    ($acc:ident) => {
        /// Sets the attributes for fo:language, fo:country and fo:script
        /// to the given locale.
        ///
        /// These attributes are evaluated for any UNICODE characters whose script type is complex.
        pub fn set_locale_complex(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.$acc
                    .set_attr("style:language-complex", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.$acc
                        .set_attr("style:country-complex", region.to_string());
                } else {
                    self.$acc.clear_attr("style:country-complex");
                }
                if let Some(script) = locale.id.script {
                    self.$acc
                        .set_attr("style:script-complex", script.to_string());
                } else {
                    self.$acc.clear_attr("style:script-complex");
                }
            } else {
                self.$acc.clear_attr("style:language-complex");
                self.$acc.clear_attr("style:country-complex");
                self.$acc.clear_attr("style:script-complex");
            }
        }
    };
}

macro_rules! style_font_name_complex {
    ($acc:ident) => {
        /// The style:font-name attribute specifies a font that is declared by a style:font-face
        /// 16.23 element with a style:name 19.502 attribute whose name is the same as that of the
        /// style:font-name attribute value.
        ///
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        pub fn set_font_name_complex<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("style:font-name-complex", name.into());
        }
    };
}

macro_rules! style_font_size_complex {
    ($acc:ident) => {
        /// See §7.8.4 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        /// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
        /// XSL. In contrast to XSL, percentage values can be used within common styles only and are
        /// based on the font height of the parent style rather than to the font height of the attributes
        /// neighborhood. Absolute font heights and relative font heights are not supported.
        ///
        /// Note: The style:font-size-asian attribute (20.284) is evaluated for
        /// UNICODE characters whose type is asian. The style:font-sizecomplex attribute (20.285) is evaluated for UNICODE characters whose type is
        /// complex.
        ///
        pub fn set_font_size_complex(&mut self, size: FontSize) {
            assert!(size.is_positive());
            self.$acc
                .set_attr("style:font-size-complex", size.to_string());
        }
    };
}
macro_rules! style_font_size_rel_complex {
    ($acc:ident) => {
        /// The style:font-size-rel attribute specifies a relative font size change.
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        /// This attribute specifies a relative font size change as a length. It cannot be used within automatic
        /// styles. This attribute changes the font size based on the font size of the parent style.
        pub fn set_font_size_rel_complex(&mut self, size: FontSize) {
            self.$acc
                .set_attr("style:font-size-rel-complex", size.to_string());
        }
    };
}
macro_rules! style_font_style_complex {
    ($acc:ident) => {
        /// See §7.8.7 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        pub fn set_font_style_complex(&mut self, style: FontStyle) {
            self.$acc
                .set_attr("style:font-style-complex", style.to_string());
        }

        /// Set the font-style to italic.
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        pub fn set_font_italic_complex(&mut self) {
            self.$acc
                .set_attr("style:font-style-complex", "italic".to_string());
        }
    };
}

macro_rules! style_font_weight_complex {
    ($acc:ident) => {
        /// See §7.8.9 of XSL.
        /// This attribute is evaluated for any UNICODE character whose script type is complex. 20.358
        pub fn set_font_weight_complex(&mut self, weight: FontWeight) {
            self.$acc
                .set_attr("style:font-weight-complex", weight.to_string());
        }

        /// Sets the font-weight to bold. See set_font_weight.
        pub fn set_font_bold_complex(&mut self) {
            self.$acc
                .set_attr("style:font-weight-complex", FontWeight::Bold.to_string());
        }
    };
}

macro_rules! style_font_attr_complex {
    ($acc:ident) => {
        /// Combined font attributes.
        pub fn set_font_attr_complex(&mut self, size: FontSize, bold: bool, italic: bool) {
            self.set_font_size_complex(size);
            if bold {
                self.set_font_bold_complex();
            }
            if italic {
                self.set_font_italic_complex();
            }
        }
    };
}

macro_rules! style_font_relief {
    ($acc:ident) => {
        /// The style:font-relief attribute specifies whether a font should be embossed, engraved, or
        /// neither.
        /// The defined values for the style:font-relief attribute are:
        /// * embossed: characters are embossed.
        /// * engraved: characters are engraved.
        /// * none: characters are neither embossed or engraved.
        pub fn set_font_relief(&mut self, relief: TextRelief) {
            self.$acc.set_attr("style:font-relief", relief.to_string());
        }
    };
}

macro_rules! style_text_position {
    ($acc:ident) => {
        /// The style:text-position attribute specifies whether text is positioned above or below the
        /// baseline and to specify the relative font height that is used for this text.
        /// This attribute can have one or two values.
        /// The first value shall be present and specifies the vertical text position as a percentage of the
        /// current font height or it takes one of the values sub or super. Negative percentages or the sub
        /// value place the text below the baseline. Positive percentages or the super value place the text
        /// above the baseline. If sub or super is specified, the consumer chooses an appropriate text
        /// position.
        /// The second value may be present and specifies the font height as a percentage of the current
        /// font-height. If this value is not specified, an appropriate font height is used.
        pub fn set_text_position(&mut self, pos: TextPosition, scale: Option<Percent>) {
            self.$acc
                .set_attr("style:text-position", text_position(pos, scale));
        }
    };
}

macro_rules! style_letter_kerning {
    ($acc:ident) => {
        /// The style:letter-kerning attribute specifies whether kerning between characters is enabled
        /// or disabled.
        pub fn set_letter_kerning(&mut self, kerning: bool) {
            self.$acc
                .set_attr("style:letter-kerning", kerning.to_string());
        }
    };
}

macro_rules! style_text_combine {
    ($acc:ident) => {
        /// The style:text-combine attribute specifies whether to combine characters so that they are
        /// displayed within two lines.
        ///
        /// The defined values for the style:text-combine attribute are:
        /// * letters: Display text in Kumimoji. Up to five (5) characters are combined within two lines
        /// and are displayed with a reduced size in a single wide-cell character. Additional characters
        /// are displayed as normal text.
        /// * lines: Displays text in Warichu. All characters with the style:text-combine attribute that
        /// immediately follow each other are displayed within two lines of approximately the same length.
        /// A line break may occur between any two characters to meet this constraint.
        /// * none: characters should not be combined.
        pub fn set_text_combine(&mut self, pos: TextCombine) {
            self.$acc.set_attr("style:text-combine", pos.to_string());
        }
    };
}

macro_rules! style_text_combine_start_char {
    ($acc:ident) => {
        /// The style:text-combine-start-char attribute specifies the start character that is displayed
        /// before a portion of text whose style:text-combine 20.367 attribute has a value of lines.
        pub fn set_text_combine_start_char(&mut self, c: char) {
            self.$acc
                .set_attr("style:text-combine-start-char", c.to_string());
        }
    };
}

macro_rules! style_text_combine_end_char {
    ($acc:ident) => {
        /// The style:text-combine-end-char attribute specifies the end character that is displayed
        /// after a portion of text whose style:text-combine 20.367 attribute has a value of lines.
        pub fn set_text_combine_end_char(&mut self, c: char) {
            self.$acc
                .set_attr("style:text-combine-end-char", c.to_string());
        }
    };
}

macro_rules! style_text_emphasize {
    ($acc:ident) => {
        /// The style:text-emphasize attribute specifies emphasis in a text composed of UNICODE
        /// characters whose script type is asian. 20.358
        /// The value of this attribute consists of two white space-separated values.
        /// The first value represents the style to use for emphasis.
        /// The second value represents the position of the emphasis and it can be above or below. If the
        /// first value is none, the second value can be omitted.
        /// The defined values for the style:text-emphasize attribute are:
        /// * accent: calligraphic accent strokes.
        /// * circle: hollow circles.
        /// * disc: filled circles.
        /// * dot: calligraphic dot.
        /// * none: no emphasis marks.
        pub fn set_text_emphasize(
            &mut self,
            emphasize: TextEmphasize,
            position: TextEmphasizePosition,
        ) {
            self.$acc.set_attr(
                "style:text-emphasize",
                format!("{} {}", emphasize, position),
            );
        }
    };
}

macro_rules! style_text_line_through {
    ($acc:ident) => {
        /// The style:text-line-through-color attribute specifies the color that is used for linethrough text.
        /// The defined values for the style:text-line-through-color attribute are:
        /// * font-color: current text color is used for underlining.
        /// * a value of type color 18.3.9
        pub fn set_text_line_through_color(&mut self, color: Rgb<u8>) {
            self.$acc
                .set_attr("style:text-line-through-color", color_string(color));
        }

        /// The style:text-line-through-mode attribute specifies whether lining through is applied to
        /// words only or to portions of text.
        /// The defined values for the style:text-line-through-mode attribute are:
        /// * continuous: lining is applied to words and separating spaces.
        /// * skip-white-space: lining is not applied to spaces between words.
        pub fn set_text_line_through_mode(&mut self, lmode: LineMode) {
            self.$acc
                .set_attr("style:text-line-through-mode", lmode.to_string());
        }

        /// The style:text-line-through-style attribute specifies a style for rendering a line-through
        /// text.
        /// The defined values for the style:text-line-through-style attribute are:
        /// * none: text has no line through it.
        /// * dash: text has a dashed line through it.
        /// * dot-dash: text has a line whose repeating pattern is a dot followed by a dash through it.
        /// * dot-dot-dash: text has a line whose repeating pattern is two dots followed by a dash
        /// through it.
        /// * dotted: text has a dotted line through it.
        /// * long-dash: text has a dashed line whose dashes are longer than the ones from the dashed
        /// line for value dash through it.
        /// * solid: text has a solid line through it.
        /// * wave: text has a wavy line through it.
        /// Note: The definitions of the values of the style:text-line-through-style attribute are
        /// based on the text decoration style 'text-line-through-style' from CSS3Text, §9.2.
        pub fn set_text_line_through_style(&mut self, lstyle: LineStyle) {
            self.$acc
                .set_attr("style:text-line-through-style", lstyle.to_string());
        }

        /// The style:text-line-through-text attribute specifies a text that is used for line-through.
        /// The attribute will be evaluated only if the value of style:text-line-through-style 20.373
        /// attribute is different than none.
        /// If the attribute value is not empty, the attribute value string is used for line-through instead of the
        /// line style that has been specified by the style:text-line-through-style attribute.
        /// Consumers that do not support line-through with text should ignore the attribute, and should use
        /// the line style specified by the style:text-line-through-style attribute.
        /// Consumers that support line-through with single characters only, should use the first character of
        /// the value for line-through, if the style:text-line-through-text attribute value has more
        /// than one character. Consumers that support line-through with specific characters only (like ”x” or
        /// ”/” (U+002F, SOLIDUS) should use one of these characters if the attribute specifies characters
        /// that are not supported.
        pub fn set_text_line_through_text<S: Into<String>>(&mut self, text: S) {
            self.$acc
                .set_attr("style:text-line-through-text", text.into());
        }

        /// The style:text-line-through-text-style specifies a text style that is applied to
        /// text-linethrough characters. It is not applied to line-through lines. If the attribute
        /// appears in an automatic style, it may reference either an automatic text style or a
        /// common style. If the attribute appears in a common style, it may reference a common
        /// style only.
        pub fn set_text_line_through_text_style(&mut self, style_ref: TextStyleRef) {
            self.$acc.set_attr(
                "style:text-line-through-text-style",
                style_ref.as_str().to_string(),
            );
        }

        /// The style:text-line-through-type attribute specifies whether text is lined through, and if
        /// so, whether a single or double line will be used.
        /// The defined values for the style:text-line-through-type attribute are:
        /// * double: a double line should be used for a line-through text.
        /// * none: deprecated.
        /// * single: a single line should be used for a line-through text.
        /// Every occurrence of the style:text-line-through-type attribute should be accompanied
        /// by an occurrence of the style:text-line-through-style 20.373 attribute on the same
        /// element. There should not be an occurrence of the style:text-line-through-type attribute
        /// if the value of the style:text-line-through-sty
        pub fn set_text_line_through_type(&mut self, ltype: LineType) {
            self.$acc
                .set_attr("style:text-line-through-type", ltype.to_string());
        }

        /// The style:text-line-through-width attribute specifies the width of a line-through line. The
        /// value bold specifies a line width that is calculated from the font sizes like an auto width, but is
        /// wider than an auto width.
        /// The defined values for the style:text-line-through-width attribute are:
        /// * auto: the width of a line-through should be calculated from the font size of the text where the
        /// line-through will appear.
        /// * bold: the width of a line-through should be calculated from the font size of the text where the
        /// line-through will appear but is wider than for the value of auto.
        /// * a value of type percent 18.3.23
        /// * a value of type positiveInteger 18.2
        /// * a value of type positiveLength 18.3.26
        /// The line-through text styles referenced by the values dash, medium, thick and thin, are
        /// implementation-defined. Thin shall be smaller width than medium and medium shall be a smaller
        /// width than thick.
        pub fn set_text_line_through_width(&mut self, lwidth: LineWidth) {
            self.$acc
                .set_attr("style:text-line-through-width", lwidth.to_string());
        }
    };
}

macro_rules! style_text_outline {
    ($acc:ident) => {
        /// The style:text-outline attribute specifies whether to display an
        /// outline of text or the text itself.
        pub fn set_font_text_outline(&mut self, outline: bool) {
            self.$acc
                .set_attr("style:text-outline", outline.to_string());
        }
    };
}

macro_rules! style_text_overline {
    ($acc:ident) => {
        /// The style:text-overline-color attribute specifies a color that is
        /// used to overline text.
        ///
        /// The defined values for the style:text-overline-color attribute are:
        /// * font-color: the current text color is used for overlining.
        /// * a value of type color
        pub fn set_text_overline_color(&mut self, color: Rgb<u8>) {
            self.$acc
                .set_attr("style:text-overline-color", color_string(color));
        }

        /// The style:text-overline-mode attribute specifies whether overlining is applied to words
        /// only or to portions of text.
        pub fn set_text_overline_mode(&mut self, lmode: LineMode) {
            self.$acc
                .set_attr("style:text-overline-mode", lmode.to_string());
        }

        /// The style:text-overline-style attribute specifies a style for rendering a line over text.
        pub fn set_text_overline_style(&mut self, lstyle: LineStyle) {
            self.$acc
                .set_attr("style:text-overline-style", lstyle.to_string());
        }

        /// The style:text-overline-type attribute specifies the type of overlining applied to a text.
        pub fn set_text_overline_type(&mut self, ltype: LineType) {
            self.$acc
                .set_attr("style:text-overline-type", ltype.to_string());
        }

        /// The style:text-overline-width attribute specifies the width of an overline. The value bold
        /// specifies a line width that is calculated from the font sizes like an auto width, but is wider than an
        /// auto width.
        pub fn set_text_overline_width(&mut self, lwidth: LineWidth) {
            self.$acc
                .set_attr("style:text-overline-width", lwidth.to_string());
        }
    };
}

macro_rules! style_text_underline {
    ($acc:ident) => {
        /// The style:text-underline-color attribute specifies a color that is used to underline text.
        /// The defined values for the style:text-underline-color attribute are:
        /// * font-color: the current text color is used for underlining.
        /// * a value of type color: the color to be used for underlining.
        pub fn set_text_underline_color(&mut self, color: Rgb<u8>) {
            self.$acc
                .set_attr("style:text-underline-color", color_string(color));
        }

        /// The style:text-underline-mode attribute specifies whether underlining is applied to words
        /// only or to portions of text. If underlining is applied to text portions, the spaces between words and
        /// the words are underlined.
        pub fn set_text_underline_mode(&mut self, lmode: LineMode) {
            self.$acc
                .set_attr("style:text-underline-mode", lmode.to_string());
        }

        /// The style:text-underline-style attribute specifies a style for underlining text
        pub fn set_text_underline_style(&mut self, lstyle: LineStyle) {
            self.$acc
                .set_attr("style:text-underline-style", lstyle.to_string());
        }

        /// The style:text-underline-type attribute specifies the type of underlining applied to a text
        pub fn set_text_underline_type(&mut self, ltype: LineType) {
            self.$acc
                .set_attr("style:text-underline-type", ltype.to_string());
        }

        /// The style:text-underline-width attribute specifies the width of an underline. The value
        /// bold specifies a line width that is calculated from the font sizes like an auto width, but is wider
        /// than an auto width.
        pub fn set_text_underline_width(&mut self, lwidth: LineWidth) {
            self.$acc
                .set_attr("style:text-underline-width", lwidth.to_string());
        }
    };
}

macro_rules! style_use_window_font_color {
    ($acc:ident) => {
        /// The style:use-window-font-color attribute specifies whether the window foreground color
        /// should be used as the foreground color for a light background color and white for a dark
        /// background color. The determination of light or dark color is implementation-defined.
        pub fn set_use_window_font_color(&mut self, window_color: bool) {
            self.$acc
                .set_attr("style:use-window-font-color", window_color.to_string());
        }
    };
}

macro_rules! style_dynamic_spacing {
    ($acc:ident) => {
        /// Dynamic spacing
        pub fn set_dynamic_spacing(&mut self, dynamic: bool) {
            self.$acc
                .set_attr("style:dynamic-spacing", dynamic.to_string());
        }
    };
}

macro_rules! style_column_width {
    ($acc:ident) => {
        /// The style:column-width attribute specifies a fixed width for a column.
        pub fn set_col_width(&mut self, width: Length) {
            if width == Length::Default {
                self.$acc.clear_attr("style:column-width");
            } else {
                self.$acc.set_attr("style:column-width", width.to_string());
            }
        }

        /// Parses the column width.
        pub fn col_width(&self) -> Result<Length, OdsError> {
            Length::parse_attr_def(self.$acc.attr("style:column-width"), Length::Default)
        }
    };
}

macro_rules! style_rel_column_width {
    ($acc:ident) => {
        /// The style:rel-column-width attribute specifies a relative width of a column with a number
        /// value, followed by a ”*” (U+002A, ASTERISK) character. If rc is the relative with of the column, rs
        /// the sum of all relative columns widths, and ws the absolute width that is available for these
        /// columns the absolute width wc of the column is wc=rcws/rs.
        pub fn set_rel_col_width(&mut self, rel: f64) {
            self.$acc
                .set_attr("style:rel-column-width", rel_width_string(rel));
        }
    };
}

macro_rules! style_use_optimal_column_width {
    ($acc:ident) => {
        /// The style:use-optimal-column-width attribute specifies that a column width should be
        /// recalculated automatically if content in the column changes.
        pub fn set_use_optimal_col_width(&mut self, opt: bool) {
            self.$acc
                .set_attr("style:use-optimal-column-width", opt.to_string());
        }

        /// Parses the flag.
        pub fn use_optimal_col_width(&self) -> Result<bool, OdsError> {
            bool::parse_attr_def(self.$acc.attr("style:use-optimal-column-width"), false)
        }
    };
}

macro_rules! style_font_family_generic {
    ($acc:ident) => {
        /// The style:font-family-generic attribute specifies a generic font family name.
        /// The defined values for the style:font-family-generic attribute are:
        /// * decorative: the family of decorative fonts.
        /// * modern: the family of modern fonts.
        /// * roman: the family roman fonts (with serifs).
        /// * script: the family of script fonts.
        /// * swiss: the family roman fonts (without serifs).
        /// * system: the family system fonts.
        pub fn set_font_family_generic(&mut self, font: FontFamilyGeneric) {
            self.$acc
                .set_attr("style:font-family-generic", font.to_string());
        }
    };
}

macro_rules! style_font_pitch {
    ($acc:ident) => {
        /// The style:font-pitch attribute specifies whether a font has a fixed or variable width.
        /// The defined values for the style:font-pitch attribute are:
        /// * fixed: font has a fixed width.
        /// * variable: font has a variable width.
        pub fn set_font_pitch(&mut self, pitch: FontPitch) {
            self.$acc.set_attr("style:font-pitch", pitch.to_string());
        }
    };
}

macro_rules! style_first_page_number {
    ($acc:ident) => {
        /// The style:first-page-number attribute specifies the number of a document.
        /// The value of this attribute can be an integer or continue. If the value is continue, the page
        /// number is the preceding page number incremented by 1. The default first page number is 1.
        pub fn set_first_page_number(&mut self, number: u32) {
            self.style_mut()
                .set_attr("style:first-page-number", number.to_string());
        }
    };
}

macro_rules! style_footnote_max_height {
    ($acc:ident) => {
        /// The style:footnote-max-height attribute specifies the maximum amount of space on a
        /// page that a footnote can occupy. The value of the attribute is a length, which determines the
        /// maximum height of a footnote area.
        /// If the value of this attribute is set to 0cm, there is no limit to the amount of space that the footnote
        /// can occupy.
        pub fn set_footnote_max_height(&mut self, height: Length) {
            self.style_mut()
                .set_attr("style:footnote-max-height", height.to_string());
        }
    };
}

macro_rules! style_num_format {
    ($acc:ident) => {
        /// The style:num-format attribute specifies a numbering sequence.
        /// If no value is given, no number sequence is displayed.
        ///
        /// The defined values for the style:num-format attribute are:
        /// * 1: number sequence starts with “1”.
        /// * a: number sequence starts with “a”.
        /// * A: number sequence starts with “A”.
        /// * empty string: no number sequence displayed.
        /// * i: number sequence starts with “i”.
        /// * I: number sequence start with “I”.
        /// * a value of type string 18.2
        pub fn set_num_format(&mut self, format: StyleNumFormat) {
            self.style_mut()
                .set_attr("style:num-format", format.to_string());
        }
    };
}

macro_rules! style_num_letter_sync {
    ($acc:ident) => {
        /// The style:num-letter-sync attribute specifies whether letter synchronization shall take
        /// place. If letters are used in alphabetical order for numbering, there are two ways to process
        /// overflows within a digit, as follows:
        /// * false: A new digit is inserted that always has the same value as the following digit. The
        /// numbering sequence (for lower case numberings) in that case is a, b, c, ..., z, aa, bb, cc, ...,
        /// zz, aaa, ..., and so on.
        /// * true: A new digit is inserted. Its start value is ”a” or ”A”, and it is incremented every time an
        /// overflow occurs in the following digit. The numbering sequence (for lower case numberings) in
        /// that case is a,b,c, ..., z, aa, ab, ac, ...,az, ba, ..., and so on
        pub fn set_num_letter_sync(&mut self, sync: bool) {
            self.style_mut()
                .set_attr("style:num-letter-sync", sync.to_string());
        }
    };
}

macro_rules! style_num_prefix {
    ($acc:ident) => {
        /// The style:num-prefix attribute specifies what to display before a number.
        /// If the style:num-prefix and style:num-suffix values do not contain any character that
        /// has a Unicode category of Nd, Nl, No, Lu, Ll, Lt, Lm or Lo, an XSLT format attribute can be
        /// created from the OpenDocument attributes by concatenating the values of the style:num-prefix,
        /// style:num-format, and style:num-suffix attributes.
        pub fn set_num_prefix<S: Into<String>>(&mut self, prefix: S) {
            self.style_mut().set_attr("style:num-prefix", prefix.into());
        }
    };
}

macro_rules! style_num_suffix {
    ($acc:ident) => {
        /// The style:num-prefix and style:num-suffix attributes specify what to display before and
        /// after a number.
        /// If the style:num-prefix and style:num-suffix values do not contain any character that
        /// has a Unicode category of Nd, Nl, No, Lu, Ll, Lt, Lm or Lo, an XSLT format attribute can be
        /// created from the OpenDocument attributes by concatenating the values of the style:numprefix, style:num-format, and style:num-suffix attributes.
        pub fn set_num_suffix<S: Into<String>>(&mut self, suffix: S) {
            self.style_mut().set_attr("style:num-suffix", suffix.into());
        }
    };
}

macro_rules! style_paper_tray_name {
    ($acc:ident) => {
        /// The style:paper-tray-name attribute specifies the paper tray to use when printing a
        /// document. The names assigned to the paper trays depends upon the printer.
        /// The defined values for the style:paper-tray-name attribute are:
        /// * default: the default tray specified by printer configuration settings.
        /// * a value of type string
        pub fn set_paper_tray_name<S: Into<String>>(&mut self, tray: S) {
            self.style_mut()
                .set_attr("style:paper-tray-name", tray.into());
        }
    };
}

macro_rules! style_print {
    ($acc:ident) => {
        /// The style:print attribute specifies the components in a spreadsheet document to print.
        /// The value of the style:print attribute is a white space separated list of one or more of these
        /// values: headers, grid, annotations, objects, charts, drawings, formulas, zerovalues, or the empty list.
        /// The defined values for the style:print attribute are:
        /// * annotations: annotations should be printed.
        /// * charts: charts should be printed.
        /// * drawings: drawings should be printed.
        /// * formulas: formulas should be printed.
        /// * headers: headers should be printed.
        /// * grid: grid lines should be printed.
        /// * objects: (including graphics): objects should be printed.
        /// * zero-values: zero-values should be printed.
        pub fn set_print(&mut self, print: &[PrintContent]) {
            let mut buf = String::new();
            for p in print {
                buf.push_str(&p.to_string());
                buf.push(' ');
            }
            self.$acc.set_attr("style:print", buf);
        }
    };
}

macro_rules! style_print_orientation {
    ($acc:ident) => {
        /// The style:print-orientation attribute specifies the orientation of the printed page. The
        /// value of this attribute can be portrait or landscape.
        /// The defined values for the style:print-orientation attribute are:
        /// * landscape: a page is printed in landscape orientation.
        /// * portrait: a page is printed in portrait orientation.
        pub fn set_print_orientation(&mut self, orientation: PrintOrientation) {
            self.$acc
                .set_attr("style:print-orientation", orientation.to_string());
        }
    };
}

macro_rules! style_print_page_order {
    ($acc:ident) => {
        /// The style:print-page-order attribute specifies the order in which data in a spreadsheet is
        /// numbered and printed when the data does not fit on one printed page.
        /// The defined values for the style:print-page-order attribute are:
        /// * ltr: create pages from the first column to the last column before continuing with the next set
        /// of rows.
        /// * ttb: create pages from the top row to the bottom row before continuing with the next set of
        /// columns.
        pub fn set_print_page_order(&mut self, order: PrintOrder) {
            self.$acc
                .set_attr("style:print-page-order", order.to_string());
        }
    };
}

macro_rules! style_scale_to {
    ($acc:ident) => {
        /// The style:scale-to attribute specifies that a document is to be scaled to a percentage value.
        /// A value of 100% means no scaling.
        /// If this attribute and style:scale-to-pages are absent, a document is not scaled.
        pub fn set_scale_to(&mut self, percent: Percent) {
            self.$acc.set_attr("style:scale-to", percent.to_string());
        }
    };
}

macro_rules! style_scale_to_pages {
    ($acc:ident) => {
        /// The style:scale-to-pages attribute specifies the number of pages on which a document
        /// should be printed. The document is scaled to fit a specified number of pages.
        /// If this attribute and style:scale-to are absent, a document is not scaled.
        pub fn set_scale_to_pages(&mut self, pages: u32) {
            self.$acc
                .set_attr("style:scale-to-pages", pages.to_string());
        }
    };
}

macro_rules! style_table_centering {
    ($acc:ident) => {
        /// The style:table-centering attribute specifies whether tables are centered horizontally
        /// and/or vertically on the page. This attribute only applies to spreadsheet documents.
        /// The default is to align the table to the top-left or top-right corner of the page, depending of its
        /// writing direction.
        /// The defined values for the style:table-centering attribute are:
        /// * both: tables should be centered both horizontally and vertically on the pages where they
        /// appear.
        /// * horizontal: tables should be centered horizontally on the pages where they appear.
        /// * none: tables should not be centered both horizontally or vertically on the pages where they
        /// appear.
        /// * vertical: tables should be centered vertically on the pages where they appear.
        pub fn set_table_centering(&mut self, center: PrintCentering) {
            self.$acc
                .set_attr("style:table-centering", center.to_string());
        }
    };
}

macro_rules! style_min_row_height {
    ($acc:ident) => {
        /// The style:min-row-height attribute specifies a fixed minimum height for a row.
        pub fn set_min_row_height(&mut self, min_height: Length) {
            assert!(min_height.is_positive());
            self.$acc
                .set_attr("style:min-row-height", min_height.to_string());
        }
    };
}

macro_rules! style_row_height {
    ($acc:ident) => {
        /// The style:row-height attribute specifies a fixed row height
        pub fn set_row_height(&mut self, height: Length) {
            self.$acc.set_attr("style:row-height", height.to_string());
        }

        /// Parses the row height
        pub fn row_height(&self) -> Result<Length, OdsError> {
            Length::parse_attr_def(self.$acc.attr("style:row-height"), Length::Default)
        }
    };
}

macro_rules! style_use_optimal_row_height {
    ($acc:ident) => {
        /// The style:use-optimal-row-height attribute specifies that a row height should be
        /// recalculated automatically if content in the row changes.
        /// The defined values for the style:use-optimal-row-height attribute are:
        /// * false: row height should not be recalculated automatically if content in the row changes.
        /// * true: row height should be recalculated automatically if content in the row changes.
        pub fn set_use_optimal_row_height(&mut self, opt: bool) {
            self.$acc
                .set_attr("style:use-optimal-row-height", opt.to_string());
        }

        /// Parses the flag.
        pub fn use_optimal_row_height(&self) -> Result<bool, OdsError> {
            bool::parse_attr_def(self.$acc.attr("style:use-optimal-row-height"), false)
        }
    };
}

macro_rules! style_may_break_between_rows {
    ($acc:ident) => {
        /// The style:may-break-between-rows attribute specifies that a page break may occur inside
        /// a table.
        /// The defined values for the style:may-break-between-rows attribute are:
        /// * false: page break shall not occur inside a table.
        /// * true: page break may occur inside a table
        pub fn set_may_break_between_rows(&mut self, br: bool) {
            self.$acc
                .set_attr("style:may-break-between-rows", br.to_string());
        }
    };
}

macro_rules! style_rel_width {
    ($acc:ident) => {
        /// The style:rel-width attribute specifies the relative width of a drawing object.
        /// The defined values for the style:rel-width attribute are:
        /// * scale: the width should be calculated depending on the height, so that the ratio of width and
        /// height of the original image or object size is preserved.
        /// * scale-min: the width should be calculated as for value scale, but the calculated width is a
        /// minimum width rather than an absolute one.
        /// * a value of type percent 18.3.23.
        /// The interpretation of the percent value depends on the anchor of the drawing object. If the anchor
        /// for the drawing object is in a table cell, the percent value of the surrounding table box. If the
        /// anchor for the drawing object is in a text box, the percentage value of the surrounding text box. In
        /// all other cases, the percent value of the containing page or window
        /// To support consumers that do not support relative width, producers should also provide the width
        /// in a svg:width 19.575 attribute.
        pub fn set_rel_width(&mut self, rel_width: RelativeScale) {
            self.$acc.set_attr("style:rel-width", rel_width.to_string());
        }
    };
}

// style:rel-height 19.513
macro_rules! style_rel_height {
    ($acc:ident) => {
        /// The style:rel-height attribute specifies height of a drawing object as a relative value within a
        /// frame.
        /// The defined values for the style:rel-height attribute are:
        /// • scale: the height should be calculated depending on the width, so that the ratio of width and
        /// height of the original image or object size is preserved.
        /// • scale-min: the height should be calculated as for value scale, but the calculated height is
        /// a minimum height rather than an absolute one.
        /// • a value of type percent 18.3.23.
        /// The interpretation of percentage values depends on the anchor of the drawing object. If the
        /// anchor for the drawing object is in a table cell, the percentage value is relative to the surrounding
        /// table box. If the anchor for the drawing object is in a text box, the percentage value is relative to
        /// the surrounding text box. In other cases, the percentage values is relative to the width of the page
        /// or window.
        /// To support consumers that do not support relative width and heights, producers should also
        /// provide the height in svg:height and fo:min-height attributes.
        pub fn set_rel_height(&mut self, rel_height: RelativeScale) {
            self.$acc
                .set_attr("style:rel-height", rel_height.to_string());
        }
    };
}

macro_rules! style_width {
    ($acc:ident) => {
        /// The style:width attribute specifies the fixed width of a table. Every table shall have a fixed
        /// width.
        pub fn set_width(&mut self, width: Length) {
            self.$acc.set_attr("style:width", width.to_string());
        }
    };
}

macro_rules! style_char {
    ($acc:ident) => {
        /// The style:char attribute specifies the delimiter character for tab stops of type char
        pub fn set_char(&mut self, c: char) {
            self.$acc.set_attr("style:char", c.to_string());
        }
    };
}

macro_rules! style_leader_color {
    ($acc:ident) => {
        /// The style:leader-color attribute specifies the color of a leader line. The value of this
        /// attribute is either font-color or a color. If the value is font-color, the current text color is
        /// used for the leader line.
        pub fn set_leader_color(&mut self, color: Rgb<u8>) {
            self.attr
                .set_attr("style:leader-color", color_string(color));
        }
    };
}

macro_rules! style_leader_style {
    ($acc:ident) => {
        /// The style:leader-style attribute specifies a style for a leader line.
        ///
        /// The defined values for the style:leader-style attribute are:
        /// * none: tab stop has no leader line.
        /// * dash: tab stop has a dashed leader line.
        /// * dot-dash: tab stop has a leader line whose repeating pattern is a dot followed by a dash.
        /// * dot-dot-dash: tab stop has a leader line whose repeating pattern has two dots followed by
        /// a dash.
        /// * dotted: tab stop has a dotted leader line.
        /// * long-dash: tab stop has a dashed leader line whose dashes are longer than the ones from
        /// the dashed line for value dash.
        /// * solid: tab stop has a solid leader line.
        /// * wave: tab stop has a wavy leader line.
        ///
        /// Note: The definitions of the values of the style:leader-style attribute are based on the text
        /// decoration style 'text-underline-style' from CSS3Text, §9.2.
        pub fn set_leader_style(&mut self, style: LineStyle) {
            self.$acc.set_attr("style:leader-style", style.to_string());
        }
    };
}

macro_rules! style_leader_text {
    ($acc:ident) => {
        /// The style:leader-text attribute specifies a single Unicode character for use as leader text
        /// for tab stops.
        /// An consumer may support only specific characters as textual leaders. If a character that is not
        /// supported by a consumer is specified by this attribute, the consumer should display a leader
        /// character that it supports instead of the one specified by this attribute.
        /// If both style:leader-text and style:leader-style 19.480 attributes are specified, the
        /// value of the style:leader-text sets the leader text for tab stops.
        ///
        /// The default value for this attribute is “ ” (U+0020, SPACE).
        pub fn set_leader_text(&mut self, text: char) {
            self.$acc.set_attr("style:leader-text", text.to_string());
        }
    };
}

macro_rules! style_leader_text_style {
    ($acc:ident) => {
        /// The style:leader-text-style specifies a text style that is applied to a textual leader. It is
        /// not applied to leader lines. If the attribute appears in an automatic style, it may reference either an
        /// automatic text style or a common style. If the attribute appears in a common style, it may
        /// reference a common style only.
        pub fn set_leader_text_style(&mut self, styleref: &TextStyleRef) {
            self.$acc
                .set_attr("style:leader-text-style", styleref.as_ref().to_string());
        }
    };
}

macro_rules! style_leader_type {
    ($acc:ident) => {
        /// The style:leader-type attribute specifies whether a leader line should be drawn, and if so,
        /// whether a single or double line will be used.
        ///
        /// The defined values for the style:leader-type attribute are:
        /// * double: a double line is drawn.
        /// * none: no line is drawn.
        /// * single: a single line is drawn.
        pub fn set_leader_type(&mut self, t: LineType) {
            self.$acc.set_attr("style:leader-type", t.to_string());
        }
    };
}

macro_rules! style_leader_width {
    ($acc:ident) => {
        /// The style:leader-width attribute specifies the width (i.e., thickness) of a leader line.
        /// The defined values for the style:leader-width attribute are:
        /// * auto: the width of a leader line should be calculated from the font size of the text where the
        /// leader line will appear.
        /// * bold: the width of a leader line should be calculated from the font size of the text where the
        /// leader line will appear but is wider than for the value of auto.
        /// * a value of type percent 18.3.23
        /// * a value of type positiveInteger 18.2
        /// * a value of type positiveLength 18.3.26
        /// The line widths referenced by the values medium, normal, thick and thin are implementation defined.
        pub fn set_leader_width(&mut self, w: LineWidth) {
            self.$acc.set_attr("style:leader-width", w.to_string());
        }
    };
}

macro_rules! style_position {
    ($acc:ident) => {
        /// The style:position attribute specifies the position of a tab stop. Depending on the value of
        /// the text:relative-tab-stop-position 19.861 attribute in the
        /// <text:table-ofcontent-source> 8.3.2,
        /// <text:illustration-index-source> 8.4.2,
        /// <text:object-index-source> 8.6.2,
        /// <text:user-index-source> 8.7.2 or
        /// <text:alphabetical-index-source> 8.8.2
        ///
        /// parent element, the position of the tab is interpreted as being relative to the left
        /// margin or the left indent.
        pub fn set_position(&mut self, pos: Length) {
            self.$acc.set_attr("style:position", pos.to_string());
        }
    };
}

macro_rules! style_type {
    ($acc:ident) => {
        /// The style:type attribute specifies the type of a tab stop within paragraph formatting properties.
        /// The defined values for the style:type attribute are:
        /// * center: text is centered on a tab stop.
        /// * char: character appears at a tab stop position.
        /// * left: text is left aligned with a tab stop.
        /// * right: text is right aligned with a tab stop.
        /// For a <style:tab-stop> 17.8 element the default value for this attribute is left.
        pub fn set_type(&mut self, t: TabStopType) {
            self.$acc.set_attr("style:type", t.to_string());
        }
    };
}

// 19.521 style:volatile
macro_rules! style_volatile {
    ($acc:ident) => {
        /// The style:volatile attribute specifies whether unused style in a document are retained or
        /// discarded by consumers.
        /// The defined values for the style:volatile attribute are:
        /// * false: consumers should discard the unused styles.
        /// * true: consumers should keep unused styles.
        pub fn set_volatile(&mut self, volatile: bool) {
            self.attr.set_attr("style:volatile", volatile.to_string());
        }

        /// Volatile format.
        pub fn volatile(&self) -> Option<bool> {
            match self.attr.attr("style:volatile") {
                None => None,
                Some(s) => FromStr::from_str(s).ok(),
            }
        }
    };
}

// 19.476 style:display-name
macro_rules! style_display_name {
    ($acc:ident) => {
        /// The style:display-name attribute specifies the name of a style as it should appear in the user
        /// interface. If this attribute is not present, the display name should be the same as the style name.
        pub fn set_display_name<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("style:display-name", name.into());
        }
    };
}

// 19.467 style:auto-update
macro_rules! style_auto_update {
    ($acc:ident) => {
        /// The style:auto-update attribute specifies whether styles are automatically updated when the
        /// formatting properties of an object that has the style assigned to it are changed.
        /// The defined values for the style:auto-update attribute are:
        /// * false: a change to a formatting property is applied for the object where the change was
        /// made. If necessary, a new automatic style will be created which is applied to the object where
        /// the change was made.
        /// * true: a change to a formatting property results in the updating of the common style that is
        /// applied to an object. The formatting change is applied to all objects subject to the common
        /// style where the change was made.
        /// The default value for this attribute is false.
        pub fn set_auto_update(&mut self, auto: bool) {
            self.$acc.set_attr("style:auto-update", auto.to_string());
        }
    };
}

// 19.470 style:class
macro_rules! style_class {
    ( $acc:ident) => {
        /// The style:class attribute specifies a style class name.
        /// A style may belong to an arbitrary class of styles. The style class name is an arbitrary string. The
        /// style class name has no meaning within the file format itself, but it can for instance be evaluated
        /// by user interfaces to show a list of styles where the styles are grouped by its name.
        pub fn set_class(&mut self, class: &str) {
            self.$acc.set_attr("style:class", class);
        }
    };
}

// 19.510 style:parent-style-name
macro_rules! style_parent_style_name {
    ($acc:ident, $styleref:ident) => {
        /// The style:parent-style-name attribute specifies the name of a parent style. The parent
        /// style cannot be an automatic style and shall exist.
        /// If a parent style is not specified, the default style which has the same style:family 19.480
        /// attribute value as the current style is used.
        pub fn set_parent_style(&mut self, name: &$styleref) {
            self.$acc
                .set_attr("style:parent-style-name", name.as_str().to_string());
        }
    };
}
