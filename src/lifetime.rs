use crate::algorithm::Printer;
use syn::Lifetime;

impl Printer<'_> {
    pub fn lifetime(&mut self, lifetime: &Lifetime) {
        self.word("'");
        self.ident(&lifetime.ident);
    }
}
