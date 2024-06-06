// 19.364 number:title
macro_rules! number_title {
    ($acc:ident) => {
        /// The number:title attribute specifies the title of a data style.
        pub fn set_title<S: Into<String>>(&mut self, title: S) {
            self.$acc.set_attr("number:title", title.into());
        }

        /// The number:title attribute specifies the title of a data style.
        pub fn title(&self) -> Option<&str> {
            self.$acc.attr("number:title")
        }
    };
}

// 19.351 number:language
// 19.342 number:country
// 19.361 number:script
macro_rules! number_locale {
    ($acc:ident) => {
        /// The number:language attribute specifies a language code. The country code is used for
        /// formatting properties whose evaluation is locale-dependent.
        /// If a language code is not specified, either the system settings or the setting for the system's
        /// language are used, depending on the property whose value should be evaluated.
        ///
        /// The number:country attribute specifies a country code for a data style. The country code is
        /// used for formatting properties whose evaluation is locale-dependent.
        /// If a country is not specified, the system settings are used.
        ///
        /// The number:script attribute specifies a script code. The script code is used for formatting
        /// properties whose evaluation is locale-dependent. The attribute should be used only if necessary
        /// according to the rules of §2.2.3 of [RFC5646](https://datatracker.ietf.org/doc/html/rfc5646), or its successors.
        pub fn set_locale(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.attr
                    .set_attr("number:language", locale.id.language.to_string());
                if let Some(region) = locale.id.region {
                    self.attr.set_attr("number:country", region.to_string());
                } else {
                    self.attr.clear_attr("number:country");
                }
                if let Some(script) = locale.id.script {
                    self.attr.set_attr("number:script", script.to_string());
                } else {
                    self.attr.clear_attr("number:script");
                }
            } else {
                self.attr.clear_attr("number:language");
                self.attr.clear_attr("number:country");
                self.attr.clear_attr("number:script");
            }
        }

        /// Returns number:language, number:country and number:script as a locale.
        pub fn locale(&self) -> Option<Locale> {
            if let Some(language) = self.attr.attr("number:language") {
                if let Ok(language) = Language::try_from_bytes(language.as_bytes()) {
                    let region = if let Some(region) = self.attr.attr("number:country") {
                        Region::try_from_bytes(region.as_bytes()).ok()
                    } else {
                        None
                    };
                    let script = if let Some(script) = self.attr.attr("number:script") {
                        Script::try_from_bytes(script.as_bytes()).ok()
                    } else {
                        None
                    };

                    let id = LanguageIdentifier::from((language, script, region));

                    Some(Locale::from(id))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

// 19.367 number:transliteration-language
// 19.365 number:transliteration-country
macro_rules! number_transliteration_locale {
    ($acc:ident) => {
        /// The number:transliteration-language attribute specifies a language code in
        /// conformance with [RFC5646](https://datatracker.ietf.org/doc/html/rfc5646).
        /// If no language/country (locale) combination is specified, the locale of the data style is used
        ///
        /// The number:transliteration-country attribute specifies a country code in conformance
        /// with [RFC5646](https://datatracker.ietf.org/doc/html/rfc5646).
        /// If no language/country (locale) combination is specified, the locale of the data style is used.
        pub fn set_transliteration_locale(&mut self, locale: Locale) {
            if locale != Locale::UND {
                self.attr.set_attr(
                    "number:transliteration-language",
                    locale.id.language.to_string(),
                );
                if let Some(region) = locale.id.region {
                    self.attr
                        .set_attr("number:transliteration-country", region.to_string());
                } else {
                    self.attr.clear_attr("number:transliteration-country");
                }
            } else {
                self.attr.clear_attr("number:transliteration-language");
                self.attr.clear_attr("number:transliteration-country");
            }
        }

        /// Returns number:transliteration_language and number:transliteration_country as a locale.
        pub fn transliteration_locale(&self) -> Option<Locale> {
            if let Some(language) = self.attr.attr("number:language") {
                if let Ok(language) = Language::try_from_bytes(language.as_bytes()) {
                    let region = if let Some(region) = self.attr.attr("number:country") {
                        Region::try_from_bytes(region.as_bytes()).ok()
                    } else {
                        None
                    };

                    let id = LanguageIdentifier::from((language, None, region));

                    Some(Locale::from(id))
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

// 19.366 number:transliteration-format
macro_rules! number_transliteration_format {
    ($acc:ident) => {
        /// The number:transliteration-format attribute specifies which number characters to use.
        ///
        /// The value of the number:transliteration-format attribute shall be a decimal "DIGIT ONE"
        /// character with numeric value 1 as listed in the Unicode Character Database file UnicodeData.txt
        /// with value 'Nd' (Numeric decimal digit) in the General_Category/Numeric_Type property field 6
        /// and value '1' in the Numeric_Value fields 7 and 8, respectively as listed in
        /// DerivedNumericValues.txt
        ///
        /// If no format is specified the default ASCII representation of Latin-Indic digits is used, other
        /// transliteration attributes present in that case are ignored.
        ///
        /// The default value for this attribute is 1
        pub fn set_transliteration_format(&mut self, format: char) {
            self.attr
                .set_attr("number:transliteration-format", format.to_string());
        }

        /// Transliteration format.
        pub fn transliteration_format(&self) -> Option<char> {
            match self.attr.attr("number:transliteration-format") {
                None => None,
                Some(v) => v.chars().next(),
            }
        }
    };
}

// 19.368 number:transliteration-style
macro_rules! number_transliteration_style {
    ($acc:ident) => {
        /// The number:transliteration-style attribute specifies the transliteration format of a
        /// number system.
        ///
        /// The semantics of the values of the number:transliteration-style attribute are locale- and
        /// implementation-dependent.
        ///
        /// The default value for this attribute is short.
        pub fn set_transliteration_style(&mut self, style: TransliterationStyle) {
            self.attr
                .set_attr("number:transliteration-style", style.to_string());
        }

        /// Transliteration style.
        pub fn transliteration_style(&self) -> Result<Option<TransliterationStyle>, OdsError> {
            TransliterationStyle::parse_attr(self.attr.attr("number:transliteration-style"))
        }
    };
}

// 19.340 number:automatic-order
macro_rules! number_automatic_order {
    ($acc:ident) => {
        /// The number:automatic-order attribute specifies whether data is ordered to match the default
        /// order for the language and country of a data style.
        /// The defined values for the number:automatic-order attribute are:
        /// * false: data is not ordered to match the default order for the language and country of a data
        /// style.
        /// * true: data is ordered to match the default order for the language and country of a data style.
        /// The default value for this attribute is false.
        ///
        /// This attribute is valid for ValueType::DateTime and ValueType::TimeDuration.
        pub fn set_automatic_order(&mut self, volatile: bool) {
            self.attr
                .set_attr("number:automatic-order", volatile.to_string());
        }

        /// Automatic order.
        pub fn automatic_order(&self) -> Option<bool> {
            if let Some(v) = self.attr.attr("number:automatic-order") {
                v.parse().ok()
            } else {
                None
            }
        }
    };
}

// 19.348 number:format-source
macro_rules! number_format_source {
    ($acc:ident) => {
        /// The number:format-source attribute specifies the source of definitions of the short and
        /// long display formats.
        ///
        /// The defined values for the number:format-source attribute are:
        /// * fixed: the values short and long of the number:style attribute are defined by this
        /// standard.
        /// * language: the meaning of the values long and short of the number:style attribute
        /// depend upon the number:language and number:country attributes of the date style. If
        /// neither of those attributes are specified, consumers should use their default locale for short
        /// and long date and time formats.
        ///
        /// The default value for this attribute is fixed.
        ///
        /// This attribute is valid for ValueType::DateTime and ValueType::TimeDuration.
        pub fn set_format_source(&mut self, source: FormatSource) {
            self.attr
                .set_attr("number:format-source", source.to_string());
        }

        /// The source of definitions of the short and long display formats.
        pub fn format_source(&mut self) -> Result<Option<FormatSource>, OdsError> {
            FormatSource::parse_attr(self.attr.attr("number:format-source"))
        }
    };
}

// 19.369 number:truncate-on-overflow
macro_rules! number_truncate_on_overflow {
    ($acc:ident) => {
        /// The number:truncate-on-overflow attribute specifies if a time or duration for which the
        /// value to be displayed by the largest time component specified in the style is too large to be
        /// displayed using the value range for number:hours 16.29.20 (0 to 23), or
        /// number:minutes 16.29.21 or number:seconds 16.29.22 (0 to 59) is truncated or if the
        /// value range of this component is extended. The largest time component is those for which a value
        /// of "1" represents the longest period of time.
        /// If a value gets truncated, then its value is displayed modulo 24 (for number:hours) or modulo
        /// 60 (for number:minutes and number:seconds).
        ///
        /// If the value range of a component get extended, then values larger than 23 or 59 are displayed.
        /// The defined values for the number:truncate-on-overflow element are:
        /// * false: the value range of the component is extended.
        /// * true: the value range of the component is not extended.
        ///
        /// The default value for this attribute is true.
        ///
        /// This attribute is valid for ValueType::TimeDuration.
        pub fn set_truncate_on_overflow(&mut self, truncate: bool) {
            self.attr
                .set_attr("number:truncate-on-overflow", truncate.to_string());
        }

        /// Truncate time-values on overflow.
        pub fn truncate_on_overflow(&mut self) -> Option<bool> {
            if let Some(v) = self.attr.attr("number:truncate-on-overflow") {
                v.parse().ok()
            } else {
                None
            }
        }
    };
}
