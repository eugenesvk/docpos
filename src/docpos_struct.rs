use crate::*;
use crate::util::make_doc_block;
use crate::try2 as try2;
use crate::util_struct::extract_doc_fields_shift_up;
use quote::quote;
use syn::{Attribute, ItemStruct};


/// Document struct arguments, but after them, not before
pub fn docpos_struct(mut strct:ItemStruct) -> proc_macro::TokenStream {
  try2!(strct.attrs.iter_mut().try_for_each(|attr| {
    if is_docpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
    } else                  {Ok(())}}));

  let struct_docs = try2!(extract_struct_doc_attrs(&mut strct.attrs)); // extrac the doc attributes on the struct itself

  let (doc_params_to_strct, doc_fields) = try2!(extract_doc_fields_shift_up(strct.fields.iter_mut()));
  let maybe_empty_doc_par_to_fn: Vec<Attribute> = doc_params_to_strct.unwrap_or_else(|| vec![]);
  // let documented_generics     = try2!(extract_documented_generics_shift_up(&mut strct.generics));
  let has_doc_fields   = !doc_fields  .is_empty();
  // let has_documented_generics = !documented_generics.is_empty();

  // if !has_doc_fields && !has_documented_generics {
  if !has_doc_fields {return syn::Error::new_spanned(strct.ident,"Struct has no documented fields or generics. Document at least one.",).into_compile_error().into();}

  let parameter_doc_block = make_doc_block("Fields", doc_fields);
  // let generics_doc_block  = make_doc_block("Generics", documented_generics);

  let docs_before = struct_docs.before_args_section;
  let docs_after  = struct_docs.after_args_section;
  let maybe_empty_doc_line = if !docs_after.is_empty() {Some(quote! {#[doc=""]})
  } else                                               {None};

  quote! {#(#docs_before)* #(#maybe_empty_doc_par_to_fn)*
    #parameter_doc_block
  //     #generics_doc_block
    #maybe_empty_doc_line
    #(#docs_after)*
    #strct
  }.into()
}
