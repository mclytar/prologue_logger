use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

pub trait Styler: Debug + Send + Sync {
    fn fmt_string(&self, f: &mut std::fmt::Formatter<'_>, template: StylerTemplate, obj: String) -> std::fmt::Result {
        self.fmt_str(f, template, &obj)
    }

    fn fmt_str(&self, f: &mut std::fmt::Formatter<'_>, _template: StylerTemplate, obj: &str) -> std::fmt::Result {
        Display::fmt(obj, f)
    }

    fn fmt_usize(&self, f: &mut std::fmt::Formatter<'_>, _template: StylerTemplate, obj: usize) -> std::fmt::Result {
        Display::fmt(&obj, f)
    }

    fn fmt_char(&self, f: &mut std::fmt::Formatter<'_>, _template: StylerTemplate, obj: char) -> std::fmt::Result {
        Display::fmt(&obj, f)
    }
}

pub trait Styled: Debug + Send + Sync {
    fn fmt_styled(&self, f: &mut std::fmt::Formatter<'_>, styler: &'static dyn Styler) -> std::fmt::Result;
}

#[derive(Clone, Debug)]
pub struct StyledLineStart<'a> {
    number: Option<usize>,
    sep_ch: char,
    number_template: StylerTemplate,
    sep_ch_template: StylerTemplate,
    styler: &'a dyn Styler
}
impl<'a> StyledLineStart<'a> {
    pub fn new(styler: &'a dyn Styler) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch: '|',
            number_template: StylerTemplate::Help,
            sep_ch_template: StylerTemplate::Help,
            styler
        }
    }

    pub fn new_bullet(styler: &'a dyn Styler) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch: '=',
            number_template: StylerTemplate::Help,
            sep_ch_template: StylerTemplate::Help,
            styler
        }
    }

    pub fn new_empty(styler: &'a dyn Styler) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch: ' ',
            number_template: StylerTemplate::Help,
            sep_ch_template: StylerTemplate::Help,
            styler
        }
    }

    pub fn new_plus(styler: &'a dyn Styler) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch: '+',
            number_template: StylerTemplate::Help,
            sep_ch_template: StylerTemplate::Add,
            styler
        }
    }

    pub fn new_minus(styler: &'a dyn Styler) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch: '-',
            number_template: StylerTemplate::Help,
            sep_ch_template: StylerTemplate::Remove,
            styler
        }
    }


    pub fn with_custom_separator(styler: &'a dyn Styler, sep_ch: char, sep_ch_template: StylerTemplate) -> StyledLineStart<'a> {
        StyledLineStart {
            number: None,
            sep_ch,
            number_template: StylerTemplate::Help,
            sep_ch_template,
            styler
        }
    }

    pub fn line_number(mut self, line_number: usize) -> Self {
        self.number = Some(line_number);
        self
    }

    pub fn line_number_template(mut self, line_number_template: StylerTemplate) -> Self {
        self.number_template = line_number_template;
        self
    }
}
impl<'a> Display for StyledLineStart<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(number) = self.number {
            self.styler.fmt_usize(f, self.number_template, number)?;
        } else {
            self.styler.fmt_str(f, self.number_template, "")?;
        }
        write!(f, " ")?;
        self.styler.fmt_char(f, self.sep_ch_template, self.sep_ch)?;
        write!(f, " ")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum StylerTemplate {
    Normal,
    Title,
    Warning,
    Error,
    Help,
    Note,
    Add,
    Remove
}
impl Default for StylerTemplate {
    fn default() -> Self {
        StylerTemplate::Normal
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct NoStyler;
impl Styler for NoStyler {}

/*#[cfg(feature = "console")]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct DefaultConsoleStyler;
#[cfg(feature = "console")]
impl Styler for DefaultConsoleStyler {
    fn fmt_title<D: Display>(&self, f: &mut Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(&console::style(obj).white().bright(), f)
    }

    fn fmt_help<D: Display>(&self, f: &mut Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(&console::style(obj).cyan().bright(), f)
    }
}*/

/*#[cfg(feature = "console")]
pub struct ConsoleColorStyle;
#[cfg(feature = "console")]
impl Style for ConsoleColorStyle {
    type StyledObject = console::StyledObject<String>;

    fn style<D: Display>(obj: D) -> Self::StyledObject {
        todo!()
    }
}*/