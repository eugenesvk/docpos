use crate::Attribute;

const DOCPOS_CRATE: &str = "docpos";
const DOCPOS_MACRO: &str = "docpos";

/// check whether an attribute is the raw #[argdocpos] main attribute.
#[inline(always)]pub fn is_docpos_main(attr: &Attribute) -> bool {
  let path = attr.path();
  if path.is_ident(DOCPOS_MACRO) {true
  } else { path.segments.len() == 2 // checks for (::)docpos::docpos
    &&     path.segments[0].ident == DOCPOS_CRATE
    &&     path.segments[1].ident == DOCPOS_MACRO
  }
}
