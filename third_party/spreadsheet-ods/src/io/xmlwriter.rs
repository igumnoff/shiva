use std::fmt;
use std::fmt::{Display, Formatter, Write as FmtWrite};
use std::io::{self, Write};
#[cfg(not(feature = "check_xml"))]
use std::marker::PhantomData;
use std::str::from_utf8_unchecked;

#[derive(PartialEq)]
enum Open {
    None,
    Elem,
    Empty,
}

impl Display for Open {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Open::None => f.write_str("None")?,
            Open::Elem => f.write_str("Elem")?,
            Open::Empty => f.write_str("Empty")?,
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Stack {
    #[cfg(feature = "check_xml")]
    stack: Vec<String>,
    #[cfg(not(feature = "check_xml"))]
    stack: PhantomData<String>,
}

#[cfg(feature = "check_xml")]
impl Stack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, name: &str) {
        self.stack.push(name.to_string());
    }

    fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(not(feature = "check_xml"))]
impl Stack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, _name: &str) {}

    fn pop(&mut self) -> Option<String> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }
}

/// The XmlWriter himself
pub(crate) struct XmlWriter<W: Write> {
    writer: Box<W>,
    buf: String,
    stack: Stack,
    open: Open,
    line_break: bool,

    // short time temp space
    tmp: Vec<u8>,
    tmp2: Vec<u8>,
}

impl<W: Write> fmt::Debug for XmlWriter<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "XmlWriter {{ stack: {:?}, opened: {} }}",
            self.stack, self.open
        )
    }
}

impl<W: Write> XmlWriter<W> {
    /// Create a new writer, by passing an `io::Write`
    pub(crate) fn new(writer: W) -> XmlWriter<W> {
        XmlWriter {
            stack: Stack::new(),
            buf: Default::default(),
            writer: Box::new(writer),
            open: Open::None,
            line_break: false,
            tmp: Default::default(),
            tmp2: Default::default(),
        }
    }

    pub(crate) fn line_break(mut self, line_break: bool) -> Self {
        self.line_break = line_break;
        self
    }

    /// Write the DTD. You have to take care of the encoding
    /// on the underlying Write yourself.
    pub(crate) fn dtd(&mut self, encoding: &str) -> io::Result<()> {
        self.buf.push_str("<?xml version=\"1.0\" encoding=\"");
        self.buf.push_str(encoding);
        self.buf.push_str("\" ?>\n");
        if self.line_break {
            self.buf.push('\n');
        }

        Ok(())
    }

    /// Write an element with inlined text (not escaped)
    pub(crate) fn elem_text<T: Display + ?Sized>(
        &mut self,
        name: &str,
        text: &T,
    ) -> io::Result<()> {
        self.close_elem()?;

        self.buf.push('<');
        self.buf.push_str(name);
        self.buf.push('>');

        let _ = write!(self.buf, "{}", text);

        self.buf.push('<');
        self.buf.push('/');
        self.buf.push_str(name);
        self.buf.push('>');
        if self.line_break {
            self.buf.push('\n');
        }

        Ok(())
    }

    /// Write an optional element with inlined text (escaped).
    /// If text.is_empty() the element is not written at all.
    pub(crate) fn elem_text_esc<T: Display + ?Sized>(
        &mut self,
        name: &str,
        text: &T,
    ) -> io::Result<()> {
        self.close_elem()?;

        self.buf.push('<');
        self.buf.push_str(name);
        self.buf.push('>');

        self.escape(text)?;

        self.buf.push('<');
        self.buf.push('/');
        self.buf.push_str(name);
        self.buf.push('>');
        if self.line_break {
            self.buf.push('\n');
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn comment(&mut self, comment: &str) -> io::Result<()> {
        self.close_elem()?;

        self.buf.push_str("<!--");
        self.buf.push_str(comment);
        self.buf.push_str("-->");
        if self.line_break {
            self.buf.push('\n');
        }

        Ok(())
    }

    /// Begin an elem, make sure name contains only allowed chars
    pub(crate) fn elem(&mut self, name: &str) -> io::Result<()> {
        self.close_elem()?;

        self.stack.push(name);

        self.buf.push('<');
        self.open = Open::Elem;
        self.buf.push_str(name);
        Ok(())
    }

    /// Begin an elem if has_content is true, otherwise begin a empty elem.
    pub(crate) fn elem_if(&mut self, has_content: bool, name: &str) -> io::Result<()> {
        self.close_elem()?;

        if has_content {
            self.stack.push(name);
        }

        self.buf.push('<');
        self.open = if has_content { Open::Elem } else { Open::Empty };
        self.buf.push_str(name);
        Ok(())
    }

    /// Begin an empty elem
    pub(crate) fn empty(&mut self, name: &str) -> io::Result<()> {
        self.close_elem()?;

        self.buf.push('<');
        self.open = Open::Empty;
        self.buf.push_str(name);
        Ok(())
    }

    /// Close an elem if open, do nothing otherwise
    fn close_elem(&mut self) -> io::Result<()> {
        match self.open {
            Open::None => {}
            Open::Elem => {
                self.buf.push('>');
            }
            Open::Empty => {
                self.buf.push('/');
                self.buf.push('>');
                if self.line_break {
                    self.buf.push('\n');
                }
            }
        }
        self.open = Open::None;
        self.write_buf()?;
        Ok(())
    }

    /// Write an attr, make sure name and value contain only allowed chars.
    /// For an escaping version use `attr_esc`
    pub(crate) fn attr_str(&mut self, name: &'static str, value: &'static str) -> io::Result<()> {
        if cfg!(feature = "check_xml") && self.open == Open::None {
            panic!(
                "Attempted to write attr to elem, when no elem was opened, stack {:?}",
                self.stack
            );
        }
        self.buf.push(' ');
        self.buf.push_str(name);
        self.buf.push('=');
        self.buf.push('"');
        self.buf.push_str(value);
        self.buf.push('"');
        Ok(())
    }

    /// Write an attr, make sure name and value contain only allowed chars.
    /// For an escaping version use `attr_esc`
    pub(crate) fn attr<T: Display + ?Sized>(&mut self, name: &str, value: &T) -> io::Result<()> {
        if cfg!(feature = "check_xml") && self.open == Open::None {
            panic!(
                "Attempted to write attr to elem, when no elem was opened, stack {:?}",
                self.stack
            );
        }

        self.buf.push(' ');
        self.buf.push_str(name);
        self.buf.push('=');
        self.buf.push('"');
        let _ = write!(self.buf, "{}", value);
        self.buf.push('"');
        Ok(())
    }

    /// Write an attr,  make sure name contains only allowed chars
    pub(crate) fn attr_esc<T: Display + ?Sized>(
        &mut self,
        name: &str,
        value: &T,
    ) -> io::Result<()> {
        if cfg!(feature = "check_xml") && self.open == Open::None {
            panic!(
                "Attempted to write attr to elem, when no elem was opened, stack {:?}",
                self.stack
            );
        }
        self.buf.push(' ');
        self.escape_name(name)?;
        self.buf.push('=');
        self.buf.push('"');
        self.escape(value)?;
        self.buf.push('"');
        Ok(())
    }

    /// Escape text
    fn escape<T: Display + ?Sized>(&mut self, text: &T) -> io::Result<()> {
        fn escape_impl(text: &[u8], tmp2: &mut Vec<u8>) {
            tmp2.clear();
            for c in text {
                match c {
                    b'"' => {
                        let _ = write!(tmp2, "&quot;");
                    }
                    b'\'' => {
                        let _ = write!(tmp2, "&apos;");
                    }
                    b'&' => {
                        let _ = write!(tmp2, "&amp;");
                    }
                    b'<' => {
                        let _ = write!(tmp2, "&lt;");
                    }
                    b'>' => {
                        let _ = write!(tmp2, "&gt;");
                    }
                    _ => tmp2.push(*c),
                };
            }
        }

        self.tmp.clear();
        let _ = write!(self.tmp, "{}", text);
        escape_impl(&self.tmp, &mut self.tmp2);
        // Safety: this is always from a string buffer.
        unsafe {
            self.buf.push_str(from_utf8_unchecked(&self.tmp2));
        }

        Ok(())
    }

    /// Escape identifiers
    fn escape_name<T: Display + ?Sized>(&mut self, text: &T) -> io::Result<()> {
        fn escape_impl(text: &[u8], tmp2: &mut Vec<u8>) {
            tmp2.clear();
            for c in text {
                match c {
                    b'"' => {
                        let _ = write!(tmp2, "&quot;");
                    }
                    b'\'' => {
                        let _ = write!(tmp2, "&apos;");
                    }
                    b'&' => {
                        let _ = write!(tmp2, "&amp;");
                    }
                    b'<' => {
                        let _ = write!(tmp2, "&lt;");
                    }
                    b'>' => {
                        let _ = write!(tmp2, "&gt;");
                    }
                    b'\\' => {
                        let _ = write!(tmp2, "\\\\");
                    }
                    _ => tmp2.push(*c),
                };
            }
        }

        self.tmp.clear();
        let _ = write!(self.tmp, "{}", text);
        escape_impl(&self.tmp, &mut self.tmp2);
        // Safety: this is always from a string buffer.
        unsafe {
            self.buf.push_str(from_utf8_unchecked(&self.tmp2));
        }

        Ok(())
    }

    /// Write a text, doesn't escape the text.
    pub(crate) fn text_str(&mut self, text: &'static str) -> io::Result<()> {
        self.close_elem()?;
        self.buf.push_str(text);
        Ok(())
    }

    /// Write a text, doesn't escape the text.
    pub(crate) fn text<T: Display + ?Sized>(&mut self, text: &T) -> io::Result<()> {
        self.close_elem()?;
        let _ = write!(self.buf, "{}", text);
        Ok(())
    }

    /// Write a text, escapes the text automatically
    pub(crate) fn text_esc<T: Display + ?Sized>(&mut self, text: &T) -> io::Result<()> {
        self.close_elem()?;
        self.escape(text)?;
        Ok(())
    }

    /// End an elem. Only checks the stack and writes the end tag if has_content is true.
    pub(crate) fn end_elem_if(&mut self, has_content: bool, name: &str) -> io::Result<()> {
        self.close_elem()?;

        if !has_content {
            return Ok(());
        }

        if cfg!(feature = "check_xml") {
            match self.stack.pop() {
                Some(test) => {
                    if name != test {
                        panic!(
                            "Attempted to close elem {} but the open was {}, stack {:?}",
                            name, test, self.stack
                        )
                    }
                }
                None => panic!(
                    "Attempted to close an elem, when none was open, stack {:?}",
                    self.stack
                ),
            }
        }

        self.buf.push('<');
        self.buf.push('/');
        self.buf.push_str(name);
        self.buf.push('>');
        if self.line_break {
            self.buf.push('\n');
        }

        Ok(())
    }

    /// End an elem. Writes the end-tag
    #[inline(always)]
    pub(crate) fn end_elem(&mut self, name: &str) -> io::Result<()> {
        self.end_elem_if(true, name)
    }

    fn write_buf(&mut self) -> io::Result<()> {
        self.writer.write_all(self.buf.as_bytes())?;
        self.buf.clear();
        Ok(())
    }

    /// Fails if there are any open elements.
    pub(crate) fn close(&mut self) -> io::Result<()> {
        self.write_buf()?;

        if cfg!(feature = "check_xml") && !self.stack.is_empty() {
            panic!(
                "Attempted to close the xml, but there are open elements on the stack {:?}",
                self.stack
            )
        }
        Ok(())
    }
}
