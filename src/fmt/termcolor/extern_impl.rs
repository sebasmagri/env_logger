use std::error::Error;
use std::borrow::Cow;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;
use std::cell::RefCell;
use std::rc::Rc;

use log::Level;
use termcolor::{self, ColorChoice, ColorSpec, WriteColor};

use ::WriteStyle;
use ::fmt::Formatter;

pub(in ::fmt) mod pub_use_in_super {
    pub use super::*;
}

impl Formatter {
    /// Begin a new [`Style`].
    ///
    /// # Examples
    ///
    /// Create a bold, red colored style and use it to print the log level:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut level_style = buf.style();
    ///
    ///     level_style.set_color(Color::Red).set_bold(true);
    ///
    ///     writeln!(buf, "{}: {}",
    ///         level_style.value(record.level()),
    ///         record.args())
    /// });
    /// ```
    ///
    /// [`Style`]: struct.Style.html
    pub fn style(&self) -> Style {
        Style {
            buf: self.buf.clone(),
            spec: ColorSpec::new(),
        }
    }

    /// Get the default [`Style`] for the given level.
    /// 
    /// The style can be used to print other values besides the level.
    pub fn default_level_style(&self, level: Level) -> Style {
        let mut level_style = self.style();
        match level {
            Level::Trace => level_style.set_color(Color::White),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Info => level_style.set_color(Color::Green),
            Level::Warn => level_style.set_color(Color::Yellow),
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
        };
        level_style
    }

    /// Get a printable [`Style`] for the given level.
    /// 
    /// The style can only be used to print the level.
    pub fn default_styled_level(&self, level: Level) -> StyledValue<'static, Level> {
        self.default_level_style(level).into_value(level)
    }
}

pub(in ::fmt) struct BufferWriter(termcolor::BufferWriter);
pub(in ::fmt) struct Buffer(termcolor::Buffer);

impl BufferWriter {
    pub(in ::fmt) fn stderr(write_style: WriteStyle) -> Self {
        BufferWriter(termcolor::BufferWriter::stderr(write_style.into_color_choice()))
    }

    pub(in ::fmt) fn stdout(write_style: WriteStyle) -> Self {
        BufferWriter(termcolor::BufferWriter::stdout(write_style.into_color_choice()))
    }

    pub(in ::fmt) fn buffer(&self) -> Buffer {
        Buffer(self.0.buffer())
    }

    pub(in ::fmt) fn print(&self, buf: &Buffer) -> io::Result<()> {
        self.0.print(&buf.0)
    }
}

impl Buffer {
    pub(in ::fmt) fn clear(&mut self) {
        self.0.clear()
    }

    pub(in ::fmt) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub(in ::fmt) fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.0.set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.0.reset()
    }
}

impl WriteStyle {
    fn into_color_choice(self) -> ColorChoice {
        match self {
            WriteStyle::Always => ColorChoice::Always,
            WriteStyle::Auto => ColorChoice::Auto,
            WriteStyle::Never => ColorChoice::Never,
        }
    }
}

/// A set of styles to apply to the terminal output.
///
/// Call [`Formatter::style`] to get a `Style` and use the builder methods to
/// set styling properties, like [color] and [weight].
/// To print a value using the style, wrap it in a call to [`value`] when the log
/// record is formatted.
///
/// # Examples
///
/// Create a bold, red colored style and use it to print the log level:
///
/// ```
/// use std::io::Write;
/// use env_logger::fmt::Color;
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(|buf, record| {
///     let mut level_style = buf.style();
///
///     level_style.set_color(Color::Red).set_bold(true);
///
///     writeln!(buf, "{}: {}",
///         level_style.value(record.level()),
///         record.args())
/// });
/// ```
///
/// Styles can be re-used to output multiple values:
///
/// ```
/// use std::io::Write;
/// use env_logger::fmt::Color;
///
/// let mut builder = env_logger::Builder::new();
///
/// builder.format(|buf, record| {
///     let mut bold = buf.style();
///
///     bold.set_bold(true);
///
///     writeln!(buf, "{}: {} {}",
///         bold.value(record.level()),
///         bold.value("some bold text"),
///         record.args())
/// });
/// ```
///
/// [`Formatter::style`]: struct.Formatter.html#method.style
/// [color]: #method.set_color
/// [weight]: #method.set_bold
/// [`value`]: #method.value
#[derive(Clone)]
pub struct Style {
    buf: Rc<RefCell<Buffer>>,
    spec: ColorSpec,
}

/// A value that can be printed using the given styles.
///
/// It is the result of calling [`Style::value`].
///
/// [`Style::value`]: struct.Style.html#method.value
pub struct StyledValue<'a, T> {
    style: Cow<'a, Style>,
    value: T,
}

impl Style {
    /// Set the text color.
    ///
    /// # Examples
    ///
    /// Create a style with red text:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_color(Color::Red);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_color(&mut self, color: Color) -> &mut Style {
        self.spec.set_fg(color.into_termcolor());
        self
    }

    /// Set the text weight.
    ///
    /// If `yes` is true then text will be written in bold.
    /// If `yes` is false then text will be written in the default weight.
    ///
    /// # Examples
    ///
    /// Create a style with bold text:
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_bold(true);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_bold(&mut self, yes: bool) -> &mut Style {
        self.spec.set_bold(yes);
        self
    }

    /// Set the text intensity.
    ///
    /// If `yes` is true then text will be written in a brighter color.
    /// If `yes` is false then text will be written in the default color.
    ///
    /// # Examples
    ///
    /// Create a style with intense text:
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_intense(true);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_intense(&mut self, yes: bool) -> &mut Style {
        self.spec.set_intense(yes);
        self
    }

    /// Set the background color.
    ///
    /// # Examples
    ///
    /// Create a style with a yellow background:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_bg(Color::Yellow);
    ///
    ///     writeln!(buf, "{}", style.value(record.args()))
    /// });
    /// ```
    pub fn set_bg(&mut self, color: Color) -> &mut Style {
        self.spec.set_bg(color.into_termcolor());
        self
    }

    /// Wrap a value in the style.
    ///
    /// The same `Style` can be used to print multiple different values.
    ///
    /// # Examples
    ///
    /// Create a bold, red colored style and use it to print the log level:
    ///
    /// ```
    /// use std::io::Write;
    /// use env_logger::fmt::Color;
    ///
    /// let mut builder = env_logger::Builder::new();
    ///
    /// builder.format(|buf, record| {
    ///     let mut style = buf.style();
    ///
    ///     style.set_color(Color::Red).set_bold(true);
    ///
    ///     writeln!(buf, "{}: {}",
    ///         style.value(record.level()),
    ///         record.args())
    /// });
    /// ```
    pub fn value<T>(&self, value: T) -> StyledValue<T> {
        StyledValue {
            style: Cow::Borrowed(&self),
            value
        }
    }

    fn into_value<T>(self, value: T) -> StyledValue<'static, T> {
        StyledValue {
            style: Cow::Owned(self),
            value
        }
    }
}

impl<'a, T> StyledValue<'a, T> {
    fn write_fmt<F>(&self, f: F) -> fmt::Result
    where
        F: FnOnce() -> fmt::Result,
    {
        self.style.buf.borrow_mut().set_color(&self.style.spec).map_err(|_| fmt::Error)?;

        // Always try to reset the terminal style, even if writing failed
        let write = f();
        let reset = self.style.buf.borrow_mut().reset().map_err(|_| fmt::Error);

        write.and(reset)
    }
}

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        f.debug_struct("Style").field("spec", &self.spec).finish()
    }
}

macro_rules! impl_styled_value_fmt {
    ($($fmt_trait:path),*) => {
        $(
            impl<'a, T: $fmt_trait> $fmt_trait for StyledValue<'a, T> {
                fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
                    self.write_fmt(|| T::fmt(&self.value, f))
                }
            }
        )*
    };
}

impl_styled_value_fmt!(
    fmt::Debug,
    fmt::Display,
    fmt::Pointer,
    fmt::Octal,
    fmt::Binary,
    fmt::UpperHex,
    fmt::LowerHex,
    fmt::UpperExp,
    fmt::LowerExp);

// The `Color` type is copied from https://github.com/BurntSushi/ripgrep/tree/master/termcolor

/// The set of available colors for the terminal foreground/background.
///
/// The `Ansi256` and `Rgb` colors will only output the correct codes when
/// paired with the `Ansi` `WriteColor` implementation.
///
/// The `Ansi256` and `Rgb` color types are not supported when writing colors
/// on Windows using the console. If they are used on Windows, then they are
/// silently ignored and no colors will be emitted.
///
/// This set may expand over time.
///
/// This type has a `FromStr` impl that can parse colors from their human
/// readable form. The format is as follows:
///
/// 1. Any of the explicitly listed colors in English. They are matched
///    case insensitively.
/// 2. A single 8-bit integer, in either decimal or hexadecimal format.
/// 3. A triple of 8-bit integers separated by a comma, where each integer is
///    in decimal or hexadecimal format.
///
/// Hexadecimal numbers are written with a `0x` prefix.
#[allow(missing_docs)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    Cyan,
    Magenta,
    Yellow,
    White,
    Ansi256(u8),
    Rgb(u8, u8, u8),
    #[doc(hidden)]
    __Nonexhaustive,
}

/// An error from parsing an invalid color specification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseColorError(ParseColorErrorKind);

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseColorErrorKind {
    /// An error originating from `termcolor`.
    TermColor(termcolor::ParseColorError),
    /// An error converting the `termcolor` color to a `env_logger::Color`.
    /// 
    /// This variant should only get reached if a user uses a new spec that's
    /// valid for `termcolor`, but not recognised in `env_logger` yet.
    Unrecognized {
        given: String,
    }
}

impl ParseColorError {
    fn termcolor(err: termcolor::ParseColorError) -> Self {
        ParseColorError(ParseColorErrorKind::TermColor(err))
    }

    fn unrecognized(given: String) -> Self {
        ParseColorError(ParseColorErrorKind::Unrecognized { given })
    }

    /// Return the string that couldn't be parsed as a valid color.
    pub fn invalid(&self) -> &str {
        match self.0 {
            ParseColorErrorKind::TermColor(ref err) => err.invalid(),
            ParseColorErrorKind::Unrecognized { ref given, .. } => given,
        }
    }
}

impl Error for ParseColorError {
    fn description(&self) -> &str {
        match self.0 {
            ParseColorErrorKind::TermColor(ref err) => err.description(),
            ParseColorErrorKind::Unrecognized { .. } => "unrecognized color value",
        }
    }
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ParseColorErrorKind::TermColor(ref err) => fmt::Display::fmt(err, f),
            ParseColorErrorKind::Unrecognized { ref given, .. } => {
                write!(f, "unrecognized color value '{}'", given)
            } 
        }
    }
}

impl Color {
    fn into_termcolor(self) -> Option<termcolor::Color> {
        match self {
            Color::Black => Some(termcolor::Color::Black),
            Color::Blue => Some(termcolor::Color::Blue),
            Color::Green => Some(termcolor::Color::Green),
            Color::Red => Some(termcolor::Color::Red),
            Color::Cyan => Some(termcolor::Color::Cyan),
            Color::Magenta => Some(termcolor::Color::Magenta),
            Color::Yellow => Some(termcolor::Color::Yellow),
            Color::White => Some(termcolor::Color::White),
            Color::Ansi256(value) => Some(termcolor::Color::Ansi256(value)),
            Color::Rgb(r, g, b) => Some(termcolor::Color::Rgb(r, g, b)),
            _ => None,
        }
    }

    fn from_termcolor(color: termcolor::Color) -> Option<Color> {
        match color {
            termcolor::Color::Black => Some(Color::Black),
            termcolor::Color::Blue => Some(Color::Blue),
            termcolor::Color::Green => Some(Color::Green),
            termcolor::Color::Red => Some(Color::Red),
            termcolor::Color::Cyan => Some(Color::Cyan),
            termcolor::Color::Magenta => Some(Color::Magenta),
            termcolor::Color::Yellow => Some(Color::Yellow),
            termcolor::Color::White => Some(Color::White),
            termcolor::Color::Ansi256(value) => Some(Color::Ansi256(value)),
            termcolor::Color::Rgb(r, g, b) => Some(Color::Rgb(r, g, b)),
            _ => None,
        }
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Color, ParseColorError> {
        let tc = termcolor::Color::from_str(s).map_err(ParseColorError::termcolor)?;
        Color::from_termcolor(tc).ok_or_else(|| ParseColorError::unrecognized(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color_name_valid() {
        let inputs = vec![
            "black",
            "blue",
            "green",
            "red",
            "cyan",
            "magenta",
            "yellow",
            "white",
        ];

        for input in inputs {
            assert!(Color::from_str(input).is_ok());
        }
    }

    #[test]
    fn parse_color_ansi_valid() {
        let inputs = vec![
            "7",
            "32",
            "0xFF",
        ];

        for input in inputs {
            assert!(Color::from_str(input).is_ok());
        }
    }

    #[test]
    fn parse_color_rgb_valid() {
        let inputs = vec![
            "0,0,0",
            "0,128,255",
            "0x0,0x0,0x0",
            "0x33,0x66,0xFF",
        ];

        for input in inputs {
            assert!(Color::from_str(input).is_ok());
        }
    }

    #[test]
    fn parse_color_invalid() {
        let inputs = vec![
            "not_a_color",
            "256",
            "0,0",
            "0,0,256",
        ];

        for input in inputs {
            let err = Color::from_str(input).unwrap_err();
            assert_eq!(input, err.invalid());
        }
    }
}