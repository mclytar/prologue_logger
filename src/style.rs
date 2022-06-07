use std::fmt::{Arguments, Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;

#[macro_export]
macro_rules! styled_write {
    ($dst:expr, $writer:expr, $style:expr, $($arg:tt)*) => ($writer.write_fmt($dst, $style, format_args!($($arg)*)))
}

macro_rules! impl_display_for_rich_display {
    [$t:ty] => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $crate::style::RichDisplay::fmt_styled(self, f, & $crate::style::StdWriter)
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
#[non_exhaustive]
pub enum Style {
    Normal,
    Title,
    Warning,
    Error,
    Help,
    Note,
    Add,
    Sub
}
impl Default for Style {
    fn default() -> Self {
        Style::Normal
    }
}

pub trait RichDisplay: Display {
    fn fmt_styled(&self, f: &mut std::fmt::Formatter<'_>, _: &dyn StyledWriter) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
impl<D: RichDisplay> RichDisplay for &D {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        self.deref().fmt_styled(f, writer)
    }
}
macro_rules! impl_se {
    [$($t:ty),*] => {
        $(impl RichDisplay for $t {} )*
    };
}
impl_se![String, str, &str];
impl_se![u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64];

pub trait StyledWriter: Debug + Send + Sync {
    fn write_fmt(&self, f: &mut std::fmt::Formatter<'_>, style: Style, args: Arguments<'_>) -> std::fmt::Result {
        if let Some(s) = args.as_str() {
            self.write_str(f, style, s)
        } else {
            self.write_str(f, style, &args.to_string())
        }
    }

    fn write_str(&self, f: &mut std::fmt::Formatter<'_>, _style: Style, obj: &str) -> std::fmt::Result {
        Display::fmt(obj, f)
    }

    fn write_usize(&self, f: &mut std::fmt::Formatter<'_>, style: Style, obj: usize) -> std::fmt::Result {
        self.write_fmt(f, style, format_args!("{}", obj))
    }

    fn write_char(&self, f: &mut std::fmt::Formatter<'_>, style: Style, obj: char) -> std::fmt::Result {
        self.write_str(f, style, &obj.to_string())
    }
}
impl StyledWriter for &dyn StyledWriter {
    fn write_fmt(&self, f: &mut Formatter<'_>, style: Style, args: Arguments<'_>) -> std::fmt::Result {
        self.deref().write_fmt(f, style, args)
    }

    fn write_str(&self, f: &mut Formatter<'_>, style: Style, obj: &str) -> std::fmt::Result {
        self.deref().write_str(f, style, obj)
    }

    fn write_usize(&self, f: &mut Formatter<'_>, style: Style, obj: usize) -> std::fmt::Result {
        self.deref().write_usize(f, style, obj)
    }

    fn write_char(&self, f: &mut Formatter<'_>, style: Style, obj: char) -> std::fmt::Result {
        self.deref().write_char(f, style, obj)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct RichMargin {
    number: Option<usize>,
    number_style: Style,
    margin_ch: char,
    margin_style: Style
}
impl Default for RichMargin {
    fn default() -> Self {
        RichMargin {
            number: None,
            number_style: Style::Help,
            margin_ch: '|',
            margin_style: Style::Help
        }
    }
}
impl Display for RichMargin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let std_writer = StdWriter;
        RichDisplay::fmt_styled(self, f, &std_writer)
    }
}
impl RichDisplay for RichMargin {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        // Get text length.
        let len = f.width().unwrap_or_else(|| {
            format!("{}", self.number.unwrap_or(0)).len()
        });
        if let Some(number) = self.number {
            styled_write!(f, writer, self.number_style, "{: <len$} ", number)?;
        } else {
            styled_write!(f, writer, self.number_style, "{: <len$} ", "")?;
        }
        styled_write!(f, writer, self.margin_style, "{}", self.margin_ch)?;
        Ok(())
    }
}
impl RichMargin {
    pub fn new() -> RichMargin {
        Default::default()
    }

    pub fn with_line_number(number: usize) -> RichMargin {
        RichMargin {
            number: Some(number),
            ..Default::default()
        }
    }

    pub fn diff_add() -> RichMargin {
        RichMargin {
            margin_ch: '+',
            margin_style: Style::Add,
            ..Default::default()
        }
    }

    pub fn diff_add_with_line_number(number: usize) -> RichMargin {
        RichMargin {
            number: Some(number),
            ..RichMargin::diff_add()
        }
    }

    pub fn diff_sub() -> RichMargin {
        RichMargin {
            margin_ch: '-',
            margin_style: Style::Sub,
            ..Default::default()
        }
    }

    pub fn diff_sub_with_line_number(number: usize) -> RichMargin {
        RichMargin {
            number: Some(number),
            ..RichMargin::diff_sub()
        }
    }

    pub fn new_dashed() -> RichMargin {
        RichMargin {
            margin_ch: '=',
            ..Default::default()
        }
    }

    pub fn new_empty() -> RichMargin {
        RichMargin {
            margin_ch: ' ',
            ..Default::default()
        }
    }
}

pub struct StyledItem<D: Display> {
    style: Style,
    text: D
}
impl<D: Display> Display for StyledItem<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RichDisplay::fmt_styled(self, f, &StdWriter)
    }
}
impl<D: Display> RichDisplay for StyledItem<D> {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        styled_write!(f, writer, self.style, "{}", self.text)
    }
}
impl<D: Display> StyledItem<D> {
    pub fn new(text: D) -> StyledItem<D> {
        StyledItem {
            style: Style::Normal,
            text
        }
    }

    pub fn with_style(text: D, style: Style) -> StyledItem<D> {
        StyledItem {
            style,
            text
        }
    }
}

pub struct Title<D: Display>(pub D);
impl<D: Display> Display for Title<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl<D: Display> RichDisplay for Title<D> {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        RichDisplay::fmt_styled(&StyledItem::with_style(&self.0, Style::Title), f, writer)
    }
}

pub struct Highlight {
    style: Style,
    h_ch: char,
    offset: usize,
    len: usize
}
impl_display_for_rich_display!(Highlight);
impl RichDisplay for Highlight {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        for _ in 0..self.offset {
            writer.write_char(f, self.style, ' ')?;
        }
        for _ in 0..self.len {
            writer.write_char(f, self.style, self.h_ch)?;
        }
        Ok(())
    }
}
impl Highlight {
    pub fn with_style(style: Style) -> Highlight {
        Highlight {
            style,
            h_ch: '^',
            offset: 0,
            len: 1
        }
    }

    pub fn offset(mut self, offset: usize) -> Highlight {
        self.offset = offset;
        self
    }

    pub fn len(mut self, len: usize) -> Highlight {
        self.len = len;
        self
    }

    pub fn char(mut self, ch: char) -> Highlight {
        self.h_ch = ch;
        self
    }
}

pub struct RightAligned<D>(pub D, pub usize);
impl<D: Display> Display for RightAligned<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RichDisplay::fmt_styled(self, f, &StdWriter)
    }
}
impl<D: Display> RichDisplay for RightAligned<D> {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        write!(f, "{: >len$}", StyledItem::new(&self.0), len = self.1)
    }
}

pub struct Width;
impl Display for Width {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        RichDisplay::fmt_styled(self, f, &StdWriter)
    }
}
impl RichDisplay for Width {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        let width = f.width().unwrap_or(0);
        styled_write!(f, writer, Style::Normal, "{: <width$}", "")
    }
}

pub struct RichLine {
    margin: Option<RichMargin>,
    text: Vec<Box<dyn RichDisplay>>
}
impl Default for RichLine {
    fn default() -> Self {
        RichLine {
            margin: None,
            text: Vec::new()
        }
    }
}
impl Display for RichLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let std_writer = StdWriter;
        RichDisplay::fmt_styled(self, f, &std_writer)
    }
}
impl RichDisplay for RichLine {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        if let Some(margin) = &self.margin {
            margin.fmt_styled(f, writer)?;
            write!(f, " ")?;
        }
        for item in self.text.iter() {
            item.fmt_styled(f, writer)?;
        }
        Ok(())
    }
}
impl RichLine {
    pub fn new() -> RichLine {
        Default::default()
    }

    pub fn with_default_margin() -> RichLine {
        RichLine {
            margin: Some(Default::default()),
            text: Vec::new()
        }
    }

    pub fn with_margin(margin: RichMargin) -> RichLine {
        RichLine {
            margin: Some(margin),
            text: Vec::new()
        }
    }

    pub fn push<D: RichDisplay + 'static>(&mut self, item: D) {
        self.text.push(Box::new(item))
    }
}

pub struct RichText {
    lines: Vec<RichLine>
}
impl Default for RichText {
    fn default() -> Self {
        RichText {
            lines: Vec::new()
        }
    }
}
impl Display for RichText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let std_writer = StdWriter;
        RichDisplay::fmt_styled(self, f, &std_writer)
    }
}
impl RichDisplay for RichText {
    fn fmt_styled(&self, f: &mut Formatter<'_>, writer: &dyn StyledWriter) -> std::fmt::Result {
        let max = self.lines.iter()
            .filter_map(|line| line.margin.as_ref().and_then(|margin| margin.number))
            .max()
            .unwrap_or(0);
        let len = format!("{}", max).len();
        for line in self.lines.iter() {
            write!(f, "{: <len$}", Styled(line, writer))?;
            //line.fmt_styled(f, writer)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
impl RichText {
    pub fn new() -> RichText {
        Default::default()
    }

    pub fn add_new_line(&mut self) {
        self.lines.push(RichLine::new());
    }

    pub fn add_new_line_with_default_margin(&mut self) {
        self.lines.push(RichLine::with_default_margin());
    }

    pub fn add_new_line_with_margin(&mut self, margin: RichMargin) {
        self.lines.push(RichLine::with_margin(margin));
    }

    pub fn add_text<D: RichDisplay + 'static>(&mut self, text: D) {
        if self.lines.is_empty() {
            self.lines.push(RichLine::new());
        }
        self.lines.last_mut().unwrap().push(text);
    }
}

pub struct Styled<D, S>(pub D, pub S);
impl<D: RichDisplay, S: StyledWriter> Display for Styled<D, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt_styled(f, &self.1)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct StdWriter;
impl StyledWriter for StdWriter {}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[cfg(feature = "console")]
pub struct ConsoleWriter;
#[cfg(feature = "console")]
impl StyledWriter for ConsoleWriter {
    fn write_str(&self, f: &mut Formatter<'_>, style: Style, obj: &str) -> std::fmt::Result {
        match style {
            Style::Normal => Display::fmt(obj, f),
            Style::Error => Display::fmt(&console::style(obj).red().bright(), f),
            Style::Warning => Display::fmt(&console::style(obj).yellow().bright(), f),
            Style::Note => Display::fmt(&console::style(obj).green().bright(), f),
            Style::Help => Display::fmt(&console::style(obj).cyan().bright(), f),
            Style::Title => Display::fmt(&console::style(obj).white().bright(), f),
            Style::Add => Display::fmt(&console::style(obj).green().bright(), f),
            Style::Sub => Display::fmt(&console::style(obj).green().bright(), f),
            _ => Display::fmt(obj, f)
        }
    }

    fn write_char(&self, f: &mut Formatter<'_>, style: Style, obj: char) -> std::fmt::Result {
        match style {
            Style::Normal => Display::fmt(&obj, f),
            Style::Error => Display::fmt(&console::style(obj).red().bright(), f),
            Style::Warning => Display::fmt(&console::style(obj).yellow().bright(), f),
            Style::Note => Display::fmt(&console::style(obj).green().bright(), f),
            Style::Help => Display::fmt(&console::style(obj).cyan().bright(), f),
            Style::Title => Display::fmt(&console::style(obj).white().bright(), f),
            Style::Add => Display::fmt(&console::style(obj).green().bright(), f),
            Style::Sub => Display::fmt(&console::style(obj).green().bright(), f),
            _ => Display::fmt(&obj, f)
        }
    }
}