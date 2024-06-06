macro_rules! styles_styles2 {
    ($style:ident, $styleref:ident) => {
        impl $style {
            /// Origin of the style, either styles.xml oder content.xml
            pub fn origin(&self) -> StyleOrigin {
                self.origin
            }

            /// Changes the origin.
            pub fn set_origin(&mut self, origin: StyleOrigin) {
                self.origin = origin;
            }

            /// Usage for the style.
            pub fn styleuse(&self) -> StyleUse {
                self.styleuse
            }

            /// Usage for the style.
            pub fn set_styleuse(&mut self, styleuse: StyleUse) {
                self.styleuse = styleuse;
            }

            /// Stylename
            pub fn name(&self) -> &str {
                &self.name
            }

            /// Stylename
            pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
                self.name = String::from(name.as_ref());
            }

            /// Returns the name as a style reference.
            pub fn style_ref(&self) -> $styleref {
                $styleref::from(self.name.as_str())
            }

            style_auto_update!(attr);
            style_class!(attr);
            style_display_name!(attr);
            style_parent_style_name!(attr, $styleref);
        }
    };
}

/// Generates a name reference for a style.
macro_rules! style_ref2 {
    ($l:ident) => {
        style_ref2_base!($l);

        impl From<AnyStyleRef> for $l {
            fn from(value: AnyStyleRef) -> Self {
                Self { id: value.id }
            }
        }

        impl From<$l> for AnyStyleRef {
            fn from(value: $l) -> Self {
                Self { id: value.id }
            }
        }
    };
}

/// Generates a name reference for a style.
macro_rules! style_ref2_base {
    ($l:ident) => {
        /// Reference
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub struct $l {
            pub(crate) id: String,
        }

        impl GetSize for $l {
            fn get_heap_size(&self) -> usize {
                self.id.get_heap_size()
            }
        }

        impl From<String> for $l {
            fn from(id: String) -> Self {
                Self { id }
            }
        }

        impl From<&String> for $l {
            fn from(id: &String) -> Self {
                Self { id: id.clone() }
            }
        }

        impl From<&str> for $l {
            fn from(id: &str) -> Self {
                Self { id: id.to_string() }
            }
        }

        impl Borrow<str> for $l {
            fn borrow(&self) -> &str {
                self.id.borrow()
            }
        }

        impl AsRef<str> for $l {
            fn as_ref(&self) -> &str {
                self.id.as_ref()
            }
        }

        impl $l {
            /// Reference as str.
            pub fn as_str(&self) -> &str {
                self.id.as_str()
            }
        }
    };
}

macro_rules! xml_id {
    ($acc:ident) => {
        /// The table:end-y attribute specifies the y-coordinate of the end position of a shape relative to
        /// the top-left edge of a cell. The size attributes of the shape are ignored.
        pub fn set_xml_id<S: Into<String>>(&mut self, id: S) {
            self.$acc.set_attr("xml_id", id.into());
        }
    };
}
