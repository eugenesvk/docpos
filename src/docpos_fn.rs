use quote::quote;
use syn::{Attribute, ItemFn};
use crate::util::{extract_fn_doc_attrs, make_doc_block};
use crate::util_fn::{extract_documented_generics_shift_up, extract_documented_parameters_shift_up,};
use crate::try2 as try2;
use crate::helper::*;


/// Document function arguments, but after them, not before
pub fn docpos_fn(mut function:ItemFn) -> proc_macro::TokenStream {
  try2!(function.attrs.iter_mut().try_for_each(|attr| {
    if is_docpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
    } else                  {Ok(())}}));

  let function_docs = try2!(extract_fn_doc_attrs(&mut function.attrs)); // extrac the doc attributes on the function itself

  let (doc_params_to_fn, documented_params) = try2!(extract_documented_parameters_shift_up(function.sig.inputs.iter_mut()));
  let maybe_empty_doc_par_to_fn: Vec<Attribute> = doc_params_to_fn.unwrap_or_else(|| vec![]);
  let documented_generics     = try2!(extract_documented_generics_shift_up(&mut function.sig.generics));
  let has_documented_params   = !documented_params  .is_empty();
  let has_documented_generics = !documented_generics.is_empty();

  if !has_documented_params && !has_documented_generics {
    return syn::Error::new_spanned(function.sig.ident,"Function has no documented parameters or generics.\nDocument at least one function parameter or generic.",)
    .into_compile_error().into();}

  let parameter_doc_block = make_doc_block("Parameters", documented_params);
  let generics_doc_block  = make_doc_block("Generics", documented_generics);

  let docs_before = function_docs.before_args_section;
  let docs_after  = function_docs.after_args_section;
  let maybe_empty_doc_line = if !docs_after.is_empty() {Some(quote! {#[doc=""]})
  } else                                               {None};

  quote! {#(#docs_before)* #(#maybe_empty_doc_par_to_fn)*
    #parameter_doc_block
    #generics_doc_block
    #maybe_empty_doc_line
    #(#docs_after)*
    #function
  }.into()
}
