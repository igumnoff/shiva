macro_rules! text_condition {
    ($acc:ident) => {
        /// The text:condition attribute specifies the display of text.
        /// The defined value of the text:condition attribute is none, which means text is hidden.
        pub fn set_text_condition(&mut self, cond: TextCondition) {
            self.$acc.set_attr("text:condition", cond.to_string());
        }
    };
}

macro_rules! text_display {
    ($acc:ident) => {
        /// The text:display attribute specifies whether text is hidden.
        /// The defined values for the text:display attribute are:
        /// * condition: text is hidden under the condition specified in the text:condition 20.426
        /// attribute.
        /// * none: text is hidden unconditionally.
        /// * true: text is displayed. This is the default setting
        pub fn set_display(&mut self, cond: TextDisplay) {
            self.$acc.set_attr("text:display", cond.to_string());
        }
    };
}
