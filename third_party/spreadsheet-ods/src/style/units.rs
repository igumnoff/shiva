//!
//! All kinds of units for use in style attributes.
//!

use crate::style::ParseStyleAttr;
use crate::OdsError;
use get_size::GetSize;
use get_size_derive::GetSize;
use std::fmt::{Display, Formatter};

/// An angle, as defined in §4.1 of SVG, is a double value that may be followed immediately by one
/// of the following angle unit identifiers: deg (degrees), grad (gradiants) or rad (radians). If no unit
/// identifier is specified, the value is assumed to be in degrees.
/// Note: OpenDocument v1.1 did not support angle specifications that contain an angle unit
/// identifier. Angle unit identifiers should be omitted for compatibility with OpenDocument v1.1
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Angle {
    /// Degrees
    Deg(f64),
    /// Grad degrees.
    Grad(f64),
    /// Radiant.
    Rad(f64),
}

impl Display for Angle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Angle::Deg(v) => write!(f, "{}deg", v),
            Angle::Grad(v) => write!(f, "{}grad", v),
            Angle::Rad(v) => write!(f, "{}rad", v),
        }
    }
}

/// A (positive or negative) length, consisting of magnitude and unit, in conformance with the Units of
/// Measure defined in §5.9.13 of XSL.
#[derive(Debug, Clone, Copy, PartialEq, Default, GetSize)]
pub enum Length {
    /// Unspecified length, the actual value is some default or whatever.
    #[default]
    Default,
    /// cm
    Cm(f64),
    /// mm
    Mm(f64),
    /// inch
    In(f64),
    /// typographic points
    Pt(f64),
    /// pica
    Pc(f64),
    /// em
    Em(f64),
}

impl Length {
    /// Is the length positive.
    pub fn is_positive(&self) -> bool {
        0f64 <= match self {
            Length::Default => 0f64,
            Length::Cm(v) => *v,
            Length::Mm(v) => *v,
            Length::In(v) => *v,
            Length::Pt(v) => *v,
            Length::Pc(v) => *v,
            Length::Em(v) => *v,
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Length::Cm(v) => write!(f, "{}cm", v),
            Length::Mm(v) => write!(f, "{}mm", v),
            Length::In(v) => write!(f, "{}in", v),
            Length::Pt(v) => write!(f, "{}pt", v),
            Length::Pc(v) => write!(f, "{}pc", v),
            Length::Em(v) => write!(f, "{}em", v),
            Length::Default => write!(f, ""),
        }
    }
}

impl ParseStyleAttr<Length> for Length {
    fn parse_attr(attr: Option<&str>) -> Result<Option<Length>, OdsError> {
        if let Some(s) = attr {
            if s.ends_with("cm") {
                Ok(Some(Length::Cm(s.split_at(s.len() - 2).0.parse()?)))
            } else if s.ends_with("mm") {
                Ok(Some(Length::Mm(s.split_at(s.len() - 2).0.parse()?)))
            } else if s.ends_with("in") {
                Ok(Some(Length::In(s.split_at(s.len() - 2).0.parse()?)))
            } else if s.ends_with("pt") {
                Ok(Some(Length::Pt(s.split_at(s.len() - 2).0.parse()?)))
            } else if s.ends_with("pc") {
                Ok(Some(Length::Pc(s.split_at(s.len() - 2).0.parse()?)))
            } else if s.ends_with("em") {
                Ok(Some(Length::Em(s.split_at(s.len() - 2).0.parse()?)))
            } else {
                Err(OdsError::Parse("invalid length", Some(s.to_string())))
            }
        } else {
            Ok(None)
        }
    }
}

/// (Positive or negative) percentage values in conformance with §5.9.11 of XSL.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Percent {
    /// Percentage
    Percent(f64),
}

impl Display for Percent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Percent::Percent(v) => write!(f, "{}%", v),
        }
    }
}

/// Length or percentage.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum LengthPercent {
    Length(Length),
    Percent(Percent),
}

impl From<Length> for LengthPercent {
    fn from(value: Length) -> Self {
        LengthPercent::Length(value)
    }
}

impl From<Percent> for LengthPercent {
    fn from(value: Percent) -> Self {
        LengthPercent::Percent(value)
    }
}

impl Display for LengthPercent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LengthPercent::Length(v) => write!(f, "{}", v),
            LengthPercent::Percent(v) => write!(f, "{}", v),
        }
    }
}

/// 19.348 number:format-source
///
/// The number:format-source attribute specifies the source of definitions of the short and
/// long display formats.
///
/// The defined values for the number:format-source attribute are:
/// • fixed: the values short and long of the number:style attribute are defined by this
/// standard.
/// • language: the meaning of the values long and short of the number:style attribute
/// depend upon the number:language and number:country attributes of the date style. If
/// neither of those attributes are specified, consumers should use their default locale for short
/// and long date and time formats.
///
/// The default value for this attribute is fixed.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FormatSource {
    Fixed,
    Language,
}

impl Display for FormatSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatSource::Fixed => write!(f, "fixed"),
            FormatSource::Language => write!(f, "language"),
        }
    }
}

impl ParseStyleAttr<FormatSource> for FormatSource {
    fn parse_attr(attr: Option<&str>) -> Result<Option<FormatSource>, OdsError> {
        if let Some(attr) = attr {
            match attr {
                "fixed" => Ok(Some(FormatSource::Fixed)),
                "language" => Ok(Some(FormatSource::Language)),
                _ => Err(OdsError::Parse(
                    "invalid format source",
                    Some(attr.to_string()),
                )),
            }
        } else {
            Ok(None)
        }
    }
}

/// 19.368 number:transliteration-style
///
/// The number:transliteration-style attribute specifies the transliteration format of a
/// number system.
///
/// The semantics of the values of the number:transliteration-style attribute are locale- and
/// implementation-dependent.
///
/// The default value for this attribute is short.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TransliterationStyle {
    Short,
    Medium,
    Long,
}

impl Display for TransliterationStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransliterationStyle::Short => write!(f, "short"),
            TransliterationStyle::Medium => write!(f, "medium"),
            TransliterationStyle::Long => write!(f, "long"),
        }
    }
}

impl ParseStyleAttr<TransliterationStyle> for TransliterationStyle {
    fn parse_attr(attr: Option<&str>) -> Result<Option<TransliterationStyle>, OdsError> {
        if let Some(attr) = attr {
            match attr {
                "short" => Ok(Some(TransliterationStyle::Short)),
                "medium" => Ok(Some(TransliterationStyle::Medium)),
                "long" => Ok(Some(TransliterationStyle::Long)),
                _ => Err(OdsError::Parse(
                    "invalid number:transliteration-style",
                    Some(attr.to_string()),
                )),
            }
        } else {
            Ok(None)
        }
    }
}

/// 19.484 style:font-family-generic
///
/// The style:font-family-generic attribute specifies a generic font family name.
///
/// The defined values for the style:font-family-generic attribute are:
/// • decorative: the family of decorative fonts.
/// • modern: the family of modern fonts.
/// • roman: the family roman fonts (with serifs).
/// • script: the family of script fonts.
/// • swiss: the family roman fonts (without serifs).
/// • system: the family system fonts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum FontFamilyGeneric {
    Decorative,
    Modern,
    Roman,
    Script,
    Swiss,
    System,
}

impl Display for FontFamilyGeneric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontFamilyGeneric::Decorative => write!(f, "decorative"),
            FontFamilyGeneric::Modern => write!(f, "modern"),
            FontFamilyGeneric::Roman => write!(f, "roman"),
            FontFamilyGeneric::Script => write!(f, "script"),
            FontFamilyGeneric::Swiss => write!(f, "swiss"),
            FontFamilyGeneric::System => write!(f, "system"),
        }
    }
}

/// 19.485 style:font-pitch
/// The style:font-pitch attribute specifies whether a font has a fixed or variable width.
/// The defined values for the style:font-pitch attribute are:
/// * fixed: font has a fixed width.
/// * variable: font has a variable width.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FontPitch {
    /// Variable font with
    Variable,
    /// Fixed font width
    Fixed,
}

impl Display for FontPitch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FontPitch::Variable => write!(f, "variable"),
            FontPitch::Fixed => write!(f, "fixed"),
        }
    }
}

/// 19.509 style:page-usage
///
/// The style:page-usage attribute specifies the type of pages that a master page should
/// generate.
///
/// The defined values for the style:page-usage attribute are:
/// • all: if there are no <style:header-left> or <style:footer-left> elements, the
/// header and footer content is the same for left and right pages.
/// • left: <style:header-right> and <style:footer-right> elements are ignored.
/// • mirrored: if there are no <style:header-left> or <style:footer-left> elements,
/// the header and footer content is the same for left and right pages.
/// • right: <style:header-left> and <style:footer-left> elements are ignored.
///
/// The default value for this attribute is all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum MasterPageUsage {
    All,
    Left,
    Mirrored,
    Right,
}

impl Display for MasterPageUsage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MasterPageUsage::All => write!(f, "all"),
            MasterPageUsage::Left => write!(f, "left"),
            MasterPageUsage::Mirrored => write!(f, "mirrored"),
            MasterPageUsage::Right => write!(f, "right"),
        }
    }
}

impl ParseStyleAttr<MasterPageUsage> for MasterPageUsage {
    fn parse_attr(attr: Option<&str>) -> Result<Option<MasterPageUsage>, OdsError> {
        if let Some(attr) = attr {
            match attr {
                "all" => Ok(Some(MasterPageUsage::All)),
                "left" => Ok(Some(MasterPageUsage::Left)),
                "mirrored" => Ok(Some(MasterPageUsage::Mirrored)),
                "right" => Ok(Some(MasterPageUsage::Right)),
                _ => Err(OdsError::Parse(
                    "invalid style:page-usage",
                    Some(attr.to_string()),
                )),
            }
        } else {
            Ok(None)
        }
    }
}

/// 19.519 style:type
///
/// The style:type attribute specifies the type of a tab stop within paragraph formatting properties.
///
/// The defined values for the style:type attribute are:
/// • center: text is centered on a tab stop.
/// • char: character appears at a tab stop position.
/// • left: text is left aligned with a tab stop.
/// • right: text is right aligned with a tab stop.
///
/// For a <style:tab-stop> 17.8 element the default value for this attribute is left.
#[derive(Clone, Copy, Debug)]
#[allow(missing_docs)]
pub enum TabStopType {
    Center,
    Left,
    Right,
    Char,
}

impl Display for TabStopType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TabStopType::Center => write!(f, "center"),
            TabStopType::Left => write!(f, "left"),
            TabStopType::Right => write!(f, "right"),
            TabStopType::Char => write!(f, "char"),
        }
    }
}

impl Default for TabStopType {
    fn default() -> Self {
        Self::Left
    }
}

/// 19.534 svg:font-stretch
///
/// See §20.8.3 of SVG.
///
/// The svg:font-stretch attribute is usable with the following element: <style:font-face>
/// 16.23.
///
/// The values of the svg:font-stretch attribute are normal, ultra-condensed, extracondensed,
/// condensed, semi-condensed, semi-expanded, expanded, extraexpanded or ultra-expanded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum FontStretch {
    Normal,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Display for FontStretch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FontStretch::Normal => write!(f, "normal"),
            FontStretch::UltraCondensed => write!(f, "ultra-condensed"),
            FontStretch::ExtraCondensed => write!(f, "extra-condensed"),
            FontStretch::Condensed => write!(f, "condensed"),
            FontStretch::SemiCondensed => write!(f, "semi-condensed"),
            FontStretch::SemiExpanded => write!(f, "semi-expanded"),
            FontStretch::Expanded => write!(f, "expanded"),
            FontStretch::ExtraExpanded => write!(f, "extra-expanded"),
            FontStretch::UltraExpanded => write!(f, "ultra-expanded"),
        }
    }
}

/// 20.183 fo-border Properties.
/// See §7.29.3ff of XSL
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Border {
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl Display for Border {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Border::None => write!(f, "none"),
            Border::Hidden => write!(f, "hidden"),
            Border::Dotted => write!(f, "dotted"),
            Border::Dashed => write!(f, "dashed"),
            Border::Solid => write!(f, "solid"),
            Border::Double => write!(f, "double"),
            Border::Groove => write!(f, "groove"),
            Border::Ridge => write!(f, "ridge"),
            Border::Inset => write!(f, "inset"),
            Border::Outset => write!(f, "outset"),
        }
    }
}

/// 20.184 fo:break-after, fo:break-before
/// See §7.19.1 of XSL. The values odd-page and even-page are not supported.
///
/// This attribute shall not be used at the same time as fo:break-before.
///
/// In the OpenDocument XSL-compatible namespace, the fo:break-after attribute does not
/// support even-page, inherit and odd-page values.
///
/// The values of the fo:break-after attribute are auto, column or page.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PageBreak {
    Auto,
    Column,
    Page,
}

impl Display for PageBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            PageBreak::Auto => write!(f, "auto")?,
            PageBreak::Column => write!(f, "column")?,
            PageBreak::Page => write!(f, "page")?,
        }
        Ok(())
    }
}

/// 20.190 fo:font-size
///
/// See §7.8.4 of XSL.
///
/// The value of this attribute is either an absolute length or a percentage as described in §7.8.4 of
/// XSL. In contrast to XSL, percentage values can be used within common styles only and are
/// based on the font height of the parent style rather than to the font height of the attributes
/// neighborhood. Absolute font heights and relative font heights are not supported.
///
/// In the OpenDocument XSL-compatible namespace, the fo:font-size attribute does not
/// support absolute-size, inherit and relative-size values.
///
/// The values of the fo:font-size attribute are a value of type positiveLength 18.3.26 or a
/// value of type percent 18.3.23.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum FontSize {
    Length(Length),
    Percent(Percent),
}

impl FontSize {
    /// Is the fontsize positive. Percentage is always positive.
    pub fn is_positive(&self) -> bool {
        match self {
            FontSize::Length(v) => v.is_positive(),
            FontSize::Percent(_) => true,
        }
    }
}

impl From<Length> for FontSize {
    fn from(value: Length) -> Self {
        FontSize::Length(value)
    }
}

impl From<Percent> for FontSize {
    fn from(value: Percent) -> Self {
        FontSize::Percent(value)
    }
}

impl Display for FontSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FontSize::Percent(v) => write!(f, "{}", v),
            FontSize::Length(v) => write!(f, "{}", v),
        }
    }
}

/// 20.191 fo:font-style
/// See §7.8.7 of XSL.
///
/// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
///
/// In the OpenDocument XSL-compatible namespace, the fo:font-style attribute does not
/// support backslant and inherit values.
///
/// The values of the fo:font-style attribute are normal, italic or oblique.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Display for FontStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FontStyle::Normal => write!(f, "normal"),
            FontStyle::Italic => write!(f, "italic"),
            FontStyle::Oblique => write!(f, "oblique"),
        }
    }
}

/// 20.192 fo:font-variant
///
/// See §7.8.8 of XSL.
///
/// In the OpenDocument XSL-compatible namespace, the fo:font-variant attribute does not
/// support the inherit value.
///
/// The values of the fo:font-variant attribute are normal or small-caps.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FontVariant {
    Normal,
    SmallCaps,
}

impl Display for FontVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FontVariant::Normal => write!(f, "normal"),
            FontVariant::SmallCaps => write!(f, "small-caps"),
        }
    }
}

/// 20.193 fo:font-weight
///
/// See §7.8.9 of XSL.
///
/// This attribute is evaluated for any UNICODE character whose script type is latin. 20.358
/// In the OpenDocument XSL-compatible namespace, the fo:font-weight attribute does not
/// support bolder, inherit and lighter values.
///
/// The values of the fo:font-weight attribute are normal, bold, 100, 200, 300, 400, 500,
/// 600, 700, 800 or 900.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum FontWeight {
    Normal,
    Bold,
    W100,
    W200,
    W300,
    W400,
    W500,
    W600,
    W700,
    W800,
    W900,
}

impl Display for FontWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            FontWeight::Normal => write!(f, "normal"),
            FontWeight::Bold => write!(f, "bold"),
            FontWeight::W100 => write!(f, "100"),
            FontWeight::W200 => write!(f, "200"),
            FontWeight::W300 => write!(f, "300"),
            FontWeight::W400 => write!(f, "400"),
            FontWeight::W500 => write!(f, "500"),
            FontWeight::W600 => write!(f, "600"),
            FontWeight::W700 => write!(f, "700"),
            FontWeight::W800 => write!(f, "800"),
            FontWeight::W900 => write!(f, "900"),
        }
    }
}

/// 20.196 fo:hyphenation-keep
///
/// See §7.15.1 of XSL.
///  
/// The values of the fo:hyphenation-keep attribute are auto or page
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Hyphenation {
    Auto,
    Page,
}

impl Display for Hyphenation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Hyphenation::Auto => write!(f, "auto"),
            Hyphenation::Page => write!(f, "page"),
        }
    }
}

/// 20.197 fo:hyphenation-ladder-count
///
/// See §7.15.2 of XSL.
///
/// The defined values for the fo:hyphenation-ladder-count attribute are:
/// • no-limit:
/// • a value of type positiveInteger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum HyphenationLadderCount {
    NoLimit,
    Count(u32),
}

impl Display for HyphenationLadderCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HyphenationLadderCount::NoLimit => write!(f, "no_limit"),
            HyphenationLadderCount::Count(c) => c.fmt(f),
        }
    }
}

/// 20.200 fo:keep-together and fo:keep-with-next
/// See §7.19.3 of XSL.
/// In the OpenDocument XSL-compatible namespace, the fo:keep-together attribute does not
/// support the integer value.
/// The values of the fo:keep-together attribute are auto or always.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextKeep {
    Auto,
    Always,
}

impl Display for TextKeep {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextKeep::Auto => write!(f, "auto")?,
            TextKeep::Always => write!(f, "always")?,
        }
        Ok(())
    }
}

/// 20.203 fo:letter-spacing
///
/// See §7.16.2 of XSL.
///
/// In the OpenDocument XSL-compatible namespace, the fo:letter-spacing attribute does not
/// support the inherit and space values.
///
/// The defined value for the fo:letter-spacing attribute is a value of type length 18.3.18.
///
/// The values of the fo:letter-spacing attribute are a value of type length 18.3.18 or
/// normal.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum LetterSpacing {
    Normal,
    Length(Length),
}

impl From<Length> for LetterSpacing {
    fn from(value: Length) -> Self {
        LetterSpacing::Length(value)
    }
}

impl Display for LetterSpacing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LetterSpacing::Normal => write!(f, "normal"),
            LetterSpacing::Length(v) => write!(f, "{}", v),
        }
    }
}

/// 20.204 fo:line-height
///
/// See §7.15.4 of XSL.
///
/// The value normal activates the default line height calculation. The value of this attribute
/// can be a length, a percentage, normal.
///
/// In the OpenDocument XSL-compatible namespace, the fo:line-height attribute does not
/// support the inherit, number, and space values.
///
/// The defined values for the fo:line-height attribute are:
/// • a value of type nonNegativeLength 18.3.20
/// • normal: disables the effects of style:line-height-at-least 20.317 and
/// style:line-spacing 20.318.
/// • a value of type percent 18.3.23
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum LineHeight {
    Normal,
    Length(Length),
    Percent(Percent),
}

impl LineHeight {
    /// Is the fontsize positive. Percentage is always positive.
    pub fn is_positive(&self) -> bool {
        match self {
            LineHeight::Normal => true,
            LineHeight::Length(v) => v.is_positive(),
            LineHeight::Percent(_) => true,
        }
    }
}

impl From<Length> for LineHeight {
    fn from(value: Length) -> Self {
        LineHeight::Length(value)
    }
}

impl From<Percent> for LineHeight {
    fn from(value: Percent) -> Self {
        LineHeight::Percent(value)
    }
}

impl Display for LineHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LineHeight::Normal => write!(f, "normal"),
            LineHeight::Length(v) => v.fmt(f),
            LineHeight::Percent(v) => v.fmt(f),
        }
    }
}

/// 20.205 fo:margin
///
/// See §7.29.14 of XSL.
///
/// In the OpenDocument XSL-compatible namespace, the fo:margin attribute does not support
/// auto and inherit values.
///
/// The values of the fo:margin attribute are a value of type nonNegativeLength 18.3.20 or a
/// value of type percent 18.3.23.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Margin {
    Length(Length),
    Percent(Percent),
}

impl Margin {
    /// Is the fontsize positive. Percentage is always positive.
    pub fn is_positive(&self) -> bool {
        match self {
            Margin::Length(v) => v.is_positive(),
            Margin::Percent(_) => true,
        }
    }
}

impl From<Length> for Margin {
    fn from(value: Length) -> Self {
        Margin::Length(value)
    }
}

impl From<Percent> for Margin {
    fn from(value: Percent) -> Self {
        Margin::Percent(value)
    }
}

impl Display for Margin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Margin::Length(v) => v.fmt(f),
            Margin::Percent(v) => v.fmt(f),
        }
    }
}

/// 20.223 fo:text-align
///
/// See §7.15.9 of XSL.
///
/// If there are no values specified for the fo:text-align and style:justify-single-word
/// 20.301 attributes within the same formatting properties element, the values of those attributes is
/// set to start and false respectively.
/// In the OpenDocument XSL-compatible namespace, the fo:text-align attribute does not
/// support the inherit, inside, outside, or string values.
///
/// The values of the fo:text-align attribute are start, end, left, right, center or
/// justify.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextAlign {
    Start,
    Center,
    End,
    Justify,
    Inside,
    Outside,
    Left,
    Right,
}

impl Display for TextAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextAlign::Start => write!(f, "start"),
            TextAlign::Center => write!(f, "center"),
            TextAlign::End => write!(f, "end"),
            TextAlign::Justify => write!(f, "justify"),
            TextAlign::Inside => write!(f, "inside"),
            TextAlign::Outside => write!(f, "outside"),
            TextAlign::Left => write!(f, "left"),
            TextAlign::Right => write!(f, "right"),
        }
    }
}

/// 20.224 fo:text-align-last
///
/// See §7.15.10 of XSL.
///
/// This attribute is ignored if it not accompanied by an fo:text-align 20.223 attribute.
/// If no value is specified for this attribute, the value is set to start.
///
/// The values of the fo:text-align-last attribute are start, center or justify.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextAlignLast {
    Start,
    Center,
    Justify,
}

impl Display for TextAlignLast {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextAlignLast::Start => write!(f, "start"),
            TextAlignLast::Center => write!(f, "center"),
            TextAlignLast::Justify => write!(f, "justify"),
        }
    }
}

/// 20.225 fo:text-indent
///
/// The fo:text-indent attribute specifies a positive or negative indent for the first line of a
/// paragraph.
///
/// See §7.15.11 of XSL.
///
/// The attribute value is a length. If the attribute is contained in a
/// common style, the attribute value may be also a percentage that refers to the corresponding text
/// indent of a parent style.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Indent {
    Length(Length),
    Percent(Percent),
}

impl From<Length> for Indent {
    fn from(value: Length) -> Self {
        Indent::Length(value)
    }
}

impl From<Percent> for Indent {
    fn from(value: Percent) -> Self {
        Indent::Percent(value)
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Indent::Length(v) => v.fmt(f),
            Indent::Percent(v) => v.fmt(f),
        }
    }
}

/// 20.227 fo:text-transform
///
/// See §7.16.6 of XSL.
///  
/// If fo:text-transform and fo:font-variant 20.192 attributes are used simultaneously and
/// have different values than normal and none, the result is undefined.
///
/// Note: In consumers, the fo:text-transform and fo:font-variant
/// attributes are mutually exclusive.
///
/// The values of the fo:text-transform attribute are none, lowercase, uppercase or
/// capitalize.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextTransform {
    None,
    Lowercase,
    Uppercase,
    Capitalize,
}

impl Display for TextTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextTransform::None => write!(f, "none"),
            TextTransform::Lowercase => write!(f, "lowercase"),
            TextTransform::Uppercase => write!(f, "uppercase"),
            TextTransform::Capitalize => write!(f, "capitalize"),
        }
    }
}

/// 20.230 fo:wrap-option
/// See §7.15.13 of XSL.
///
/// If wrapping is disabled, it is implementation-defined whether the overflow text is visible or hidden.
/// If the text is hidden consumers may support a scrolling to access the text.
///
/// The values of the fo:wrap-option attribute are no-wrap or wrap.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WrapOption {
    NoWrap,
    Wrap,
}

impl Display for WrapOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WrapOption::NoWrap => write!(f, "no-wrap"),
            WrapOption::Wrap => write!(f, "wrap"),
        }
    }
}

/// 20.253 style:cell-protect
///
/// The style:cell-protect attribute specifies how a cell is protected.
/// This attribute is only evaluated if the current table is protected.
///
/// The defined values for the style:cell-protect attribute are:
/// • formula-hidden: if cell content is a formula, it is not displayed. It can be replaced by
/// changing the cell content.
/// Note: Replacement of cell content includes replacement with another formula or
/// other cell content.
/// • hidden-and-protected: cell content is not displayed and cannot be edited. If content is a
/// formula, the formula result is not displayed.
/// • none: formula responsible for cell content is neither hidden nor protected.
/// • protected: cell content cannot be edited.
/// • protected formula-hidden: cell content cannot be edited. If content is a formula, it is not
/// displayed. A formula result is displayed.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum CellProtect {
    /// If cell content is a formula, it is not displayed. It can be replaced by
    /// changing the cell content.
    /// Note: Replacement of cell content includes replacement with another formula or
    /// other cell content.
    FormulaHidden,
    /// cell content is not displayed and cannot be edited. If content is a
    /// formula, the formula result is not displayed.
    HiddenAndProtected,
    /// Formula responsible for cell content is neither hidden nor protected.
    None,
    /// Cell content cannot be edited.
    Protected,
    /// cell content cannot be edited. If content is a formula, it is not
    /// displayed.
    ProtectedFormulaHidden,
}

impl Display for CellProtect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CellProtect::FormulaHidden => write!(f, "formula-hidden"),
            CellProtect::HiddenAndProtected => write!(f, "hidden-and-protected"),
            CellProtect::None => write!(f, "none"),
            CellProtect::Protected => write!(f, "protected"),
            CellProtect::ProtectedFormulaHidden => write!(f, "protected formula-hidden"),
        }
    }
}

/// 20.263 style:direction
///
/// The style:direction attribute specifies the direction of characters.
///
/// The style:direction attribute modifies the direction of text rendering as specified by a
/// style:writing-mode attribute. 20.404
///
/// The defined values for the style:direction attribute are:
/// * ltr – left to right, text is rendered in the direction specified by the style:writing-mode
/// attribute
/// * ttb – top to bottom, characters are vertically stacked but not rotated
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum WritingDirection {
    Ltr,
    Ttb,
}

impl Display for WritingDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WritingDirection::Ltr => write!(f, "ltr"),
            WritingDirection::Ttb => write!(f, "ttb"),
        }
    }
}

/// 20.283 style:font-relief
///
/// The style:font-relief attribute specifies whether a font should be embossed, engraved, or
/// neither.
///
/// The defined values for the style:font-relief attribute are:
/// * embossed: characters are embossed.
/// * engraved: characters are engraved.
/// * none: characters are neither embossed or engraved.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextRelief {
    None,
    Embossed,
    Engraved,
}

impl Display for TextRelief {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextRelief::None => write!(f, "none"),
            TextRelief::Embossed => write!(f, "embossed"),
            TextRelief::Engraved => write!(f, "engraved"),
        }
    }
}

/// 20.297 style:glyph-orientation-vertical
///
/// The style:glyph-orientation-vertical attribute specifies a vertical glyph orientation.
/// See §10.7.3 of SVG. The attribute specifies an angle or automatic mode. The only defined angle
/// is 0 degrees, which disables this feature.
///
/// Note: OpenDocument v1.1 did not support angle specifications that contain an
/// angle unit identifier. Angle unit identifiers should be omitted for compatibility with
/// OpenDocument v1.1.
///
/// The values of the style:glyph-orientation-vertical attribute are auto, 0, 0deg, 0rad
/// or 0grad.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum GlyphOrientation {
    Auto,
    Zero,
    Angle(Angle),
}

impl Display for GlyphOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GlyphOrientation::Auto => write!(f, "auto"),
            GlyphOrientation::Zero => write!(f, "0"),
            GlyphOrientation::Angle(a) => a.fmt(f),
        }
    }
}

/// 20.315 style:line-break
/// The style:line-break attribute specifies line breaking rules.
/// The defined values for the style:line-break attribute are:
/// * normal: line breaks may occur between any characters.
/// * strict: line breaks shall not occur before or after implementation-defined characters.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LineBreak {
    Normal,
    Strict,
}

impl Display for LineBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineBreak::Normal => write!(f, "normal")?,
            LineBreak::Strict => write!(f, "strict")?,
        }
        Ok(())
    }
}

/// 20.322 style:num-format
///
/// The style:num-format attribute specifies a numbering sequence.
/// If no value is given, no number sequence is displayed.
///
/// The defined values for the style:num-format attribute are:
/// • 1: number sequence starts with “1”.
/// • a: number sequence starts with “a”.
/// • A: number sequence starts with “A”.
/// • empty string: no number sequence displayed.
/// • i: number sequence starts with “i”.
/// • I: number sequence start with “I”.
/// • a value of type string 18.2
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum StyleNumFormat {
    None,
    Number,
    LowerAlpha,
    Alpha,
    LowerRoman,
    Roman,
    Text(String),
}

impl Display for StyleNumFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StyleNumFormat::None => write!(f, ""),
            StyleNumFormat::Number => write!(f, "1"),
            StyleNumFormat::LowerAlpha => write!(f, "a"),
            StyleNumFormat::Alpha => write!(f, "A"),
            StyleNumFormat::LowerRoman => write!(f, "i"),
            StyleNumFormat::Roman => write!(f, "I"),
            StyleNumFormat::Text(v) => write!(f, "{}", v),
        }
    }
}

/// 20.328 style:page-number
///
/// The style:page-number attribute specifies the page number that should be used for a new
/// page when either a paragraph or table style specifies a master page that should be applied
/// beginning from the start of a paragraph or table.
///
/// The defined values for the style:page-number attribute are:
/// • auto: a page has the page number of the previous page, incremented by one.
/// • a value of type nonNegativeInteger 18.2: specifies a page number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum PageNumber {
    Auto,
    Number(u32),
}

impl Display for PageNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PageNumber::Auto => write!(f, "auto"),
            PageNumber::Number(v) => v.fmt(f),
        }
    }
}

/// 20.330 style:print
///
/// The style:print attribute specifies the components in a spreadsheet document to print.
///
/// The value of the style:print attribute is a white space separated list of one or more of these
/// values:
/// • annotations: annotations should be printed.
/// • charts: charts should be printed.
/// • drawings: drawings should be printed.
/// • formulas: formulas should be printed.
/// • grid: grid lines should be printed.
/// • headers: headers should be printed.
/// • objects: (including graphics): objects should be printed.
/// • zero-values: zero-values should be printed.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PrintContent {
    Headers,
    Grid,
    Annotations,
    Objects,
    Charts,
    Drawings,
    Formulas,
    ZeroValues,
}

impl Display for PrintContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintContent::Headers => write!(f, "headers"),
            PrintContent::Grid => write!(f, "grid"),
            PrintContent::Annotations => write!(f, "annotations"),
            PrintContent::Objects => write!(f, "objects"),
            PrintContent::Charts => write!(f, "charts"),
            PrintContent::Drawings => write!(f, "drawings"),
            PrintContent::Formulas => write!(f, "formulas"),
            PrintContent::ZeroValues => write!(f, "zero-values"),
        }
    }
}

/// 20.332 style:print-page-order
///
/// The style:print-page-order attribute specifies the order in which data in a spreadsheet is
/// numbered and printed when the data does not fit on one printed page.
///
/// The defined values for the style:print-page-order attribute are:
/// • ltr: create pages from the first column to the last column before continuing with the next set
/// of rows.
/// • ttb: create pages from the top row to the bottom row before continuing with the next set of
/// columns.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PrintOrder {
    Ltr,
    Ttb,
}

impl Display for PrintOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintOrder::Ltr => write!(f, "ltr"),
            PrintOrder::Ttb => write!(f, "ttb"),
        }
    }
}

/// 20.333 style:print-orientation
///
/// The style:print-orientation attribute specifies the orientation of the printed page. The
/// value of this attribute can be portrait or landscape.
///
/// The defined values for the style:print-orientation attribute are:
/// • landscape: a page is printed in landscape orientation.
/// • portrait: a page is printed in portrait orientation
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PrintOrientation {
    Landscape,
    Portrait,
}

impl Display for PrintOrientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintOrientation::Landscape => write!(f, "landscape"),
            PrintOrientation::Portrait => write!(f, "portrait"),
        }
    }
}

/// 20.335 style:punctuation-wrap
///
/// The style:punctuation-wrap attribute specifies whether a punctuation mark, if one is
/// present, can be hanging, that is, whether it can placed in the margin area at the end of a full line of
/// text.
///
/// The defined values for the style:punctuation-wrap attribute are:
/// • hanging: a punctuation mark can be placed in the margin area at the end of a full line of text.
/// • simple: a punctuation mark cannot be placed in the margin area at the end of a full line of
/// text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum PunctuationWrap {
    Hanging,
    Simple,
}

impl Display for PunctuationWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PunctuationWrap::Hanging => write!(f, "hanging"),
            PunctuationWrap::Simple => write!(f, "simple"),
        }
    }
}

/// 20.340 style:rel-width
///
/// The style:rel-width attribute specifies the width of a table relative to the width of the area
/// that the table is in.
///
/// The style:rel-width attribute has the data type percent 18.3.23.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum RelativeScale {
    Scale,
    ScaleMin,
    Percent(Percent),
}

impl Display for RelativeScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RelativeScale::Scale => write!(f, "scale"),
            RelativeScale::ScaleMin => write!(f, "scale-min"),
            RelativeScale::Percent(v) => write!(f, "{}", v),
        }
    }
}

/// 20.346 style:rotation-align
///  The style:rotation-align attribute specifies how the edge of the text in a cell is aligned
/// after a rotation.
///
/// The values of the style:rotation-align attribute are none, bottom, top or center.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum RotationAlign {
    None,
    Bottom,
    Top,
    Center,
}

impl Display for RotationAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RotationAlign::None => write!(f, "none"),
            RotationAlign::Bottom => write!(f, "bottom"),
            RotationAlign::Top => write!(f, "top"),
            RotationAlign::Center => write!(f, "center"),
        }
    }
}

/// 20.363 style:table-centering
///
/// The style:table-centering attribute specifies whether tables are centered horizontally and/
/// or vertically on the page. This attribute only applies to spreadsheet documents.
///
/// The default is to align the table to the top-left or top-right corner of the page, depending of its
/// writing direction.
///
/// The defined values for the style:table-centering attribute are:
/// • both: tables should be centered both horizontally and vertically on the pages where they
/// appear.
/// • horizontal: tables should be centered horizontally on the pages where they appear.
/// • none: tables should not be centered both horizontally or vertically on the pages where they
/// appear.
/// • vertical: tables should be centered vertically on the pages where they appear.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum PrintCentering {
    None,
    Horizontal,
    Vertical,
    Both,
}

impl Display for PrintCentering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintCentering::None => write!(f, "none"),
            PrintCentering::Horizontal => write!(f, "horizontal"),
            PrintCentering::Vertical => write!(f, "vertical"),
            PrintCentering::Both => write!(f, "both"),
        }
    }
}

/// 20.364 style:text-align-source
///
/// The style:text-align-source attribute specifies the source of a text-align attribute.
///
/// The defined values for the style:text-align-source attribute are:
/// • fix: content alignment uses the value of the fo:text-align 20.223 attribute.
/// • value-type: content alignment uses the value-type of the cell.
///
/// The default alignment for a cell value-type string is left, for other value-types it is right.
///
/// The values of the style:text-align-source attribute are fix or value-type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextAlignSource {
    Fix,
    ValueType,
}

impl Display for TextAlignSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextAlignSource::Fix => write!(f, "fix"),
            TextAlignSource::ValueType => write!(f, "value-type"),
        }
    }
}

/// 20.365 style:text-autospace
///
/// The style:text-autospace attribute specifies whether to add space between portions of
/// Asian, Western, and complex texts.
///
/// The defined values for the style:text-autospace attribute are:
/// • ideograph-alpha: space should be added between portions of Asian, Western and
/// Complex texts.
/// • none: space should not be added between portions of Asian, Western and complex texts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TextAutoSpace {
    IdeographAlpha,
    None,
}

impl Display for TextAutoSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAutoSpace::IdeographAlpha => write!(f, "ideograph-alpha"),
            TextAutoSpace::None => write!(f, "none"),
        }
    }
}

/// 20.367 style:text-combine
///
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
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextCombine {
    None,
    Letters,
    Lines,
}

impl Display for TextCombine {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextCombine::None => write!(f, "none"),
            TextCombine::Letters => write!(f, "letters"),
            TextCombine::Lines => write!(f, "lines"),
        }
    }
}

/// 20.370 style:text-emphasize
///
/// The style:text-emphasize attribute specifies emphasis in a text composed of UNICODE
/// characters whose script type is asian. 20.358
///
/// The value of this attribute consists of two white space-separated values.
/// The first value represents the style to use for emphasis.
/// The second value represents the position of the emphasis and it can be above or below. If the
/// first value is none, the second value can be omitted.
///
/// The defined values for the style:text-emphasize attribute are:
/// * accent: calligraphic accent strokes.
/// * circle: hollow circles.
/// * disc: filled circles.
/// * dot: calligraphic dot.
/// * none: no emphasis marks.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextEmphasize {
    None,
    Accent,
    Circle,
    Disc,
    Dot,
}

impl Display for TextEmphasize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextEmphasize::None => write!(f, "none"),
            TextEmphasize::Accent => write!(f, "accent"),
            TextEmphasize::Circle => write!(f, "circle"),
            TextEmphasize::Disc => write!(f, "disc"),
            TextEmphasize::Dot => write!(f, "dot"),
        }
    }
}

/// 20.370 style:text-emphasize
///
/// The style:text-emphasize attribute specifies emphasis in a text composed of UNICODE
/// characters whose script type is asian. 20.358
///
/// The value of this attribute consists of two white space-separated values.
/// The first value represents the style to use for emphasis.
/// The second value represents the position of the emphasis and it can be above or below. If the
/// first value is none, the second value can be omitted.
///
/// The defined values for the style:text-emphasize attribute are:
/// * accent: calligraphic accent strokes.
/// * circle: hollow circles.
/// * disc: filled circles.
/// * dot: calligraphic dot.
/// * none: no emphasis marks.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum TextEmphasizePosition {
    Above,
    Below,
}

impl Display for TextEmphasizePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextEmphasizePosition::Above => write!(f, "above"),
            TextEmphasizePosition::Below => write!(f, "below"),
        }
    }
}

/// Line modes for underline, overline, line-through.
///
/// 20.372 style:text-line-through-mode
/// 20.380 style:text-overline-mode
/// 20.389 style:text-underline-mode
///
/// The style:text-line-through-mode attribute specifies whether lining through is applied to
/// words only or to portions of text.
///
/// The defined values for the style:text-line-through-mode attribute are:
/// • continuous: lining is applied to words and separating spaces.
/// • skip-white-space: lining is not applied to spaces between words.
///
/// The values of the style:text-line-through-mode attribute are continuous or skip-white-space.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LineMode {
    Continuous,
    SkipWhiteSpace,
}

impl Display for LineMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineMode::Continuous => write!(f, "continuous"),
            LineMode::SkipWhiteSpace => write!(f, "skip-white-space"),
        }
    }
}

/// Line style for underline, overline, line-through.
///
/// 20.373 style:text-line-through-style
/// 20.390 style:text-underline-style
/// 20.381 style:text-overline-style
///
/// The style:text-underline-style attribute specifies a style for underlining text.
/// The defined values for the style:text-underline-style attribute are:
/// * none: text has no underlining.
/// * dash: text has a dashed line underlining it.
/// * dot-dash: text has a line whose repeating pattern is a dot followed by a dash underlining it.
/// * dot-dot-dash: text has a line whose repeating pattern is two dots followed by a dash
/// underlining it.
/// * dotted: text has a dotted line underlining it.
/// * long-dash: text has a dashed line whose dashes are longer than the ones from the dashed
/// line for value dash underlining it.
/// * solid: text has a solid line underlining it.
/// * wave: text has a wavy line underlining it.
///
/// Note: The definitions of the values of the style:text-underline-style
/// attribute are based on the text decoration style 'text-underline-style' from
/// CSS3Text, §9.2.
///
/// The values of the style:text-underline-style attribute are none, solid, dotted, dash,
/// long-dash, dot-dash, dot-dot-dash or wave.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LineStyle {
    Dash,
    DotDash,
    DotDotDash,
    Dotted,
    LongDash,
    None,
    Solid,
    Wave,
}

impl Display for LineStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineStyle::Dash => write!(f, "dash"),
            LineStyle::DotDash => write!(f, "dot-dash"),
            LineStyle::DotDotDash => write!(f, "dot-dot-dash"),
            LineStyle::Dotted => write!(f, "dotted"),
            LineStyle::LongDash => write!(f, "long-dash"),
            LineStyle::None => write!(f, "none"),
            LineStyle::Solid => write!(f, "solid"),
            LineStyle::Wave => write!(f, "wave"),
        }
    }
}

/// 20.376 style:text-line-through-type
/// 20.382 style:text-overline-type
/// 20.391 style:text-underline-type
///
/// The style:text-line-through-type attribute specifies whether text is lined through, and if
/// so, whether a single or double line will be used.
///
/// The defined values for the style:text-line-through-type attribute are:
/// • double: a double line should be used for a line-through text.
/// • none: deprecated.
/// • single: a single line should be used for a line-through text.
///
/// Every occurrence of the style:text-line-through-type attribute should be accompanied
/// by an occurrence of the style:text-line-through-style 20.373 attribute on the same
/// element. There should not be an occurrence of the style:text-line-through-type attribute
/// if the value of the style:text-line-through-style attribute is none.
///
/// The values of the style:text-line-through-type attribute are none, single or double.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum LineType {
    None,
    Single,
    Double,
}

impl Display for LineType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineType::None => write!(f, "none"),
            LineType::Single => write!(f, "single"),
            LineType::Double => write!(f, "double"),
        }
    }
}

/// Line width for underline, overline, line-through.
///
/// 20.377 style:text-line-through-width
/// 20.383 style:text-overline-width
/// 20.392 style:text-underline-width
///
/// The style:text-line-through-width attribute specifies the width of a line-through line. The
/// value bold specifies a line width that is calculated from the font sizes like an auto width, but is
/// wider than an auto width.
///
/// The defined values for the style:text-line-through-width attribute are:
/// • auto: the width of a line-through should be calculated from the font size of the text where the
/// line-through will appear.
/// • bold: the width of a line-through should be calculated from the font size of the text where the
/// line-through will appear but is wider than for the value of auto.
/// • a value of type percent 18.3.23
/// • a value of type positiveInteger 18.2
/// • a value of type positiveLength 18.3.26
///
/// The line-through text styles referenced by the values dash, medium, thick and thin, are
/// implementation-defined. Thin shall be smaller width than medium and medium shall be a smaller
/// width than thick.
///
/// The values of the style:text-line-through-width attribute are auto, normal, bold,
/// thin, medium, thick, a value of type positiveInteger 18.2, a value of type percent
/// 18.3.23 or a value of type positiveLength 18.3.26.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum LineWidth {
    Auto,
    Bold,
    Percent(Percent),
    Int(u32),
    Length(Length),
    Normal,
    Dash,
    Thin,
    Medium,
    Thick,
}

impl From<Length> for LineWidth {
    fn from(value: Length) -> Self {
        LineWidth::Length(value)
    }
}

impl From<Percent> for LineWidth {
    fn from(value: Percent) -> Self {
        LineWidth::Percent(value)
    }
}

impl Display for LineWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineWidth::Auto => write!(f, "auto"),
            LineWidth::Bold => write!(f, "bold"),
            LineWidth::Percent(v) => write!(f, "{}", v),
            LineWidth::Int(v) => write!(f, "{}", v),
            LineWidth::Length(v) => write!(f, "{}", v),
            LineWidth::Normal => write!(f, "normal"),
            LineWidth::Dash => write!(f, "dash"),
            LineWidth::Thin => write!(f, "thin"),
            LineWidth::Medium => write!(f, "medium"),
            LineWidth::Thick => write!(f, "thick"),
        }
    }
}

/// 20.384 style:text-position
///
/// The style:text-position attribute specifies whether text is positioned above or below the
/// baseline:
///
/// The value specifies the vertical text position as a percentage of the
/// current font height or it takes one of the values sub or super. Negative percentages or the sub
/// value place the text below the baseline. Positive percentages or the super value place the text
/// above the baseline. If sub or super is specified, the consumer chooses an appropriate text
/// position.
///
/// The style:text-position attribute has one or two white space separated values. The first
/// values is of type percent 18.3.23, or is one of: super or sub.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum TextPosition {
    Sub,
    Super,
    Percent(Percent),
}

impl From<Percent> for TextPosition {
    fn from(value: Percent) -> Self {
        TextPosition::Percent(value)
    }
}

impl Display for TextPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TextPosition::Sub => write!(f, "sub"),
            TextPosition::Super => write!(f, "super"),
            TextPosition::Percent(v) => write!(f, "{}", v),
        }
    }
}

/// 20.386 style:text-rotation-scale
/// The style:text-rotation-scale attribute specifies whether for rotated text the width of the
/// text should be scaled to fit into the current line height or the width of the text should remain fixed,
/// therefore changing the current line height.
///
/// The defined values for the style:text-rotation-scale attribute are:
/// * fixed: width of text should remain fixed.
/// * line-height: width of text should be scaled to fit the current line height.
///
/// The values of the style:text-rotation-scale attribute are fixed or line-height
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum RotationScale {
    Fixed,
    LineHeight,
}

impl Display for RotationScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            RotationScale::Fixed => write!(f, "fixed"),
            RotationScale::LineHeight => write!(f, "line-height"),
        }
    }
}

/// 20.396 style:vertical-align
///
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
/// The values of the style:vertical-align attribute are top, middle, bottom, auto or
/// baseline.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum ParaAlignVertical {
    Top,
    Middle,
    Bottom,
    Auto,
    Baseline,
}

impl Display for ParaAlignVertical {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ParaAlignVertical::Top => write!(f, "top"),
            ParaAlignVertical::Middle => write!(f, "middle"),
            ParaAlignVertical::Bottom => write!(f, "bottom"),
            ParaAlignVertical::Auto => write!(f, "auto"),
            ParaAlignVertical::Baseline => write!(f, "baseline"),
        }
    }
}

/// 20.396 style:vertical-align
///
/// The style:vertical-align attribute specifies the vertical alignment of text in a table cell. The
/// options for the vertical alignment attribute are as follows:
///
/// The defined values for the style:vertical-align attribute are:
/// * automatic: consumer determines how to align the text.
/// * bottom: aligns text vertically with the bottom of the cell.
/// * middle: aligns text vertically with the middle of the cell.
/// * top: aligns text vertically with the top of the cell.
///
/// The values of the style:vertical-align attribute are top, middle, bottom or
/// automatic.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum CellAlignVertical {
    Top,
    Middle,
    Bottom,
    Automatic,
}

impl Display for CellAlignVertical {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CellAlignVertical::Top => write!(f, "top"),
            CellAlignVertical::Middle => write!(f, "middle"),
            CellAlignVertical::Bottom => write!(f, "bottom"),
            CellAlignVertical::Automatic => write!(f, "automatic"),
        }
    }
}

/// 20.404 style:writing-mode
///
/// See §7.27.7 of XSL with the additional value of page.
/// The defined value for the style:writing-mode attribute is page: writing mode is inherited from
/// the page that contains the element where this attribute appears.
///
/// The values of the style:writing-mode attribute are lr-tb, rl-tb, tb-rl, tb-lr, lr, rl,
/// tb or page.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum WritingMode {
    LrTb,
    RlTb,
    TbRl,
    TbLr,
    Lr,
    Rl,
    Tb,
    Page,
}

impl Display for WritingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WritingMode::LrTb => write!(f, "lr-tb"),
            WritingMode::RlTb => write!(f, "rl-tb"),
            WritingMode::TbRl => write!(f, "tb-rl"),
            WritingMode::TbLr => write!(f, "tb-lr"),
            WritingMode::Lr => write!(f, "lr"),
            WritingMode::Rl => write!(f, "rl"),
            WritingMode::Tb => write!(f, "tb"),
            WritingMode::Page => write!(f, "page"),
        }
    }
}

/// 20.414 table:align
///
/// The table:align attribute specifies the horizontal alignment of a table.
///
/// The defined values for the table:align attribute are:
/// • center: table aligns to the center between left and right margins.
/// • left: table aligns to the left margin.
/// • margins: table fills all the space between the left and right margins.
/// • right: table aligns to the right margin.
///
/// Consumers that do not support the margins value, may treat this value as left.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TableAlign {
    Center,
    Left,
    Right,
    Margins,
}

impl Display for TableAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableAlign::Center => write!(f, "center"),
            TableAlign::Left => write!(f, "left"),
            TableAlign::Right => write!(f, "right"),
            TableAlign::Margins => write!(f, "margins"),
        }
    }
}

/// 20.415 table:border-model
///
/// The table:border-model attribute specifies what border model to use when creating a table
/// with a border.
///
/// The defined values for the table:border-model attribute are:
/// • collapsing: when two adjacent cells have different borders, the wider border appears as
/// the border between the cells. Each cell receives half of the width of the border.
/// • separating: borders appear within the cell that specifies the border.
///
/// In OpenDocument, a row height or column width includes any space required to display borders
/// or padding. This means that, while the width and height of the content area is less than the
/// column width and row height, the sum of the widths of all columns is equal to the total width of the
/// table.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TableBorderModel {
    Collapsing,
    Separating,
}

impl Display for TableBorderModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TableBorderModel::Collapsing => write!(f, "collapsing"),
            TableBorderModel::Separating => write!(f, "separating"),
        }
    }
}

/// 20.426 text:condition
///
/// The text:condition attribute specifies the display of text.
/// The defined value of the text:condition attribute is none, which means text is hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TextCondition {
    None,
}

impl Display for TextCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextCondition::None => write!(f, "none"),
        }
    }
}

/// 20.427 text:display
///
/// The text:display attribute specifies whether text is hidden.
///
/// The defined values for the text:display attribute are:
/// • condition: text is hidden under the condition specified in the text:condition 20.426
/// attribute.
/// • none: text is hidden unconditionally.
/// • true: text is displayed. This is the default setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TextDisplay {
    None,
    Condition,
    True,
}

impl Display for TextDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextDisplay::None => write!(f, "none"),
            TextDisplay::Condition => write!(f, "condition"),
            TextDisplay::True => write!(f, "true"),
        }
    }
}
