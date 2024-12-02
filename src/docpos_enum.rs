use crate::*;
use crate::util::make_doc_block;
use crate::try2 as try2;
use quote::quote;
use syn::{Attribute, ItemEnum};
use crate::util_enum::extract_doc_variants_shift_up;


/// Document Enum arguments, but after them, not before
pub fn docpos_enum(mut enm:ItemEnum) -> proc_macro::TokenStream {
  try2!(enm.attrs.iter_mut().try_for_each(|attr| {
    if is_docpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
    } else                  {Ok(())}}));

  let enum_docs = try2!(extract_struct_doc_attrs(&mut enm.attrs)); // extrac the doc attributes on the enum itself

  let (doc_params_to_enm, doc_variants) = try2!(extract_doc_variants_shift_up(enm.variants.iter_mut()));
  let maybe_empty_doc_par_to_enum: Vec<Attribute> = doc_params_to_enm.unwrap_or_else(|| vec![]);
  // let documented_generics     = try2!(extract_documented_generics_shift_up(&mut enm.generics));
  let has_doc_variants   = !doc_variants  .is_empty();
  // let has_documented_generics = !documented_generics.is_empty();

  // if !has_doc_variants && !has_documented_generics {
  if !has_doc_variants {return syn::Error::new_spanned(enm.ident,"Enum has no documented variants or generics. Document at least one.",).into_compile_error().into();}

  let parameter_doc_block = make_doc_block("Variants", doc_variants);
  // let generics_doc_block  = make_doc_block("Generics", documented_generics);

  let docs_before = enum_docs.before_args_section;
  let docs_after  = enum_docs.after_args_section;
  let maybe_empty_doc_line = if !docs_after.is_empty() {Some(quote! {#[doc=""]})
  } else                                               {None};

  quote! {#(#docs_before)* #(#maybe_empty_doc_par_to_enum)*
    #parameter_doc_block
  //     #generics_doc_block
    #maybe_empty_doc_line
    #(#docs_after)*
    #enm
  }.into()
}
