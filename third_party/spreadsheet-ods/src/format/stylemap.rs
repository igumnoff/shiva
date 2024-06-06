//!
//! Conditional styles.
//!

use crate::condition::ValueCondition;
use get_size::GetSize;
use get_size_derive::GetSize;

/// A style-map is one way for conditional formatting of value formats.
#[derive(Clone, Debug, Default, GetSize)]
pub struct ValueStyleMap {
    condition: ValueCondition,
    applied_style: String, // todo:
}

impl ValueStyleMap {
    /// Create a stylemap for a ValueFormat. When the condition is fullfilled the style
    /// applied_style is used.
    pub fn new<T: AsRef<str>>(condition: ValueCondition, applied_style: T) -> Self {
        Self {
            condition,
            applied_style: applied_style.as_ref().to_string(),
        }
    }

    /// Condition
    pub fn condition(&self) -> &ValueCondition {
        &self.condition
    }

    /// Condition
    pub fn set_condition(&mut self, cond: ValueCondition) {
        self.condition = cond;
    }

    /// The applied style.
    pub fn applied_style(&self) -> &String {
        &self.applied_style
    }

    /// Sets the applied style.
    pub fn set_applied_style<S: AsRef<str>>(&mut self, style: S) {
        self.applied_style = style.as_ref().to_string();
    }
}
