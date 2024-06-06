use crate::format::{
    FormatCalendarStyle, FormatNumberStyle, FormatPart, FormatPartType, ValueFormatTrait,
};
use icu_locid::Locale;

/// Builder for FormatPart with type Number.
#[derive(Debug)]
pub struct PartNumberBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartNumberBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Number),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// If the number:decimal-places attribute is not specified, the number of decimal places
    /// specified by the default table cell style is used.
    #[must_use]
    pub fn decimal_places(mut self, decimal_places: u8) -> Self {
        self.part
            .set_attr("number:decimal-places", decimal_places.to_string());
        self
    }

    /// Sets decimal_places and min_decimal_places to the same value,
    /// which in effect always displays the same number of decimals.
    #[must_use]
    pub fn fixed_decimal_places(mut self, decimal_places: u8) -> Self {
        self.part
            .set_attr("number:decimal-places", decimal_places.to_string());
        self.part
            .set_attr("number:min-decimal-places", decimal_places.to_string());
        self
    }

    /// The number:grouping attribute specifies whether the integer digits of a number should be
    /// grouped using a separator character.
    /// The grouping character that is used and the number of digits that are grouped together depends
    /// on the language and country of the style.
    /// The defined values for the number:grouping attribute are:
    /// * false: integer digits of a number are not grouped using a separator character.
    /// * true: integer digits of a number should be grouped by a separator character.
    /// The default value for this attribute is false.
    #[must_use]
    pub fn grouping(mut self) -> Self {
        self.part.set_attr("number:grouping", String::from("true"));
        self
    }

    /// The number:min-decimal-places attribute specifies the minimum number of digits in the
    /// decimal part.
    /// The value of the number:min-decimal-places attribute shall not be greater than the value of
    /// the number:decimal-places 19.343 attribute.
    /// If the value of number:min-decimal-places is less than the value of number:decimalplaces, trailing zero digits in decimal places following the position indicated by the value of
    /// number:min-decimal-places shall not be displayed.
    #[must_use]
    pub fn min_decimal_places(mut self, min_decimal_places: u8) -> Self {
        self.part
            .set_attr("number:min-decimal-places", min_decimal_places.to_string());
        self
    }

    /// The number:min-integer-digits attribute specifies the minimum number of integer digits to
    /// display in the integer portion of a number, a scientific number, or a fraction.
    /// For a number:fraction element, if the number:min-integer-digits attribute is not
    /// present, no integer portion is displayed.
    #[must_use]
    pub fn min_integer_digits(mut self, mininteger_digits: u8) -> Self {
        self.part
            .set_attr("number:min-integer-digits", mininteger_digits.to_string());
        self
    }

    /// The number:display-factor attribute specifies a factor by which each number is scaled
    /// (divided) before displaying.
    /// The default value for this attribute is 1
    #[must_use]
    pub fn display_factor(mut self, display_factor: f64) -> Self {
        self.part
            .set_attr("number:display-factor", display_factor.to_string());
        self
    }

    /// The number:decimal-replacement attribute specifies a replacement text for decimal places if
    /// a number style specifies that decimal places are used but the number displayed is an integer.
    /// Note: What replacement text is supported is implementation-dependent
    #[must_use]
    pub fn decimal_replacement(mut self, decimal_replacement: char) -> Self {
        self.part.set_attr(
            "number:decimal-replacement",
            decimal_replacement.to_string(),
        );
        self
    }

    /// The number:embedded-text element specifies text that is displayed at one specific position
    /// within a number.

    ///
    /// The number:embedded-text element is usable within the following element:
    /// * number:number 16.29.3.
    ///
    /// The number:embedded-text element has the following attribute:
    /// * number:position 19.358.
    ///
    /// The number:position attribute specifies the position where text appears.
    /// The index of a position starts with 1 and is counted by digits from right to left in the integer part of
    /// a number, starting left from a decimal separator if one exists, or from the last digit of the number.
    /// Text is inserted before the digit at the specified position. If the value of number:position
    /// attribute is greater than the value of number:min-integer-digits 19.355 and greater than
    /// the number of integer digits in the number, text is prepended to the number.    
    ///
    /// The number:embedded-text element has no child elements.
    /// The number:embedded-text element has character data content
    #[must_use]
    pub fn embedded_text<S: Into<String>>(mut self, text: S, pos: i32) -> Self {
        self.part.position = Some(pos);
        self.part.content = Some(text.into());
        self
    }
}

/// Builder for FormatPart with type Number.
#[derive(Debug)]
pub struct PartFillCharacterBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartFillCharacterBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::FillCharacter),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// If the number:decimal-places attribute is not specified, the number of decimal places
    /// specified by the default table cell style is used.
    #[must_use]
    pub fn fill_char(mut self, c: char) -> Self {
        self.part.set_content(c.to_string());
        self
    }
}

/// Builder for FormatPart with type ScientificNumber.
///
/// The number:scientific-number element specifies the display formatting properties for a
/// number style that should be displayed in scientific format.
///
/// The number:scientific-number element is usable within the following element:
/// * number:number-style 16.27.2.
///
/// The number:scientific-number element has the following attributes:
/// * number:decimal-places 19.343.4,
/// * number:grouping 19.348,
/// * number:min-exponentdigits 19.351 and
/// * number:min-integer-digits 19.352.
///
/// The number:scientific-number element has no child elements.
#[derive(Debug)]
pub struct PartScientificBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartScientificBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::ScientificNumber),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// If the number:decimal-places attribute is not specified, the number of decimal places
    /// specified by the default table cell style is used.
    #[must_use]
    pub fn decimal_places(mut self, v: u8) -> Self {
        self.part.set_attr("number:decimal-places", v.to_string());
        self
    }

    /// The number:exponent-interval attribute determines the valid exponents to be used: the
    /// valid exponents are the integer multiples of the value of the number:exponent-interval
    /// attribute.
    /// The default value for this attribute is 1.
    #[must_use]
    pub fn expontent_interval(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:exponent-interval", v.to_string());
        self
    }

    /// The number:forced-exponent-sign attribute specifies whether the sign of the exponent for a
    /// scientific number is always displayed.
    /// The defined values for the number:forced-exponent-sign attribute are:
    /// * false: the exponent sign is displayed only for a negative value of the exponent, otherwise it
    /// is not displayed.
    /// * true: the exponent sign is always displayed regardless of the value of exponent.
    /// The default value for this attribute is true.
    #[must_use]
    pub fn forced_exponent_sign(mut self, v: bool) -> Self {
        self.part
            .set_attr("number:forced-exponent-sign", v.to_string());
        self
    }

    /// The number:grouping attribute specifies whether the integer digits of a number should be
    /// grouped using a separator character.
    /// The grouping character that is used and the number of digits that are grouped together depends
    /// on the language and country of the style.
    /// The defined values for the number:grouping attribute are:
    /// * false: integer digits of a number are not grouped using a separator character.
    /// * true: integer digits of a number should be grouped by a separator character.
    /// The default value for this attribute is false.
    #[must_use]
    pub fn grouping(mut self) -> Self {
        self.part.set_attr("number:grouping", String::from("true"));
        self
    }

    /// The number:min-decimal-places attribute specifies the minimum number of digits in the
    /// decimal part.
    /// The value of the number:min-decimal-places attribute shall not be greater than the value of
    /// the number:decimal-places 19.343 attribute.
    /// If the value of number:min-decimal-places is less than the value of number:decimalplaces, trailing zero digits in decimal places following the position indicated by the value of
    /// number:min-decimal-places shall not be displayed.
    #[must_use]
    pub fn min_decimal_places(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-decimal-places", v.to_string());
        self
    }

    /// The number:min-exponent-digits attribute specifies the minimum number of digits to use to
    /// display an exponent.
    #[must_use]
    pub fn min_exponent_digits(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-exponent-digits", v.to_string());
        self
    }

    /// The number:min-integer-digits attribute specifies the minimum number of integer digits to
    /// display in the integer portion of a number, a scientific number, or a fraction.
    /// For a number:fraction element, if the number:min-integer-digits attribute is not
    /// present, no integer portion is displayed.
    #[must_use]
    pub fn min_integer_digits(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-integer-digits", v.to_string());
        self
    }
}

/// The number:fraction element specifies the display formatting properties for a number style
/// that should be displayed as a fraction.
///
/// The number:fraction element is usable within the following element:
/// * number:numberstyle 16.29.2.
///
/// The number:fraction element has the following attributes:
/// * number:denominatorvalue 19.345,
/// * number:grouping 19.350,
/// * number:max-denominator-value 19.352,
/// * number:min-denominator-digits 19.353,
/// * number:min-integer-digits 19.355 and
/// * number:min-numerator-digits 19.357.
///
/// The number:fraction element has no child elements.
#[derive(Debug)]
pub struct PartFractionBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartFractionBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Fraction),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:denominator-value attribute specifies an integer value that is used as the
    /// denominator of a fraction. If this attribute is not present, a denominator that is appropriate for
    /// displaying the number is used.
    #[must_use]
    pub fn denominator(mut self, v: i64) -> Self {
        self.part
            .set_attr("number:denominator-value", v.to_string());
        self
    }

    /// The number:grouping attribute specifies whether the integer digits of a number should be
    /// grouped using a separator character.
    /// The grouping character that is used and the number of digits that are grouped together depends
    /// on the language and country of the style.
    /// The defined values for the number:grouping attribute are:
    /// * false: integer digits of a number are not grouped using a separator character.
    /// * true: integer digits of a number should be grouped by a separator character.
    /// The default value for this attribute is false.
    #[must_use]
    pub fn grouping(mut self) -> Self {
        self.part.set_attr("number:grouping", String::from("true"));
        self
    }

    /// The number:max-denominator-value attribute specifies the maximum
    /// denominator permitted to be chosen if its number:fraction element does not
    /// have a number:denominator-value attribute. The number:max-denominator-value
    /// attribute is ignored in the presence of a number:denominator-value 19.345
    /// attribute. The absence of the number:max-denominator-value attribute indicates
    /// that no maximum denominator is specified.
    #[must_use]
    pub fn max_denominator(mut self, v: i64) -> Self {
        self.part
            .set_attr("number:max-denominator-value", v.to_string());
        self
    }

    /// The number:min-denominator-digits attribute specifies the minimum number of digits to
    /// use to display the denominator of a fraction.
    #[must_use]
    pub fn min_denominator_digits(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-denominator-digits", v.to_string());
        self
    }

    /// The number:min-integer-digits attribute specifies the minimum number of integer digits to
    /// display in the integer portion of a number, a scientific number, or a fraction.
    /// For a number:fraction element, if the number:min-integer-digits attribute is not
    /// present, no integer portion is displayed.
    #[must_use]
    pub fn min_integer_digits(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-numerator-digits", v.to_string());
        self
    }

    /// The number:min-numerator-digits attribute specifies the minimum number of digits to use
    /// to display the numerator in a fraction.
    #[must_use]
    pub fn min_numerator_digits(mut self, v: u8) -> Self {
        self.part
            .set_attr("number:min-numerator-digits", v.to_string());
        self
    }
}

/// The number:currency-symbol element specifies whether a currency symbol is displayed in
/// a currency style.
/// The content of this element is the text that is displayed as the currency symbol.
/// If the element is empty or contains white space characters only, the default currency
/// symbol for the currency style or the language and country of the currency style is displayed.
///
/// The number:currency-symbol element is usable within the following element:
/// * number:currency-style 16.27.7.
///
/// The number:currency-symbol element has the following attributes:
/// * number:country 19.342,
/// * number:language 19.349,
/// * number:rfc-language-tag 19.356 and
/// * number:script 19.357.
///
/// The number:currency-symbol element has no child elements.
/// The number:currency-symbol element has character data content.
#[derive(Debug)]
pub struct PartCurrencySymbolBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartCurrencySymbolBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::CurrencySymbol),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:language attribute specifies a language code. The country code is used for
    /// formatting properties whose evaluation is locale-dependent.
    /// If a language code is not specified, either the system settings or the setting for the system's
    /// language are used, depending on the property whose value should be evaluated.
    ///
    /// The number:country attribute specifies a country code for a data style. The country code is
    /// used for formatting properties whose evaluation is locale-dependent.
    /// If a country is not specified, the system settings are used.
    /// The number:country attribute on a number:currency-symbol element, specifies the
    /// country of a currency symbol.
    ///
    /// The number:script attribute specifies a script code. The script code is used for formatting
    /// properties whose evaluation is locale-dependent. The attribute should be used only if necessary
    /// according to the rules of §2.2.3 of [RFC5646](https://datatracker.ietf.org/doc/html/rfc5646), or its successors.
    #[must_use]
    pub fn locale(mut self, v: Locale) -> Self {
        self.part
            .set_attr("number:language", v.id.language.to_string());
        if let Some(region) = v.id.region {
            self.part.set_attr("number:country", region.to_string());
        }
        if let Some(script) = v.id.script {
            self.part.set_attr("number:script", script.to_string());
        }
        self
    }

    /// Symbol text that is used for the currency symbol. If not set
    /// the default according to the country is used.
    #[must_use]
    pub fn symbol<S: Into<String>>(mut self, v: S) -> Self {
        self.part.set_content(v.into());
        self
    }
}

/// The number:day element specifies a day of a month in a date.
///
/// The number:day element is usable within the following element:
/// * number:date-style 16.27.10.
///
/// The number:day element has the following attributes:
/// * number:calendar 19.341 and
/// * number:style 19.358.2.
///
/// The number:day element has no child elements.
#[derive(Debug)]
pub struct PartDayBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartDayBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Day),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// Builder for FormatPart with type Month.
///
/// The number:month element specifies a month in a date.
///
/// The number:month element is usable within the following element:
/// * number:date-style 16.27.10.
///
/// The number:month element has the following attributes:
/// number:calendar 19.341,
/// number:possessive-form 19.355,
/// number:style 19.358.7 and
/// number:textual 19.359.
///
/// The number:month element has no child elements
#[derive(Debug)]
pub struct PartMonthBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartMonthBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Month),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:textual attribute specifies whether the name or number of a month is displayed in
    /// the month portion of a date.
    /// The defined values for the number:textual element are:
    /// * false: the number of the month is displayed.
    /// * true: the name of the month is displayed.
    /// The name or number of a month is defined by the number:calendar 19.341 attribute on the
    /// same parent element as the number:textual attribute.
    /// The default value for this attribute is false.
    #[must_use]
    pub fn textual(mut self) -> Self {
        self.part.set_attr("number:textual", true.to_string());
        self
    }

    /// The number:possessive-form attribute specifies whether the month is displayed as a noun or
    /// using the possessive form.
    /// The number:possessive-form attribute is only applied when a number:textual 19.363
    /// attribute on the same number:month element has the value of true.
    /// The defined values for the number:possessive-form attribute are:
    /// * false: the name of the month is displayed in nominative form.
    /// * true: the name of the month is displayed in possessive form.
    #[must_use]
    pub fn possessive_form(mut self) -> Self {
        self.part
            .set_attr("number:possessive-form", true.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:year element specifies a year in a date.
/// The number:year element is usable within the following element:
/// * number:date-style 16.27.10.
///
/// The number:year element has the following attributes:
/// * number:calendar 19.341 and
/// * number:style 19.358.10.
///
/// The number:year element has no child elements.
#[derive(Debug)]
pub struct PartYearBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartYearBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Year),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:era element specifies an era in which a year is counted.
///
/// The number:era element is usable within the following element:
/// * number:date-style 16.27.10.
///
/// The number:era element has the following attributes:
/// * number:calendar 19.341 and
/// * number:style 19.358.4.
///
/// The number:era element has no child elements
#[derive(Debug)]
pub struct PartEraBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartEraBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Era),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:day-of-week element specifies a day of a week in a date.
///
/// The number:day-of-week element is usable within the following element:
/// * number:datestyle 16.27.10.
///
/// The number:day-of-week element has the following attributes:
/// * number:calendar 19.341 and
/// * number:style 19.358.3.
///
/// The number:day-of-week element has no child elements.
#[derive(Debug)]
pub struct PartDayOfWeekBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartDayOfWeekBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::DayOfWeek),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:week-of-year element specifies a week of a year in a date.
///
/// The number:week-of-year element is usable within the following element:
/// * number:date-style 16.27.10.
///
/// The number:week-of-year element has the following attribute:
/// * number:calendar 19.341.
///
/// The number:week-of-year element has no child elements.
#[derive(Debug)]
pub struct PartWeekOfYearBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartWeekOfYearBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::WeekOfYear),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:quarter element specifies a quarter of the year in a date.
///
/// The number:quarter element is usable within the following element:
/// * number:datestyle 16.27.10.
///
/// The number:quarter element has the following attributes:
/// * number:calendar 19.341 and
/// * number:style 19.358.8.
///
/// The number:quarter element has no child elements
#[derive(Debug)]
pub struct PartQuarterBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartQuarterBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Quarter),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }

    /// The number:calendar attribute specifies the calendar system used to extract parts of a date.
    ///
    /// The attribute value may also be a string value. If this attribute is not specified, the default calendar
    /// system for the locale of the data style is used.
    #[must_use]
    pub fn calendar(mut self, calendar: FormatCalendarStyle) -> Self {
        self.part.set_attr("number:calendar", calendar.to_string());
        self
    }
}

/// The number:hours element specifies whether hours are displayed as part of a date or time.
///
/// The number:hours element is usable within the following elements:
/// * number:datestyle 16.27.10 and
/// * number:time-style 16.27.18.
///
/// The number:hours element has the following attribute:
/// * number:style 19.358.5.
///
/// The number:hours element has no child elements.
#[derive(Debug)]
pub struct PartHoursBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartHoursBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Hours),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }
}

/// The number:minutes element specifies whether minutes are displayed as part of a date or
/// time.
/// The number:minutes element is usable within the following elements:
/// * number:datestyle 16.27.10 and
/// * number:time-style 16.27.18.
///
/// The number:minutes element has the following attribute:
/// * number:style 19.358.6.
///
/// The number:minutes element has no child elements.
#[derive(Debug)]
pub struct PartMinutesBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartMinutesBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Minutes),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }
}

/// The number:seconds element specifies whether seconds are displayed as part of a date or
/// time.
///
/// The number:seconds element is usable within the following elements:
/// * number:datestyle 16.27.10 and
/// * number:time-style 16.27.18.
///
/// The number:seconds element has the following attributes:
/// * number:decimal-places 19.343.3 and
/// * number:style 19.358.9.
///
/// The number:seconds element has no child elements.
#[derive(Debug)]
pub struct PartSecondsBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartSecondsBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Seconds),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// If the number:decimal-places attribute is not specified, the number of decimal places
    /// specified by the default table cell style is used.
    #[must_use]
    pub fn decimal_places(mut self, decimal_places: u8) -> Self {
        self.part
            .set_attr("number:decimal-places", decimal_places.to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn long_style(mut self) -> Self {
        self.part.set_attr("number:style", "long".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn short_style(mut self) -> Self {
        self.part.set_attr("number:style", "short".to_string());
        self
    }

    /// The number:style attribute specifies whether the content of a time element is displayed in short
    /// or long format. The value of this attribute can be short or long. The meaning of these values
    /// depends on the value of the number:format-source 19.348 attribute that is attached to a date
    /// or time style.
    #[must_use]
    pub fn style(mut self, style: FormatNumberStyle) -> Self {
        self.part.set_attr("number:style", style.to_string());
        self
    }
}

/// Adds a format part to this format.
///
/// The number:am-pm element specifies whether AM/PM is included as part of a date or time.
/// If a number:am-pm element is contained in a date or time style, hours are displayed using
/// values from 1 to 12 only.
///
/// Can be used with ValueTypes:
/// * DateTime
/// * TimeDuration
#[derive(Debug)]
pub struct PartAmPmBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartAmPmBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::AmPm),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }
}

/// Adds a format part to this format.
///
/// The number:boolean element marks the position of the Boolean value of a Boolean style.
///
/// Can be used with ValueTypes:
/// * Boolean
#[derive(Debug)]
pub struct PartBooleanBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartBooleanBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Boolean),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }
}

/// Adds a format part to this format.
///
/// The number:text element contains any fixed text for a data style.
///
/// Can be used with ValueTypes:
/// * Boolean
/// * Currency
/// * DateTime
/// * Number
/// * Percentage
/// * Text
/// * TimeDuration
#[derive(Debug)]
pub struct PartTextBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartTextBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::Text),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }

    /// Only applies the builder if the test is true.
    #[must_use]
    pub fn if_then<F>(self, test: bool, build: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        if test {
            build(self)
        } else {
            self
        }
    }

    /// Sets the text value.
    #[must_use]
    pub fn text<S: Into<String>>(mut self, txt: S) -> Self {
        self.part.set_content(txt.into());
        self
    }
}

/// Adds a format part to this format.
///    
/// The number:text-content element marks the position of variable text content of a text
/// style.
///
/// Can be used with ValueTypes:
/// * Text

#[derive(Debug)]
pub struct PartTextContentBuilder<'vf, T: ValueFormatTrait> {
    part: FormatPart,
    valueformat: &'vf mut T,
}

impl<'vf, T: ValueFormatTrait> PartTextContentBuilder<'vf, T> {
    /// New builder for the valueformat.
    pub fn new<'a>(valueformat: &'a mut T) -> Self
    where
        'a: 'vf,
    {
        Self {
            part: FormatPart::new(FormatPartType::TextContent),
            valueformat,
        }
    }

    /// Appends the constructed FormatPart to the original value format.
    pub fn build(self) {
        self.valueformat.push_part(self.part);
    }
}
