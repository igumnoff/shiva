use crate::text::TextTag;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use get_size::GetSize;
use get_size_derive::GetSize;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::borrow::Cow;

/// Datatypes for the values. Only the discriminants of the Value enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, GetSize)]
#[allow(missing_docs)]
pub enum ValueType {
    Empty,
    Boolean,
    Number,
    Percentage,
    Currency,
    Text,
    TextXml,
    DateTime,
    TimeDuration,
}

/// Content-Values
#[derive(Debug, Clone, PartialEq, Default)]
#[allow(missing_docs)]
pub enum Value {
    #[default]
    Empty,
    Boolean(bool),
    Number(f64),
    Percentage(f64),
    Currency(f64, Box<str>),
    Text(String),
    TextXml(Vec<TextTag>),
    DateTime(NaiveDateTime),
    TimeDuration(Duration),
}

impl GetSize for Value {
    fn get_heap_size(&self) -> usize {
        match self {
            Value::Empty => 0,
            Value::Boolean(_) => 0,
            Value::Number(_) => 0,
            Value::Percentage(_) => 0,
            Value::Currency(_, v) => v.get_heap_size(),
            Value::Text(v) => v.get_heap_size(),
            Value::TextXml(v) => v.get_heap_size(),
            Value::DateTime(_) => 0,
            Value::TimeDuration(_) => 0,
        }
    }
}

impl Value {
    /// Return the plan ValueType for this value.
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Empty => ValueType::Empty,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Number(_) => ValueType::Number,
            Value::Percentage(_) => ValueType::Percentage,
            Value::Currency(_, _) => ValueType::Currency,
            Value::Text(_) => ValueType::Text,
            Value::TextXml(_) => ValueType::TextXml,
            Value::TimeDuration(_) => ValueType::TimeDuration,
            Value::DateTime(_) => ValueType::DateTime,
        }
    }

    /// Return the bool if the value is a Boolean. Default otherwise.
    pub fn as_bool_or(&self, d: bool) -> bool {
        match self {
            Value::Boolean(b) => *b,
            _ => d,
        }
    }

    /// Return the content as i64 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_i64_or(&self, d: i64) -> i64 {
        match self {
            Value::Number(n) => *n as i64,
            Value::Percentage(p) => *p as i64,
            Value::Currency(v, _) => *v as i64,
            _ => d,
        }
    }

    /// Return the content as i64 if the value is a number, percentage or
    /// currency.
    pub fn as_i64_opt(&self) -> Option<i64> {
        match self {
            Value::Number(n) => Some(*n as i64),
            Value::Percentage(p) => Some(*p as i64),
            Value::Currency(v, _) => Some(*v as i64),
            _ => None,
        }
    }

    /// Return the content as u64 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_u64_or(&self, d: u64) -> u64 {
        match self {
            Value::Number(n) => *n as u64,
            Value::Percentage(p) => *p as u64,
            Value::Currency(v, _) => *v as u64,
            _ => d,
        }
    }

    /// Return the content as u64 if the value is a number, percentage or
    /// currency.
    pub fn as_u64_opt(&self) -> Option<u64> {
        match self {
            Value::Number(n) => Some(*n as u64),
            Value::Percentage(p) => Some(*p as u64),
            Value::Currency(v, _) => Some(*v as u64),
            _ => None,
        }
    }

    /// Return the content as i32 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_i32_or(&self, d: i32) -> i32 {
        match self {
            Value::Number(n) => *n as i32,
            Value::Percentage(p) => *p as i32,
            Value::Currency(v, _) => *v as i32,
            _ => d,
        }
    }

    /// Return the content as i32 if the value is a number, percentage or
    /// currency.
    pub fn as_i32_opt(&self) -> Option<i32> {
        match self {
            Value::Number(n) => Some(*n as i32),
            Value::Percentage(p) => Some(*p as i32),
            Value::Currency(v, _) => Some(*v as i32),
            _ => None,
        }
    }

    /// Return the content as u32 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_u32_or(&self, d: u32) -> u32 {
        match self {
            Value::Number(n) => *n as u32,
            Value::Percentage(p) => *p as u32,
            Value::Currency(v, _) => *v as u32,
            _ => d,
        }
    }

    /// Return the content as u32 if the value is a number, percentage or
    /// currency.
    pub fn as_u32_opt(&self) -> Option<u32> {
        match self {
            Value::Number(n) => Some(*n as u32),
            Value::Percentage(p) => Some(*p as u32),
            Value::Currency(v, _) => Some(*v as u32),
            _ => None,
        }
    }

    /// Return the content as i16 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_i16_or(&self, d: i16) -> i16 {
        match self {
            Value::Number(n) => *n as i16,
            Value::Percentage(p) => *p as i16,
            Value::Currency(v, _) => *v as i16,
            _ => d,
        }
    }

    /// Return the content as i16 if the value is a number, percentage or
    /// currency.
    pub fn as_i16_opt(&self) -> Option<i16> {
        match self {
            Value::Number(n) => Some(*n as i16),
            Value::Percentage(p) => Some(*p as i16),
            Value::Currency(v, _) => Some(*v as i16),
            _ => None,
        }
    }

    /// Return the content as u16 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_u16_or(&self, d: u16) -> u16 {
        match self {
            Value::Number(n) => *n as u16,
            Value::Percentage(p) => *p as u16,
            Value::Currency(v, _) => *v as u16,
            _ => d,
        }
    }

    /// Return the content as u16 if the value is a number, percentage or
    /// currency.
    pub fn as_u16_opt(&self) -> Option<u16> {
        match self {
            Value::Number(n) => Some(*n as u16),
            Value::Percentage(p) => Some(*p as u16),
            Value::Currency(v, _) => Some(*v as u16),
            _ => None,
        }
    }

    /// Return the content as i8 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_i8_or(&self, d: i8) -> i8 {
        match self {
            Value::Number(n) => *n as i8,
            Value::Percentage(p) => *p as i8,
            Value::Currency(v, _) => *v as i8,
            _ => d,
        }
    }

    /// Return the content as i8 if the value is a number, percentage or
    /// currency.
    pub fn as_i8_opt(&self) -> Option<i8> {
        match self {
            Value::Number(n) => Some(*n as i8),
            Value::Percentage(p) => Some(*p as i8),
            Value::Currency(v, _) => Some(*v as i8),
            _ => None,
        }
    }

    /// Return the content as u8 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_u8_or(&self, d: u8) -> u8 {
        match self {
            Value::Number(n) => *n as u8,
            Value::Percentage(p) => *p as u8,
            Value::Currency(v, _) => *v as u8,
            _ => d,
        }
    }

    /// Return the content as u8 if the value is a number, percentage or
    /// currency.
    pub fn as_u8_opt(&self) -> Option<u8> {
        match self {
            Value::Number(n) => Some(*n as u8),
            Value::Percentage(p) => Some(*p as u8),
            Value::Currency(v, _) => Some(*v as u8),
            _ => None,
        }
    }

    /// Return the content as decimal if the value is a number, percentage or
    /// currency. Default otherwise.
    #[cfg(feature = "use_decimal")]
    pub fn as_decimal_or(&self, d: Decimal) -> Decimal {
        match self {
            Value::Number(n) => Decimal::from_f64(*n).unwrap_or(d),
            Value::Currency(v, _) => Decimal::from_f64(*v).unwrap_or(d),
            Value::Percentage(p) => Decimal::from_f64(*p).unwrap_or(d),
            _ => d,
        }
    }

    /// Return the content as decimal if the value is a number, percentage or
    /// currency. Default otherwise.
    #[cfg(feature = "use_decimal")]
    pub fn as_decimal_opt(&self) -> Option<Decimal> {
        match self {
            Value::Number(n) => Decimal::from_f64(*n),
            Value::Currency(v, _) => Decimal::from_f64(*v),
            Value::Percentage(p) => Decimal::from_f64(*p),
            _ => None,
        }
    }

    /// Return the content as f64 if the value is a number, percentage or
    /// currency. Default otherwise.
    pub fn as_f64_or(&self, d: f64) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Currency(v, _) => *v,
            Value::Percentage(p) => *p,
            _ => d,
        }
    }

    /// Return the content as f64 if the value is a number, percentage or
    /// currency.
    pub fn as_f64_opt(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Currency(v, _) => Some(*v),
            Value::Percentage(p) => Some(*p),
            _ => None,
        }
    }

    /// Return the content as str if the value is text.
    pub fn as_str_or<'a>(&'a self, d: &'a str) -> &'a str {
        match self {
            Value::Text(s) => s.as_ref(),
            _ => d,
        }
    }

    /// Return the content as str if the value is text or markup text.
    /// When the cell contains markup all the markup is removed, but
    /// line-breaks are kept as \n.
    pub fn as_cow_str_or<'a>(&'a self, d: &'a str) -> Cow<'a, str> {
        match self {
            Value::Text(s) => Cow::from(s),
            Value::TextXml(v) => {
                let mut buf = String::new();
                for t in v {
                    if !buf.is_empty() {
                        buf.push('\n');
                    }
                    t.extract_text(&mut buf);
                }
                Cow::from(buf)
            }
            _ => Cow::from(d),
        }
    }

    /// Return the content as str if the value is text.
    pub fn as_str_opt(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s.as_ref()),
            _ => None,
        }
    }

    /// Return the content as Duration if the value is a TimeDuration.
    /// Default otherwise.
    pub fn as_timeduration_or(&self, d: Duration) -> Duration {
        match self {
            Value::TimeDuration(td) => *td,
            _ => d,
        }
    }

    /// Return the content as Duration if the value is a TimeDuration.
    /// Default otherwise.
    pub fn as_timeduration_opt(&self) -> Option<Duration> {
        match self {
            Value::TimeDuration(td) => Some(*td),
            _ => None,
        }
    }

    /// Return the content as NaiveDateTime if the value is a DateTime.
    /// Default otherwise.
    pub fn as_datetime_or(&self, d: NaiveDateTime) -> NaiveDateTime {
        match self {
            Value::DateTime(dt) => *dt,
            _ => d,
        }
    }

    /// Return the content as an optional NaiveDateTime if the value is
    /// a DateTime.
    pub fn as_datetime_opt(&self) -> Option<NaiveDateTime> {
        match self {
            Value::DateTime(dt) => Some(*dt),
            _ => None,
        }
    }

    /// Return the content as NaiveDate if the value is a DateTime.
    /// Default otherwise.
    pub fn as_date_or(&self, d: NaiveDate) -> NaiveDate {
        match self {
            Value::DateTime(dt) => dt.date(),
            _ => d,
        }
    }

    /// Return the content as an optional NaiveDateTime if the value is
    /// a DateTime.
    pub fn as_date_opt(&self) -> Option<NaiveDate> {
        match self {
            Value::DateTime(dt) => Some(dt.date()),
            _ => None,
        }
    }

    /// Returns the currency code or "" if the value is not a currency.
    pub fn currency(&self) -> &str {
        match self {
            Value::Currency(_, c) => c,
            _ => "",
        }
    }

    /// Create a currency value.
    #[allow(clippy::needless_range_loop)]
    pub fn new_currency<S: AsRef<str>>(cur: S, value: f64) -> Self {
        Value::Currency(value, cur.as_ref().into())
    }

    /// Create a percentage value.
    pub fn new_percentage(value: f64) -> Self {
        Value::Percentage(value)
    }
}

/// currency value
#[macro_export]
macro_rules! currency {
    ($c:expr, $v:expr) => {
        Value::new_currency($c, $v as f64)
    };
}

/// currency value
#[macro_export]
macro_rules! percent {
    ($v:expr) => {
        Value::new_percentage($v)
    };
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Text(s)
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value::Text(s.to_string())
    }
}

impl From<TextTag> for Value {
    fn from(t: TextTag) -> Self {
        Value::TextXml(vec![t])
    }
}

impl From<Vec<TextTag>> for Value {
    fn from(t: Vec<TextTag>) -> Self {
        Value::TextXml(t)
    }
}

impl From<Option<&str>> for Value {
    fn from(s: Option<&str>) -> Self {
        if let Some(s) = s {
            Value::Text(s.to_string())
        } else {
            Value::Empty
        }
    }
}

impl From<Option<&String>> for Value {
    fn from(s: Option<&String>) -> Self {
        if let Some(s) = s {
            Value::Text(s.to_string())
        } else {
            Value::Empty
        }
    }
}

impl From<Option<String>> for Value {
    fn from(s: Option<String>) -> Self {
        if let Some(s) = s {
            Value::Text(s)
        } else {
            Value::Empty
        }
    }
}

#[cfg(feature = "use_decimal")]
impl From<Decimal> for Value {
    fn from(f: Decimal) -> Self {
        Value::Number(f.to_f64().expect("decimal->f64 should not fail"))
    }
}

#[cfg(feature = "use_decimal")]
impl From<Option<Decimal>> for Value {
    fn from(f: Option<Decimal>) -> Self {
        if let Some(f) = f {
            Value::Number(f.to_f64().expect("decimal->f64 should not fail"))
        } else {
            Value::Empty
        }
    }
}

macro_rules! from_number {
    ($l:ty) => {
        impl From<$l> for Value {
            #![allow(trivial_numeric_casts)]
            fn from(f: $l) -> Self {
                Value::Number(f as f64)
            }
        }

        impl From<&$l> for Value {
            #![allow(trivial_numeric_casts)]
            fn from(f: &$l) -> Self {
                Value::Number(*f as f64)
            }
        }

        impl From<Option<$l>> for Value {
            #![allow(trivial_numeric_casts)]
            fn from(f: Option<$l>) -> Self {
                if let Some(f) = f {
                    Value::Number(f as f64)
                } else {
                    Value::Empty
                }
            }
        }

        impl From<Option<&$l>> for Value {
            #![allow(trivial_numeric_casts)]
            fn from(f: Option<&$l>) -> Self {
                if let Some(f) = f {
                    Value::Number(*f as f64)
                } else {
                    Value::Empty
                }
            }
        }
    };
}

from_number!(f64);
from_number!(f32);
from_number!(i64);
from_number!(i32);
from_number!(i16);
from_number!(i8);
from_number!(u64);
from_number!(u32);
from_number!(u16);
from_number!(u8);

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<Option<bool>> for Value {
    fn from(b: Option<bool>) -> Self {
        if let Some(b) = b {
            Value::Boolean(b)
        } else {
            Value::Empty
        }
    }
}

impl From<NaiveDateTime> for Value {
    fn from(dt: NaiveDateTime) -> Self {
        Value::DateTime(dt)
    }
}

impl From<Option<NaiveDateTime>> for Value {
    fn from(dt: Option<NaiveDateTime>) -> Self {
        if let Some(dt) = dt {
            Value::DateTime(dt)
        } else {
            Value::Empty
        }
    }
}

impl From<NaiveDate> for Value {
    fn from(dt: NaiveDate) -> Self {
        Value::DateTime(dt.and_hms_opt(0, 0, 0).unwrap())
    }
}

impl From<Option<NaiveDate>> for Value {
    fn from(dt: Option<NaiveDate>) -> Self {
        if let Some(dt) = dt {
            Value::DateTime(dt.and_hms_opt(0, 0, 0).expect("valid time"))
        } else {
            Value::Empty
        }
    }
}

impl From<NaiveTime> for Value {
    fn from(ti: NaiveTime) -> Self {
        Value::DateTime(NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1900, 1, 1).expect("valid date"),
            ti,
        ))
    }
}

impl From<Option<NaiveTime>> for Value {
    fn from(dt: Option<NaiveTime>) -> Self {
        if let Some(ti) = dt {
            Value::DateTime(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1900, 1, 1).expect("valid date"),
                ti,
            ))
        } else {
            Value::Empty
        }
    }
}

impl From<Duration> for Value {
    fn from(d: Duration) -> Self {
        Value::TimeDuration(d)
    }
}

impl From<Option<Duration>> for Value {
    fn from(d: Option<Duration>) -> Self {
        if let Some(d) = d {
            Value::TimeDuration(d)
        } else {
            Value::Empty
        }
    }
}
