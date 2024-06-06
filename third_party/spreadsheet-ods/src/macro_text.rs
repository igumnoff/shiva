/// Generates a text/xml tag.
macro_rules! text_tag {
    ($tag:ident, $xml:literal) => {
        /// $literal
        #[derive(Debug)]
        pub struct $tag {
            xml: XmlTag,
        }

        impl From<$tag> for XmlTag {
            fn from(t: $tag) -> XmlTag {
                t.xml
            }
        }

        impl Display for $tag {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, "{}", self.xml)
            }
        }

        impl Default for $tag {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $tag {
            /// Creates a new $xml.
            pub fn new() -> Self {
                $tag {
                    xml: XmlTag::new($xml),
                }
            }

            /// Appends a tag.
            pub fn tag<T: Into<XmlTag>>(mut self, tag: T) -> Self {
                self.xml.add_tag(tag);
                self
            }

            /// Appends text.
            pub fn text<S: Into<String>>(mut self, text: S) -> Self {
                self.xml.add_text(text);
                self
            }

            /// Extracts the finished XmlTag.
            pub fn into_xmltag(self) -> XmlTag {
                self.xml
            }
        }
    };
}
