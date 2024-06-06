macro_rules! draw_caption_id {
    ($acc:ident) => {
        /// The draw:caption-id attribute establishes a relationship between a drawing shape and its
        /// caption. It takes a value of type IDREF. The value for the draw:caption-id attribute is the
        /// target ID assigned to the <draw:text-box> 10.4.3 element that contains the caption.
        /// Note: When a caption is assigned to a drawing shape, an id shall be assigned to the element
        /// containing the text used to caption a drawing shape. Removing the caption should result in
        /// removing the draw:caption-id attribute of the drawing shape that was being captioned
        pub fn set_draw_caption_id<S: Into<String>>(&mut self, id: S) {
            self.$acc.set_attr("draw:caption-id", id.into());
        }
    };
}

macro_rules! draw_name {
    ($acc:ident) => {
        /// The draw:name attribute specifies a name by which a <draw:frame> element can be
        /// referenced.
        pub fn set_draw_name<S: Into<String>>(&mut self, id: S) {
            self.$acc.set_attr("draw:name", id.into());
        }
    };
}

macro_rules! draw_class_names {
    ($acc:ident) => {
        /// The draw:class-names attribute specifies a white-space-separated list of styles with the family
        /// value of graphic. The referenced styles are applied in the order they are contained in the list.
        /// If both draw:style-name and draw:class-names are present, the style referenced by the
        /// draw:style-name attribute is applied before the styles referenced by the draw:class-names
        /// attribute.
        pub fn set_draw_class_names(&mut self, names: &[GraphicStyleRef]) {
            let mut s = String::new();
            for v in names {
                s.push_str(v.as_str());
                s.push(' ');
            }
            self.$acc.set_attr("draw:class-names", s);
        }
    };
}

macro_rules! draw_corner_radius {
    ($acc:ident) => {
        /// The draw:corner-radius attribute specifies the radius of the circle used to round off the
        /// corners of a caption <draw:caption>, rectangle <draw:rect>, or a text-box <draw:textbox>.
        /// The svg:rx 19.554 and svg:ry 19.555 attributes can also be used to round off the corners of a
        /// rectangle <draw:rect>.
        ///
        /// If svg:rx and/or svg:ry and draw:corner-radius attributes are present on an element, the
        /// svg:rx and svg:ry attributes control the rounding applied to the shape defined by the element.
        /// If one or both of svg:rx and svg:ry attributes are present, any draw:corner-radius
        /// attribute is ignored.
        pub fn set_draw_corner_radius(&mut self, radius: u32) {
            self.$acc.set_attr("draw:corner-radius", radius.to_string());
        }
    };
}

macro_rules! draw_id {
    ($acc:ident) => {
        /// The draw:id attribute specifies identifiers for draw elements.
        /// OpenDocument consumers shall ignore a draw:id attribute if it occurs on a draw element with
        /// an xml:id attribute value.
        /// OpenDocument producers may write draw:id attributes for any draw element in addition to an
        /// xml:id attribute.
        /// The value of a draw:id attribute shall equal the value of an xml:id attribute on the same
        /// element.
        ///
        /// The draw:id attribute is deprecated in favor of xml:id
        pub fn set_draw_id<S: Into<String>>(&mut self, id: S) {
            self.$acc.set_attr("draw:id", id.into());
        }
    };
}

macro_rules! draw_layer {
    ($acc:ident) => {
        /// The draw:layer attribute specifies the name of a layer in the layer-set of a document.
        ///
        /// Note: The effect of this attribute is to assign a shape to a particular layer.
        pub fn set_draw_layer<S: Into<String>>(&mut self, layer: S) {
            self.$acc.set_attr("draw:layer", layer.into());
        }
    };
}

macro_rules! draw_style_name {
    ($acc:ident) => {
        /// The draw:style-name attribute specifies the name of a <style:style> element with a
        /// style:family attribute value value of graphic.
        pub fn set_draw_style_name(&mut self, style: GraphicStyleRef) {
            self.$acc
                .set_attr("draw:style-name", style.as_str().to_string());
        }
    };
}

macro_rules! draw_text_style_name {
    ($acc:ident) => {
        /// The draw:text-style-name attribute specifies a style for formatting of text in a shape.
        /// The value of this attribute is the name of a <style:style> 16.2 element with a style:family
        /// 19.480 attribute value of paragraph.
        pub fn set_draw_text_style_name(&mut self, style: ParagraphStyleRef) {
            self.$acc
                .set_attr("draw:text-style-name", style.as_str().to_string());
        }
    };
}

macro_rules! draw_transform {
    ($acc:ident) => {
        /// The draw:transform attribute specifies a list of transformations that can be applied to a
        /// drawing shape.
        /// The value of this attribute is a list of transform definitions, which are applied to the drawing shape
        /// in the order in which they are listed. The transform definitions in the list shall be separated by a
        /// white space and/or a comma “,” (U+002C, COMMA). Unless otherwise stated, the parameters of
        /// the transform definitions are double values (18.2)
        ///
        /// The defined transforms are:
        /// • matrix(a b c d e f), specifies a transformation in the form of a
        /// transformation matrix of six values. "The values describe a standard 3x2 homogeneous
        /// transformation matrix in column-major order, where the right column (e, f) describes the
        /// translation.
        /// • rotate(rotate-angle), specifies a rotation by rotate-angle degrees about the
        /// origin of the shape’s coordinate system.
        /// • scale(sx \[sy\]), specifies a scale operation by sx and sy. If sy is not
        /// provided, it is assumed to be equal to sx.
        /// • skewX(skew-angle), specifies a skew transformation by skew-angle degrees along
        /// the x-axis.
        /// • skewY(skew-angle), specifies a skew transformation by skew-angle degrees along
        /// the y-axis.
        /// • translate(tx \[ty\]), specifies a translation by tx and ty, where tx and
        /// ty are lengths (18.3.18). If ty is not provided, it is assumed to be zero.
        pub fn set_draw_transform<S: AsRef<str>>(&mut self, transform: &[S]) {
            let mut s = String::new();
            for v in transform {
                s.push_str(v.as_ref());
                s.push(' ');
            }
            self.$acc.set_attr("draw:transform", s.to_string());
        }
    };
}

macro_rules! draw_z_index {
    ($acc:ident) => {
        /// The draw:z-index attribute defines a rendering order for shapes in a document instance. In the
        /// absence of this attribute, shapes are rendered in the order in which they appear in the document.
        /// The draw:z-index values increase from back to front.
        ///
        /// For a shape on which the style:run-through 20.351 attribute with value foreground is in
        /// effect, producers should not generate a draw:z-index value that is smaller than the value of
        /// any draw:z-index on a shape on which the style:run-through attribute with value
        /// background is in effect.
        ///
        /// Producers shall not generate a draw:z-index for shapes that are children of a <draw:g>
        /// element 10.3.15 or a <dr3d:scene> element 10.5.2.
        pub fn set_draw_z_index(&mut self, z_index: u32) {
            self.$acc.set_attr("draw:z-index", z_index.to_string());
        }
    };
}

macro_rules! draw_caption_point_x {
    ($acc:ident) => {
        /// The draw:caption-point-x attribute, along with draw:caption-point-y specifies the
        /// position of a point that is captioned. A set of lines is rendered to that point from the caption area.
        pub fn set_draw_caption_point_x(&mut self, x: Length) {
            self.$acc.set_attr("draw:caption-point-x", x.to_string());
        }
    };
}

macro_rules! draw_caption_point_y {
    ($acc:ident) => {
        /// The draw:caption-point-y attribute, along with draw:caption-point-y specifies the
        /// position of a point that is captioned. A set of lines is rendered to that point from the caption area.
        pub fn set_draw_caption_point_y(&mut self, y: Length) {
            self.$acc.set_attr("draw:caption-point-y", y.to_string());
        }
    };
}

macro_rules! draw_copy_of {
    ($acc:ident) => {
        /// The draw:copy-of attribute specifies that a frame displays the contents of another frame. This
        /// does not effect style and position information. The style and position information of the frame with
        /// the draw:copy-of attribute is used to render the copied contents.
        /// Note: Multiple frames can be set to display the exact same underlying data: for
        /// instance for a company logo, that that is to appear somewhere on every page,
        /// without being part of a header or footer.
        pub fn set_draw_copy_of<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("draw:copy-of", name.into());
        }
    };
}

macro_rules! draw_filter_name {
    ($acc:ident) => {
        /// The draw:filter-name attribute specifies the implementation-dependent filter name that has
        /// been used to load an image into the document
        pub fn set_draw_filter_name<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("draw:filter-name", name.into());
        }
    };
}

macro_rules! draw_mime_type {
    ($acc:ident) => {
        /// The draw:mime-type attribute specifies the MIME type of the media type that a plugin
        /// processes, or the MIME type of the image given by a <draw:image> element. Valid values for
        /// this attribute are those defined in accordance with §3.7 of RFC2616, or registered in accordance
        /// with RFC6838.
        /// Note: Additional information on MIME media types can be found at MIMETYPES.
        pub fn set_draw_mime_type<S: Into<String>>(&mut self, name: S) {
            self.$acc.set_attr("draw:mime-type", name.into());
        }
    };
}
