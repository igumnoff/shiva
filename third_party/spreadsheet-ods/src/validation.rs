//!
//! Content validation.
//!

use get_size::GetSize;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::condition::Condition;
use crate::style::AnyStyleRef;
use crate::text::TextTag;
use crate::{CellRef, OdsError};
use get_size_derive::GetSize;
use std::borrow::Borrow;
use std::str::from_utf8;

/// This defines how lists of entries are displayed to the user.
#[derive(Copy, Clone, Debug, Default, GetSize)]
pub enum ValidationDisplay {
    /// Don't show.
    NoDisplay,
    /// Show the entries in the original order.
    #[default]
    Unsorted,
    /// Sort the entries.
    SortAscending,
}

impl TryFrom<&str> for ValidationDisplay {
    type Error = OdsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "unsorted" => Ok(ValidationDisplay::Unsorted),
            "sort-ascending" => Ok(ValidationDisplay::SortAscending),
            "none" => Ok(ValidationDisplay::NoDisplay),
            _ => Err(OdsError::Parse(
                "invalid table:display-list ",
                Some(value.to_string()),
            )),
        }
    }
}

impl TryFrom<&[u8]> for ValidationDisplay {
    type Error = OdsError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match value {
            b"unsorted" => Ok(ValidationDisplay::Unsorted),
            b"sort-ascending" => Ok(ValidationDisplay::SortAscending),
            b"none" => Ok(ValidationDisplay::NoDisplay),
            _ => Err(OdsError::Parse(
                "invalid table:display-list ",
                Some(from_utf8(value)?.into()),
            )),
        }
    }
}

/// Help text for a validation.
#[derive(Clone, Debug, GetSize)]
pub struct ValidationHelp {
    display: bool,
    title: Option<String>,
    text: Option<Box<TextTag>>,
}

impl Default for ValidationHelp {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationHelp {
    /// Empty message.
    pub fn new() -> Self {
        Self {
            display: true,
            title: None,
            text: None,
        }
    }

    /// Show the help text.
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Show the help text.
    pub fn display(&self) -> bool {
        self.display
    }

    /// Title for the help text.
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Title for the help text.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Help text as formatted text.
    pub fn set_text(&mut self, text: Option<TextTag>) {
        if let Some(txt) = text {
            self.text = Some(Box::new(txt));
        } else {
            self.text = None;
        };
    }

    /// Help text as formatted text.
    pub fn text(&self) -> Option<&TextTag> {
        self.text.as_deref()
    }
}

/// Determines the severity of a validation error.
/// When this is error the entered value is discarded, otherwise
/// the error is just shown as a warning or a hint.
#[derive(Copy, Clone, Debug, GetSize)]
pub enum MessageType {
    /// Hard error.
    Error,
    /// Warning.
    Warning,
    /// Informational.
    Info,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Error => write!(f, "stop"),
            MessageType::Warning => write!(f, "warning"),
            MessageType::Info => write!(f, "information"),
        }
    }
}

/// Error handling for content validations.
#[derive(Clone, Debug, GetSize)]
pub struct ValidationError {
    display: bool,
    msg_type: MessageType,
    title: Option<String>,
    text: Option<Box<TextTag>>,
}

impl Default for ValidationError {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationError {
    /// Empty message.
    pub fn new() -> Self {
        Self {
            display: true,
            msg_type: MessageType::Error,
            title: None,
            text: None,
        }
    }

    /// Is the error text shown.
    pub fn set_display(&mut self, display: bool) {
        self.display = display;
    }

    /// Is the error text shown.
    pub fn display(&self) -> bool {
        self.display
    }

    /// Type of error.
    pub fn set_msg_type(&mut self, msg_type: MessageType) {
        self.msg_type = msg_type;
    }

    /// Type of error.
    pub fn msg_type(&self) -> &MessageType {
        &self.msg_type
    }

    /// Title for the message.
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Title for the message.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Styled text for the message.
    pub fn set_text(&mut self, text: Option<TextTag>) {
        if let Some(txt) = text {
            self.text = Some(Box::new(txt));
        } else {
            self.text = None;
        };
    }

    /// Styled text for the message.
    pub fn text(&self) -> Option<&TextTag> {
        self.text.as_deref()
    }
}

style_ref2!(ValidationRef);

/// Cell content validations.
///
/// This defines a validity constraint via the contained condition.
/// It can be applied to a cell by setting the validation name.
#[derive(Clone, Debug, Default, GetSize)]
pub struct Validation {
    name: String,
    condition: Condition,
    base_cell: CellRef,
    allow_empty: bool,
    display_list: ValidationDisplay,
    err: Option<ValidationError>,
    help: Option<ValidationHelp>,
}

impl Validation {
    /// Empty validation.
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            condition: Default::default(),
            base_cell: Default::default(),
            allow_empty: true,
            display_list: Default::default(),
            err: Some(ValidationError {
                display: true,
                msg_type: MessageType::Error,
                title: None,
                text: None,
            }),
            help: None,
        }
    }

    /// Validation name.
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().to_string();
    }

    /// Validation name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Creates a reference struct for this one.
    pub fn validation_ref(&self) -> ValidationRef {
        ValidationRef::from(self.name.clone())
    }

    /// Sets the condition that is checked for new values.
    pub fn set_condition(&mut self, cond: Condition) {
        self.condition = cond;
    }

    /// Condition for new values.
    pub fn condition(&self) -> &Condition {
        &self.condition
    }

    /// Base-cell for the validation. Relative CellReferences in the
    /// condition are relative to this cell. They are moved with the
    /// actual cell this condition is applied to.
    pub fn set_base_cell(&mut self, base: CellRef) {
        self.base_cell = base;
    }

    /// Base-cell for the validation.
    pub fn base_cell(&self) -> &CellRef {
        &self.base_cell
    }

    /// Empty ok?
    pub fn set_allow_empty(&mut self, allow: bool) {
        self.allow_empty = allow;
    }

    /// Empty ok?
    pub fn allow_empty(&self) -> bool {
        self.allow_empty
    }

    /// Display list of choices.
    pub fn set_display(&mut self, display: ValidationDisplay) {
        self.display_list = display;
    }

    /// Display list of choices.
    pub fn display(&self) -> ValidationDisplay {
        self.display_list
    }

    /// Error message.
    pub fn set_err(&mut self, err: Option<ValidationError>) {
        self.err = err;
    }

    /// Error message.
    pub fn err(&self) -> Option<&ValidationError> {
        self.err.as_ref()
    }

    /// Help message.
    pub fn set_help(&mut self, help: Option<ValidationHelp>) {
        self.help = help;
    }

    /// Help message.
    pub fn help(&self) -> Option<&ValidationHelp> {
        self.help.as_ref()
    }
}
