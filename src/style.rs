use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

pub struct ObjectWithStyler<O, STYLER: Styler = NoStyler> {
    obj: O,
    fmt: fn(&mut std::fmt::Formatter<'_>, &O) -> std::fmt::Result,
    _styler: PhantomData<STYLER>
}
impl<O: Display, STYLER: Styler> Display for ObjectWithStyler<O, STYLER> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let fmt = self.fmt;
        fmt(f, &self.obj)
    }
}

pub fn title<D: Display, STYLER: Styler>(obj: D) -> ObjectWithStyler<D, STYLER> {
    ObjectWithStyler { obj, fmt: STYLER::fmt_title, _styler: PhantomData }
}
pub fn help<D: Display, STYLER: Styler>(obj: D) -> ObjectWithStyler<D, STYLER> {
    ObjectWithStyler { obj, fmt: STYLER::fmt_help, _styler: PhantomData }
}
pub fn note_start<STYLER: Styler>(offset: usize) -> ObjectWithStyler<String, STYLER> {
    let obj = format!("{: >len$}", "=", len = offset + 2);
    ObjectWithStyler { obj, fmt: STYLER::fmt_help, _styler: PhantomData }
}
pub fn note_continue<STYLER: Styler>(offset: usize) -> ObjectWithStyler<String, STYLER> {
    let obj = format!("{: >len$}", " ", len = offset + 2);
    ObjectWithStyler { obj, fmt: STYLER::fmt_help, _styler: PhantomData }
}


pub trait Styler: Copy + Clone + Debug + Default + Eq + PartialEq + Hash {
    fn fmt_title<D: Display>(f: &mut std::fmt::Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(obj, f)
    }
    fn fmt_warning<D: Display>(f: &mut std::fmt::Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(obj, f)
    }
    fn fmt_error<D: Display>(f: &mut std::fmt::Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(obj, f)
    }
    fn fmt_help<D: Display>(f: &mut std::fmt::Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(obj, f)
    }
    fn fmt_note<D: Display>(f: &mut std::fmt::Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(obj, f)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct NoStyler;
impl Styler for NoStyler {}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct DefaultConsoleStyler;
impl Styler for DefaultConsoleStyler {
    fn fmt_title<D: Display>(f: &mut Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(&console::style(obj).white().bright(), f)
    }

    fn fmt_help<D: Display>(f: &mut Formatter<'_>, obj: &D) -> std::fmt::Result {
        Display::fmt(&console::style(obj).cyan().bright(), f)
    }
}

/*#[cfg(feature = "console")]
pub struct ConsoleColorStyle;
#[cfg(feature = "console")]
impl Style for ConsoleColorStyle {
    type StyledObject = console::StyledObject<String>;

    fn style<D: Display>(obj: D) -> Self::StyledObject {
        todo!()
    }
}*/