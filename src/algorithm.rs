use leptosfmt_pretty_printer::{BeginToken, BreakToken};
use std::borrow::Cow;
use syn::Macro;

pub trait MacroFormatter {
    fn format(&mut self, printer: &mut leptosfmt_pretty_printer::Printer, mac: &Macro) -> bool;
}

pub struct Printer<'a> {
    pub inner: &'a mut leptosfmt_pretty_printer::Printer,
    pub macro_formatter: Option<&'a mut dyn MacroFormatter>,
}

impl<'a> Printer<'a> {
    pub fn new(
        printer: &'a mut leptosfmt_pretty_printer::Printer,
        macro_formatter: Option<&'a mut dyn MacroFormatter>,
    ) -> Self {
        Self {
            inner: printer,
            macro_formatter,
        }
    }

    pub fn settings(&self) -> &leptosfmt_pretty_printer::PrinterSettings {
        &self.inner.settings
    }

    pub fn indent(&self) -> isize {
        self.inner.settings.tab_spaces
    }

    pub fn scan_begin(&mut self, token: BeginToken) {
        self.inner.scan_begin(token);
    }

    pub fn scan_end(&mut self) {
        self.inner.scan_end()
    }

    pub fn scan_break(&mut self, token: BreakToken) {
        self.inner.scan_break(token);
    }

    pub fn scan_string(&mut self, string: Cow<'static, str>) {
        self.inner.scan_string(string);
    }

    pub fn offset(&mut self, offset: isize) {
        self.inner.offset(offset)
    }

    pub fn end_with_max_width(&mut self, max: isize) {
        self.inner.end_with_max_width(max)
    }

    pub fn ends_with(&self, c: char) -> bool {
        self.inner.ends_with(c)
    }
}
