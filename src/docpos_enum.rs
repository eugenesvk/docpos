use crate::*;
use crate::util::make_doc_block;
use crate::try2 as try2;
use quote::quote;
use syn::{Attribute, ItemStruct};


/// Document Enum arguments, but after them, not before
pub fn docpos_enum(mut enm:ItemEnum) -> proc_macro::TokenStream {
  try2!(enm.attrs.iter_mut().try_for_each(|attr| {
    if is_docpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
    } else                  {Ok(())}}));

  let enm_docs = try2!(extract_struct_doc_attrs(&mut enm.attrs)); // extrac the doc attributes on the struct itself

  let (doc_params_to_enm, doc_fields) = try2!(extract_doc_fields_shift_up(enm.fields.iter_mut()));
  let maybe_empty_doc_par_to_fn: Vec<Attribute> = doc_params_to_enm.unwrap_or_else(|| vec![]);
  // let documented_generics     = try2!(extract_documented_generics_shift_up(&mut enm.generics));
  let has_doc_fields   = !doc_fields  .is_empty();
  // let has_documented_generics = !documented_generics.is_empty();

  // if !has_doc_fields && !has_documented_generics {
  if !has_doc_fields {return syn::Error::new_spanned(enm.ident,"Struct has no documented fields or generics. Document at least one.",).into_compile_error().into();}

  let parameter_doc_block = make_doc_block("Fields", doc_fields);
  // let generics_doc_block  = make_doc_block("Generics", documented_generics);

  let docs_before = enm_docs.before_args_section;
  let docs_after  = enm_docs.after_args_section;
  let maybe_empty_doc_line = if !docs_after.is_empty() {Some(quote! {#[doc=""]})
  } else                                               {None};

  quote! {#(#docs_before)* #(#maybe_empty_doc_par_to_fn)*
    #parameter_doc_block
  //     #generics_doc_block
    #maybe_empty_doc_line
    #(#docs_after)*
    #enm
  }.into()
}
