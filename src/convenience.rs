use std::borrow::Cow;

use crate::algorithm::Printer;

impl Printer {
    pub fn ibox(&mut self, indent: isize) {
        self.inner.ibox(indent)
    }

    pub fn cbox(&mut self, indent: isize) {
        self.inner.cbox(indent)
    }

    pub fn end(&mut self) {
        self.inner.end();
    }

    pub fn word<S: Into<Cow<'static, str>>>(&mut self, wrd: S) {
        self.inner.word(wrd)
    }

    fn spaces(&mut self, n: usize) {
        self.inner.spaces(n)
    }

    pub fn zerobreak(&mut self) {
        self.inner.zerobreak()
    }

    pub fn space(&mut self) {
        self.inner.space()
    }

    pub fn nbsp(&mut self) {
        self.inner.nbsp()
    }

    pub fn hardbreak(&mut self) {
        self.inner.hardbreak()
    }

    pub fn space_if_nonempty(&mut self) {
        self.inner.space_if_nonempty()
    }

    pub fn hardbreak_if_nonempty(&mut self) {
        self.inner.hardbreak_if_nonempty()
    }

    pub fn trailing_comma(&mut self, is_last: bool) {
        self.inner.trailing_comma(is_last)
    }

    pub fn trailing_comma_or_space(&mut self, is_last: bool) {
        self.inner.trailing_comma_or_space(is_last)
    }

    pub fn neverbreak(&mut self) {
        self.inner.neverbreak();
    }
}
