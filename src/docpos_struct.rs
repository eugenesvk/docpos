use crate::*;
use crate::util::make_doc_block;
use crate::try2 as try2;
use crate::util_struct::{extract_doc_fields_shift_up,doc_fields_shift_up};
use quote::quote;
use syn::{Attribute, ItemStruct};


/// Document Struct field, but after them, not before. With 'extract' removes the docs from fields to its own smaller section (though the default rustdocs still adds an empty duplicate section)
pub fn docpos_struct(mut strct:ItemStruct, extract:bool) -> proc_macro::TokenStream {
  try2!(strct.attrs.iter_mut().try_for_each(|attr| {
    if is_docpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
    } else                  {Ok(())}}));

  let struct_docs = try2!(extract_struct_doc_attrs(&mut strct.attrs)); // extrac the doc attributes on the struct itself

  let (doc_par2struct, doc_fields) = if extract	{ try2!(extract_doc_fields_shift_up(strct.fields.iter_mut()))
  } else                                       	{(try2!(        doc_fields_shift_up(strct.fields.iter_mut())), vec![])};
  let maybe_empty_doc_par_to_struct: Vec<Attribute> = doc_par2struct.unwrap_or_else(|| vec![]);
  // let doc_generics     = try2!(extract_documented_generics_shift_up(&mut strct.generics));
  let has_doc_fields = ! extract || ! doc_fields  .is_empty();
  // let has_documented_generics =     ! doc_generics.is_empty();

  if !has_doc_fields /*&& !has_documented_generics*/ {
    return syn::Error::new_spanned(strct.ident,"Struct has no documented fields or generics. Document at least one.",).into_compile_error().into();}

  let field_doc_block = if extract{make_doc_block("Fields", doc_fields)} else {None};
  // let generics_doc_block  = make_doc_block("Generics", doc_generics);

  let docs_before = struct_docs.before_args_section;
  let docs_after  = struct_docs.after_args_section;
  let maybe_empty_doc_line = if !docs_after.is_empty() {Some(quote! {#[doc=""]})
  } else                                               {None};

  quote! {#(#docs_before)* #(#maybe_empty_doc_par_to_struct)*
    #field_doc_block
  //     #generics_doc_block
    #maybe_empty_doc_line
    #(#docs_after)*
    #strct
  }.into()
}
