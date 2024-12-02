use crate::util::split_doc_in2;
use crate::util::{DocumentedIdent, extract_doc_attrs};
use crate::is_parameters_section;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, Generics, Ident, LitStr, Meta, MetaNameValue, Pat, Fields, Field, ExprLit, Lit};
use itertools::{Itertools, Position as IPos};


/// Extract the field documentation from an iterator over struct fields, shifting the usual order by -1, returning the 1st field's docs separately so that it can be used as a struct comment.
/// Will also remove all the doc comments from the collection of attributes, but will leave all the other attributes untouched.
/// Also allows splitting the last field's docs into 2: belonging to the last field (after ///!) and to the previous one
///
/// struct StructyPos { /// this comment belongs to the struct, not to the next field, so will be returned separately
///  image_data : &[f32],/// this comment belongs to the preceding `image_data` field, not `nrows`
///  nrows      :   u32 ,/// this part of the comment belongs to `nrows`
///                             ///! but this part — to the last `ncols` field
///  ncols      :   u32 ,// it's a syntax error to add doc comments at the end
/// }
pub fn extract_doc_fields_shift_up<'a,I>(args: I) -> Result<(Option<Vec<Attribute>>,Vec<DocumentedIdent<'a>>), syn::Error>
  where                                        I:Iterator<Item = &'a mut Field>,{
  // will contain the docs comments for each documented struct field together with the identifier of the struct field
  let (lower, upper) = args.size_hint();
  let mut doc_fields = Vec::<DocumentedIdent>::with_capacity(upper.unwrap_or(lower));

  let mut doc0struct:Option<Vec::<Attribute>> = None;
  let mut id_prev   :Option<     &Ident     > = None;
  let mut id_last   :Option<     &Ident     > = None;
  let mut id_only   :Option<     &Ident     > = None;
  let mut docs_last :       Vec::<Attribute> = vec![];
  for (pos,arg) in args.with_position() {
    if let Some(id) = &arg.ident { // ident:Some(Ident {ident: "f2",..}) some structs have no name
      let docs = extract_doc_attrs(&mut arg.attrs); // attrs:Attribute → meta:Meta::NameValue → value:Expr::Lit → lit:Lit::Str → token:" f1→f1 doc"
      // println!("arg.ty={:#?}",arg);
      if !docs.is_empty() {
        match pos {
          IPos::Only   => {id_only = Some(id); docs_last = docs;break;},// can be ///! split; break to avoid wrong id_prev
          IPos::First  => {             doc0struct = Some(docs);     },// no ///! split needed, pre-field docs go to struct
          IPos::Middle => {doc_fields.push(DocumentedIdent::new(id_prev.take().expect("saved prev id"), docs));},
          IPos::Last   => {id_last = Some(id); docs_last = docs;break;},// can be ///! split; break to avoid wrong id_prev
          } // ↓ don't set on last item, break before
      }; id_prev = Some(id); // save id even without docs since next docs might need to be split-attached to it
    }
  }
  if        let Some(id_last) = id_last { // on ///! split the docs between 2 fields, removing !
    let (docs2prev,docs2last) = split_doc_in2(docs_last);
    if ! docs2last.is_empty() {
      if let Some(mut docum_field_prev) = doc_fields.pop() { // replace last-1 item's docs with its pre-///! docs
        docum_field_prev.docs = docs2prev;
        doc_fields.push(docum_field_prev);
        doc_fields.push(DocumentedIdent::new(id_last, docs2last));
      } else {                                               // add     last-i item's docs …
        doc_fields.push(DocumentedIdent::new(id_prev.expect("saved prev ident"), docs2prev));
        doc_fields.push(DocumentedIdent::new(id_last, docs2last));
      }
    }
  } else if let Some(id_only) = id_only { // on ///! split the docs between struct and field, removing !
    let (docs2struct,docs2field) = split_doc_in2(docs_last);
    if ! docs2struct.is_empty() {                docs0struct = Some(docs2struct);}
    if ! docs2field .is_empty() {
        doc_fields.push(DocumentedIdent::new(id_only, docs2field));}
  }
  Ok((docs0struct,doc_fields))
}
