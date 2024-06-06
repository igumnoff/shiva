macro_rules! xlink_actuate {
    ($acc:ident) => {
        /// See §5.6.2 of XLink.
        pub fn set_xlink_actuate(&mut self, actuate: XLinkActuate) {
            self.$acc.set_attr("xlink:actuate", actuate.to_string());
        }
    };
}

macro_rules! xlink_href {
    ($acc:ident) => {
        /// The xlink:href 19.916 attribute specifies a remote resource. Its data type is anyIRI. See §5.4
        /// of XLink.
        pub fn set_xlink_href<S: Into<String>>(&mut self, href: S) {
            self.$acc.set_attr("xlink:href", href.into());
        }
    };
}

macro_rules! xlink_show {
    ($acc:ident) => {
        /// See §5.6.1 of XLink.
        pub fn set_xlink_show(&mut self, show: XLinkShow) {
            self.$acc.set_attr("xlink:show", show.to_string());
        }
    };
}

macro_rules! xlink_type {
    ($acc:ident) => {
        /// See §3.2 of XLink. This attribute always has the value 'simple' in OpenDocument document
        /// instances.
        pub fn set_xlink_type(&mut self, ty: XLinkType) {
            self.$acc.set_attr("xlink:type", ty.to_string());
        }
    };
}
