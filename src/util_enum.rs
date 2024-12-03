use crate::util::split_doc_in2;
use crate::util::{DocumentedIdent, extract_doc_attrs};
use crate::is_parameters_section;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, Generics, Ident, LitStr, Meta, MetaNameValue, Pat, Variant, Field, ExprLit, Lit};
use itertools::{Itertools, Position as IPos};


/// Extract the variant documentation from an iterator over enum variants, shifting the usual order by -1, returning the 1st variant's docs separately so that it can be used as a enum comment.
/// Will also remove all the doc comments from the collection of attributes, but will leave all the other attributes untouched.
/// Also allows splitting the last variant's docs into 2: belonging to the last variant (after ///!) and to the previous one
///
/// enum EnumyPos { /// this comment belongs to the enum, not to the next variant, so will be returned separately
///  image_data,/// this comment belongs to the preceding `image_data` variant, not `nrows`
///  nrows     ,/// this part of the comment belongs to `nrows`
///             ///! but this part — to the last `ncols` variant
///  ncols     ,// it's a syntax error to add doc comments at the end
/// }
pub fn extract_doc_variants_shift_up<'a,I>(args: I) -> Result<(Option<Vec<Attribute>>,Vec<DocumentedIdent<'a>>), syn::Error>
  where                                          I:Iterator<Item = &'a mut Variant>,{
  // will contain the docs comments for each documented enum variant together with the identifier of the enum variant
  let (lower, upper) = args.size_hint();
  let mut doc_fields = Vec::<DocumentedIdent>::with_capacity(upper.unwrap_or(lower));

  let mut docs0enum	:Option<Vec::<Attribute>>	= None;
  let mut id_prev  	:Option<     &Ident     >	= None;
  let mut id_last  	:Option<     &Ident     >	= None;
  let mut id_only  	:Option<     &Ident     >	= None;
  let mut docs_last	:       Vec::<Attribute> 	= vec![];
  for (pos,arg) in args.with_position() {
    let id = &arg.ident;
    let docs = extract_doc_attrs(&mut arg.attrs); // attrs:Attribute → meta:Meta::NameValue → value:Expr::Lit → lit:Lit::Str → token:" var→var1 doc"
    if !docs.is_empty() {
      match pos {
        IPos::Only   => {id_only = Some(id); docs_last = docs;break;},// can be ///! split; break to avoid wrong id_prev
        IPos::First  => {             docs0enum = Some(docs);     },// no ///! split needed, pre-field docs go to enum
        IPos::Middle => {doc_fields.push(DocumentedIdent::new(id_prev.take().expect("saved prev id"), docs));},
        IPos::Last   => {id_last = Some(id); docs_last = docs;break;},// can be ///! split; break to avoid wrong id_prev
        } // ↓ don't set on last item, break before
    }; id_prev = Some(id); // save id even without docs since next docs might need to be split-attached to it
  } // on ///! split the docs between 2 fields, removing !
  if        let Some(id_last) = id_last {let (doc2prev,doc2last) = split_doc_in2(docs_last);
    if ! doc2prev.is_empty () {doc_fields.push(DocumentedIdent::new(id_prev.expect("saved prev id"), doc2prev ));} // add last-1 variants docs
    if ! doc2last.is_empty () {doc_fields.push(DocumentedIdent::new(id_last                        , doc2last ));} // pos-///! → last variant
  } else if let Some(id_only) = id_only {let (doc2enum,doc2field) = split_doc_in2(docs_last);
    if ! doc2enum .is_empty() {                                                     docs0enum = Some(doc2enum ) ;}
    if ! doc2field.is_empty() {doc_fields.push(DocumentedIdent::new(id_only                        , doc2field));} // pos-///! →     variant
  }
  Ok((docs0enum,doc_fields))
}

/// Reorder the variants' docs (shifts up by 1), only extracting the first one so it can become part of the enum's doc
/// The last variant's docs can be split between last-1 and last based on ///!
pub fn doc_variants_shift_up<'a,I>(args: I) -> Result<Option<Vec<Attribute>>, syn::Error>
  where                                  I:Iterator<Item = &'a mut Variant>,{
  let mut docs0enum	:Option<Vec::<Attribute>>	= None;
  let mut arg_prev 	:Option<&mut Variant    >	= None;
  for (pos,arg) in args.with_position() {
    let mut docs = extract_doc_attrs(&mut arg.attrs); // attrs:Attribute → meta:Meta::NameValue → value:Expr::Lit → lit:Lit::Str → token:" var→var1 doc"
    if !docs.is_empty() {
      match pos { // on ///! split the docs between 2 fields, removing !
        IPos::Only   => {let (    doc2prev, mut doc2last) = split_doc_in2(docs);
          if ! doc2prev.is_empty() {docs0enum = Some(     doc2prev)}  // pre///! → enum
          if ! doc2last.is_empty() {arg.attrs.append(&mut doc2last)}},
        IPos::First  => {           docs0enum = Some(     docs    ) },// no ///! split needed, pre-1st-variant docs go to enum
        IPos::Middle => {           arg_prev.take().expect("saved prev arg").attrs.append(&mut docs    )}
        IPos::Last   => {let (mut doc2prev, mut doc2last) = split_doc_in2(docs); //split docs between 2 variants
          if ! doc2prev.is_empty() {arg_prev.take().expect("saved prev arg").attrs.append(&mut doc2prev)}
          if ! doc2last.is_empty() {arg                                     .attrs.append(&mut doc2last)}},
      }; // ↓ don't set on last item, break before
    }; arg_prev = Some(arg); // save arg even without docs since next docs might need to be (split)-attached to it
  }
  Ok(docs0enum)
}
