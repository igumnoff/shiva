//! Defines conditional expressions that are used for cell-validation and
//! conditional styles via style-maps.
use get_size::GetSize;
use get_size_derive::GetSize;
use std::fmt::{Display, Formatter};

use crate::CellRange;

/// A value that is used in a comparison.
#[derive(Clone, Debug)]
pub struct Value {
    val: String,
}

fn quote(val: &str) -> String {
    let mut buf = String::new();
    buf.push('"');
    for c in val.chars() {
        if c == '"' {
            buf.push('"');
            buf.push('"');
        } else {
            buf.push(c);
        }
    }
    buf.push('"');
    buf
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value { val: quote(s) }
    }
}

impl From<&&str> for Value {
    fn from(s: &&str) -> Self {
        Value { val: quote(s) }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value {
            val: quote(s.as_str()),
        }
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        Value {
            val: quote(s.as_str()),
        }
    }
}

macro_rules! from_x_conditionvalue {
    ($int:ty) => {
        impl From<$int> for Value {
            fn from(v: $int) -> Self {
                Value { val: v.to_string() }
            }
        }

        impl From<&$int> for Value {
            fn from(v: &$int) -> Self {
                Value { val: v.to_string() }
            }
        }
    };
}

from_x_conditionvalue!(i8);
from_x_conditionvalue!(i16);
from_x_conditionvalue!(i32);
from_x_conditionvalue!(i64);
from_x_conditionvalue!(u8);
from_x_conditionvalue!(u16);
from_x_conditionvalue!(u32);
from_x_conditionvalue!(u64);
from_x_conditionvalue!(f32);
from_x_conditionvalue!(f64);
from_x_conditionvalue!(bool);

/// Defines a condition that compares the cell-content with a value.
#[derive(Default, Clone, Debug, GetSize)]
pub struct ValueCondition {
    cond: String,
}

impl Display for ValueCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cond)
    }
}

impl ValueCondition {
    /// Creates a value condition from a string that was read.
    pub(crate) fn new<S: Into<String>>(str: S) -> Self {
        Self { cond: str.into() }
    }

    /// Compares the cell-content with a value.
    pub fn value_eq<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()=");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn value_ne<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()!=");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn value_lt<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()<");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn value_gt<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()>");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn value_le<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()<=");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn value_ge<V: Into<Value>>(value: V) -> ValueCondition {
        let mut buf = String::new();
        buf.push_str("value()>=");
        buf.push_str(value.into().to_string().as_str());
        ValueCondition { cond: buf }
    }
}

/// Defines a condition for a cell-validation.
#[derive(Default, Clone, Debug, GetSize)]
pub struct Condition {
    cond: String,
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cond)
    }
}

impl Condition {
    /// Creates a condition from a read string.
    pub(crate) fn new<S: Into<String>>(str: S) -> Self {
        Self { cond: str.into() }
    }

    /// Compares the cell-content with a value.
    pub fn content_eq<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()=");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn content_ne<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()!=");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn content_lt<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()<");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn content_gt<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()>");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn content_le<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()<=");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the cell-content with a value.
    pub fn content_ge<V: Into<Value>>(value: V) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content()>=");
        buf.push_str(value.into().to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_eq(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()=");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_ne(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()!=");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_lt(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()<");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_gt(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()>");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_le(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()<=");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a value.
    pub fn content_text_length_ge(len: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length()>=");
        buf.push_str(len.to_string().as_str());
        Condition { cond: buf }
    }

    /// Compares the content length to a range of values.
    pub fn content_text_length_is_between(from: u32, to: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length-is-between(");
        buf.push_str(from.to_string().as_str());
        buf.push_str(", ");
        buf.push_str(to.to_string().as_str());
        buf.push(')');
        Condition { cond: buf }
    }

    /// Range check.
    pub fn content_text_length_is_not_between(from: u32, to: u32) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-text-length-is-not-between(");
        buf.push_str(from.to_string().as_str());
        buf.push_str(", ");
        buf.push_str(to.to_string().as_str());
        buf.push(')');
        Condition { cond: buf }
    }

    /// The value is in this list.
    pub fn content_is_in_list<'a, V>(list: &'a [V]) -> Condition
    where
        Value: From<&'a V>,
    {
        let mut buf = String::new();
        buf.push_str("cell-content-is-in-list(");

        let mut sep = false;
        for v in list {
            if sep {
                buf.push(';');
            }
            let vv: Value = v.into();
            let vstr = vv.to_string();
            if !vstr.starts_with('"') {
                buf.push('"');
            }
            buf.push_str(vstr.as_str());
            if !vstr.starts_with('"') {
                buf.push('"');
            }
            sep = true;
        }

        buf.push(')');
        Condition { cond: buf }
    }

    /// The choices are made up from the values in the cellrange.
    ///
    /// Warning
    /// For the cellrange the distance to the base-cell is calculated,
    /// and this result is added to the cell this condition is applied to.
    /// You may want to use an absolute cell-reference to avoid this..
    ///
    pub fn content_is_in_cellrange(range: CellRange) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-is-in-list(");
        buf.push_str(range.to_formula().as_str());
        buf.push(')');
        Condition { cond: buf }
    }

    /// Content is a date and matches a comparison.
    /// The date is an integer value that amounts to the days since
    /// 30.12.1899.
    pub fn content_is_date_and(vcond: Condition) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-is-date()");
        buf.push_str(" and ");
        buf.push_str(vcond.to_string().as_str());
        Condition { cond: buf }
    }

    /// Content is a time and matches a comparison.
    /// The time is given as a fraction of a day.
    pub fn content_is_time_and(vcond: Condition) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-is-time()");
        buf.push_str(" and ");
        buf.push_str(vcond.to_string().as_str());
        Condition { cond: buf }
    }

    /// Content is a number and matches the comparison.
    pub fn content_is_decimal_number_and(vcond: Condition) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-is-decimal-number()");
        buf.push_str(" and ");
        buf.push_str(vcond.to_string().as_str());
        Condition { cond: buf }
    }

    /// Content is a whole number and matches the comparison.
    pub fn content_is_whole_number_and(vcond: Condition) -> Condition {
        let mut buf = String::new();
        buf.push_str("cell-content-is-whole-number()");
        buf.push_str(" and ");
        buf.push_str(vcond.to_string().as_str());
        Condition { cond: buf }
    }

    /// Evaluates a formula.
    pub fn is_true_formula<S: AsRef<str>>(formula: S) -> Condition {
        let mut buf = String::new();
        buf.push_str("is-true-formula(");
        buf.push_str(formula.as_ref());
        buf.push(')');
        Condition { cond: buf }
    }
}

#[cfg(test)]
mod tests {
    use crate::condition::{Condition, ValueCondition};
    use crate::CellRange;

    #[test]
    fn test_valuecondition() {
        let c = ValueCondition::value_eq(5);
        assert_eq!(c.to_string(), "value()=5");
        let c = ValueCondition::value_ne(5);
        assert_eq!(c.to_string(), "value()!=5");
        let c = ValueCondition::value_lt(5);
        assert_eq!(c.to_string(), "value()<5");
        let c = ValueCondition::value_gt(5);
        assert_eq!(c.to_string(), "value()>5");
        let c = ValueCondition::value_le(5);
        assert_eq!(c.to_string(), "value()<=5");
        let c = ValueCondition::value_ge(5);
        assert_eq!(c.to_string(), "value()>=5");
    }

    #[test]
    fn test_condition() {
        let c = Condition::content_text_length_eq(7);
        assert_eq!(c.to_string(), "cell-content-text-length()=7");
        let c = Condition::content_text_length_is_between(5, 7);
        assert_eq!(c.to_string(), "cell-content-text-length-is-between(5, 7)");
        let c = Condition::content_text_length_is_not_between(5, 7);
        assert_eq!(
            c.to_string(),
            "cell-content-text-length-is-not-between(5, 7)"
        );
        let c = Condition::content_is_in_list(&[1, 2, 3, 4, 5]);
        assert_eq!(
            c.to_string(),
            r#"cell-content-is-in-list("1";"2";"3";"4";"5")"#
        );
        let c = Condition::content_is_in_list(&["a", "b", "c"]);
        assert_eq!(c.to_string(), r#"cell-content-is-in-list("a";"b";"c")"#);
        let c = Condition::content_is_in_cellrange(CellRange::remote("other", 0, 0, 10, 0));
        assert_eq!(c.to_string(), "cell-content-is-in-list([other.A1:.A11])");

        let c = Condition::content_is_date_and(Condition::content_eq(0));
        assert_eq!(c.to_string(), "cell-content-is-date() and cell-content()=0");
        let c = Condition::content_is_time_and(Condition::content_eq(0));
        assert_eq!(c.to_string(), "cell-content-is-time() and cell-content()=0");
        let c = Condition::content_is_decimal_number_and(Condition::content_eq(0));
        assert_eq!(
            c.to_string(),
            "cell-content-is-decimal-number() and cell-content()=0"
        );
        let c = Condition::content_is_whole_number_and(Condition::content_eq(0));
        assert_eq!(
            c.to_string(),
            "cell-content-is-whole-number() and cell-content()=0"
        );

        let c = Condition::is_true_formula("formula");
        assert_eq!(c.to_string(), "is-true-formula(formula)");
    }
}
