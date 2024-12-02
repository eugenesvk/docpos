use crate::Attribute;

/// check whether an attribute is the raw #[argdocpos] main attribute.
#[inline(always)]pub fn is_docpos_main(attr: &Attribute) -> bool {attr.path().is_ident("docpos")}
