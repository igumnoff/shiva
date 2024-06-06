macro_rules! table_align {
    ($acc:ident) => {
        /// The table:align attribute specifies the horizontal alignment of a table.
        /// The defined values for the table:align attribute are:
        /// * center: table aligns to the center between left and right margins.
        /// * left: table aligns to the left margin.
        /// * margins: table fills all the space between the left and right margins.
        /// * right: table aligns to the right margin.
        /// Consumers that do not support the margins value, may treat this value as left.
        pub fn set_align(&mut self, align: TableAlign) {
            self.$acc.set_attr("table:align", align.to_string());
        }
    };
}

macro_rules! table_border_model {
    ($acc:ident) => {
        /// The table:border-model attribute specifies what border model to use when creating a table
        /// with a border.
        /// The defined values for the table:border-model attribute are:
        /// * collapsing: when two adjacent cells have different borders, the wider border appears as
        /// the border between the cells. Each cell receives half of the width of the border.
        /// * separating: borders appear within the cell that specifies the border.
        /// In OpenDocument, a row height or column width includes any space required to display borders
        /// or padding. This means that, while the width and height of the content area is less than the
        /// column width and row height, the sum of the widths of all columns is equal to the total width of the
        /// table.
        pub fn set_border_model(&mut self, border: TableBorderModel) {
            self.$acc.set_attr("table:border-model", border.to_string());
        }
    };
}

macro_rules! table_display {
    ($acc:ident) => {
        /// The table:display attribute specifies whether a table is displayed.
        /// The defined values for the table:display attribute are:
        /// * false: table should not be displayed.
        /// * true: table should be displayed.
        pub fn set_display(&mut self, display: bool) {
            self.$acc.set_attr("table:display", display.to_string())
        }
    };
}

macro_rules! table_tab_color {
    ($acc:ident) => {
        /// The table:tab-color attribute specifies the color of the tab associated with a sheet.
        /// When this attribute is missing, the application should use the default color used for sheet tabs.
        pub fn set_tab_color(&mut self, color: Rgb<u8>) {
            self.$acc.set_attr("table:tab-color", color_string(color));
        }
    };
}

macro_rules! table_end_cell_address {
    ($acc:ident) => {
        /// The table:end-cell-address attribute specifies the end position of the shape if it is included
        /// in a spreadsheet document.
        pub fn set_table_end_cell_address(&mut self, cellref: CellRef) {
            self.$acc
                .set_attr("table:end-cell-address", cellref.to_string());
        }
    };
}

macro_rules! table_end_x {
    ($acc:ident) => {
        /// The table:end-x attribute specifies the x-coordinate of the end position of a shape relative to
        /// the top-left edge of a cell. The size attributes of the shape are ignored.
        pub fn set_table_end_x(&mut self, x: Length) {
            self.$acc.set_attr("table:end-x", x.to_string());
        }
    };
}

macro_rules! table_end_y {
    ($acc:ident) => {
        /// The table:end-y attribute specifies the y-coordinate of the end position of a shape relative to
        /// the top-left edge of a cell. The size attributes of the shape are ignored.
        pub fn set_table_end_y(&mut self, y: Length) {
            self.$acc.set_attr("table:end-y", y.to_string());
        }
    };
}

macro_rules! table_table_background {
    ($acc:ident) => {
        /// The table:table-background attribute specifies whether a shape is in the table background if
        /// the drawing shape is included in a spreadsheet document.
        /// The defined values for the table:table-background attribute are:
        /// • false: shape is included in foreground of a table.
        /// • true: shape is included in background of a table.
        pub fn set_table_table_background(&mut self, is_bg: bool) {
            self.$acc
                .set_attr("table:table-background", is_bg.to_string());
        }
    };
}
