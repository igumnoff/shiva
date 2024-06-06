macro_rules! svg_font_family {
    ($acc:ident) => {
        /// External font family name.
        pub fn set_font_family<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("svg:font-family", name.into());
        }
    };
}

macro_rules! svg_font_stretch {
    ($acc:ident) => {
        /// External font stretch value.
        pub fn set_font_stretch(&mut self, stretch: FontStretch) {
            self.$acc.set_attr("svg:font-stretch", stretch.to_string());
        }
    };
}

macro_rules! svg_font_style {
    ($acc:ident) => {
        /// External font style value.
        pub fn set_font_style(&mut self, style: FontStyle) {
            self.$acc.set_attr("svg:font-style", style.to_string());
        }
    };
}

macro_rules! svg_font_variant {
    ($acc:ident) => {
        /// External font variant.
        pub fn set_font_variant(&mut self, variant: FontVariant) {
            self.$acc.set_attr("svg:font-variant", variant.to_string());
        }
    };
}

macro_rules! svg_font_weight {
    ($acc:ident) => {
        /// External font weight.
        pub fn set_font_weight(&mut self, weight: FontWeight) {
            self.$acc.set_attr("svg:font-weight", weight.to_string());
        }
    };
}

macro_rules! svg_height {
    ($acc:ident) => {
        /// Height.
        pub fn set_height(&mut self, height: Length) {
            self.$acc.set_attr("svg:height", height.to_string());
        }
    };
}

macro_rules! svg_width {
    ($acc:ident) => {
        /// Width.
        pub fn set_width(&mut self, width: Length) {
            self.$acc.set_attr("svg:width", width.to_string());
        }
    };
}

#[allow(unused_macros)]
macro_rules! svg_rx {
    ($acc:ident) => {
        /// See §9.4 of[SVG].
        /// The svg:rx and svg:ry attributes can be used to round off the corners of a rectangle. The
        /// svg:rx attribute specifies the x-axis radius of the ellipse used to round off the corners of a
        /// rectangle. The svg:ry attribute specifies the y-axis radius of that ellipse. If only the svg:rx
        /// attribute is present then its value will be used for svg:ry. If only a svg:ry attribute is present
        /// then its value will be used for svg:rx.
        ///
        /// For use of this attribute with <draw:rect> see §9.2 of [SVG].
        /// For use of this attribute with <draw:ellipse> see §9.4 of [SVG].
        pub fn svg_rx(&mut self, rx: Length) {
            self.$acc.set_attr("svg:rx", rx.to_string());
        }
    };
}

#[allow(unused_macros)]
macro_rules! svg_ry {
    ($acc:ident) => {
        /// See §9.4 of [SVG].
        /// The svg:rx and svg:ry attributes can be used to round off the corners of a rectangle. The
        /// svg:rx attribute specifies the x-axis radius of the ellipse used to round off the corners of a
        /// rectangle. The svg:ry attribute specifies the y-axis radius of that ellipse. If only the svg:rx
        /// attribute is present then its value will be used for svg:ry. If only a svg:ry attribute is present
        /// then its value will be used for svg:rx.
        pub fn svg_ry(&mut self, ry: Length) {
            self.$acc.set_attr("svg:ry", ry.to_string());
        }
    };
}

macro_rules! svg_x {
    ($acc:ident) => {
        /// See §5.1.2 of SVG. For drawing shapes that have a non-rectangular shape, the coordinate
        /// refers to the drawing shape's bounding box.
        pub fn svg_x(&mut self, x: Length) {
            self.$acc.set_attr("svg:x", x.to_string());
        }
    };
}

macro_rules! svg_y {
    ($acc:ident) => {
        /// See §5.1.2 of SVG. For drawing shapes that have a non-rectangular shape, the coordinate
        /// refers to the drawing shape's bounding box.
        pub fn svg_y(&mut self, y: Length) {
            self.$acc.set_attr("svg:y", y.to_string());
        }
    };
}
