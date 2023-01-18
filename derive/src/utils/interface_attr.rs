use std::ops::Deref;

use darling::FromMeta;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::Lit;

#[derive(Debug, Clone)]
pub struct InterfaceAttr {
    pub inner: String,
    pub span: Span,
}

impl Deref for InterfaceAttr {
    type Target = String;

    fn deref(&self) -> &String {
        &self.inner
    }
}

impl Spanned for InterfaceAttr {
    fn span(&self) -> Span {
        self.span
    }
}

impl FromMeta for InterfaceAttr {
    fn from_value(value: &Lit) -> darling::Result<Self> {
        match value {
            Lit::Str(s) => Ok(InterfaceAttr {
                inner: s.value(),
                span: s.span(),
            }),
            _ => Err(darling::Error::unsupported_shape(
                "expected a string literal",
            )),
        }
    }
}
