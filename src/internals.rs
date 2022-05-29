use std::fmt::{Debug, Display, Formatter};

#[cfg(not(feature = "console"))]
pub mod console {
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct StyledObject<D: Display>(D);
    impl<D: Display> Display for StyledObject<D> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            (&self.0 as &dyn Display).fmt(f)
        }
    }
    impl<D: Display> StyledObject<D> {
        #[allow(unused)]
        pub fn black(self) -> Self { self }
        #[allow(unused)]
        pub fn red(self) -> Self { self }
        #[allow(unused)]
        pub fn green(self) -> Self { self }
        #[allow(unused)]
        pub fn blue(self) -> Self { self }
        #[allow(unused)]
        pub fn cyan(self) -> Self { self }
        #[allow(unused)]
        pub fn magenta(self) -> Self { self }
        #[allow(unused)]
        pub fn yellow(self) -> Self { self }
        #[allow(unused)]
        pub fn white(self) -> Self { self }
        #[allow(unused)]
        pub fn bright(self) -> Self { self }
        #[allow(unused)]
        pub fn hidden(self) -> StyledObject<String> {
            let len = format!("{}", self.0).len();
            StyledObject(format!("{: >len$}", len = len))
        }
    }

    pub fn style<D: Display>(object: D) -> StyledObject<D> {
        StyledObject(object)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LineStart;
impl Display for LineStart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style("|").cyan().bright())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct NoteDash;
impl Display for NoteDash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style("=").cyan().bright())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Arrow;
impl Display for Arrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style("-->").cyan().bright())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Colon;
impl Display for Colon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style(":").white().bright())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Title<T: Display>(pub T);
impl<T: Display> Display for Title<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", console::style(&self.0).white().bright())
    }
}
#[derive(Copy, Clone, Debug)]
pub struct AnnotationUnderline(pub(super) super::EntryKind, pub usize);
impl Display for AnnotationUnderline {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == super::EntryKind::Help {
            write!(f, "{:->len$}", self.0.style(""), len = self.1)
        } else {
            write!(f, "{:^>len$}", self.0.style(""), len = self.1)
        }
    }
}