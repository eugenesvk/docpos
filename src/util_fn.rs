use crate::util::split_doc_in2;
use crate::util::{DocumentedIdent, extract_doc_attrs};
use crate::is_parameters_section;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, FnArg, Generics, Ident, LitStr, Meta, MetaNameValue, Pat, Variant, Field, ExprLit, Lit};
use itertools::{Itertools, Position as IPos};

/// Same as extract_documented_parameters, but shifts all docs by -1, returning the 1st parameter's docs separately,
/// so that it can be used as a function comment
/// Also allows splitting the last parameter's docs into 2: belonging to the last parameter (after ///!) and to the previous one
///
/// fn sum_image_rows( /// this comment belongs to the function, not to the next parameter, so will be returned separately
///  image_data : &[f32],/// this comment belongs to the preceding `image_data` parameter, not `nrows`
///  nrows      :   u32 ,/// this part of the comment belongs to `nrows`
///                             ///! but this part — to the last `ncols` parameter
///  ncols      :   u32 ,// it's a syntax error to add doc comments at the end
/// )
pub fn extract_documented_parameters_shift_up<'a,I>(args: I) -> Result<(Option<Vec<Attribute>>,Vec<DocumentedIdent<'a>>), syn::Error>
  where                                        I:Iterator<Item = &'a mut FnArg>,{
  // will contain the docs comments for each documented function parameter together with the identifier of the function parameter
  let (lower, upper) = args.size_hint();
  let mut documented_params = Vec::<DocumentedIdent>::with_capacity(upper.unwrap_or(lower));

  let mut docs0fn   :Option<Vec::<Attribute>> = None;
  let mut ident_prev:Option<     &Ident     > = None;
  let mut ident_last:Option<     &Ident     > = None;
  let mut ident_only:Option<     &Ident     > = None;
  let mut docs_last :       Vec::<Attribute> = vec![];
  for (pos,arg) in args.with_position() {
    match arg {
      FnArg::Typed(pat_type) => {
        let Pat::Ident(pat_ident) = pat_type.pat.as_ref() else {unreachable!("unexpected node while parsing");};
        let ident = &pat_ident.ident;
        let docs = extract_doc_attrs(&mut pat_type.attrs);
        if !docs.is_empty() {
          match pos {
            IPos::Only   => {ident_only = Some(ident); docs_last =      docs;break;},// break to avoid wrong ident_prev
            IPos::First  => {                          docs0fn   = Some(docs);     },// no ///! split needed, pre-par docs go to fn
            IPos::Middle => {documented_params.push(DocumentedIdent::new(ident_prev.take().expect("saved prev ident"), docs));},
            IPos::Last   => {documented_params.push(DocumentedIdent::new(ident_prev.take().expect("saved prev ident"), docs.clone()));
                     ident_last = Some(ident); docs_last =      docs;break;},
          } // ↓ don't set on last item, break before
        }; ident_prev = Some(ident); // save id even without docs since next docs might need to be split-attached to it
      }
      FnArg::Receiver(_) => {}
    }
  }
  if        let Some(ident_last) = ident_last { // on ///! split the docs between 2 parameters, removing !
    let (docs2prev,docs2last) = split_doc_in2(docs_last);
    if ! docs2last.is_empty() {
      if let Some(mut docum_par_prev) = documented_params.pop() { // replace last-1 item's docs with its pre-///! docs
        docum_par_prev.docs = docs2prev;
        documented_params.push(docum_par_prev);
        documented_params.push(DocumentedIdent::new(ident_last, docs2last));
      } else {                                                    // add     last-i item's docs …
        documented_params.push(DocumentedIdent::new(ident_prev.expect("saved prev ident"), docs2prev));
        documented_params.push(DocumentedIdent::new(ident_last, docs2last));
      }
    }
  } else if let Some(ident_only) = ident_only { // on ///! split the docs between fn and parameter, removing !
    let (docs2fn,docs2par) = split_doc_in2(docs_last);
    if ! docs2fn .is_empty() {                docs0fn = Some(docs2fn);}
    if ! docs2par.is_empty() {
        documented_params.push(DocumentedIdent::new(ident_only, docs2par));}
  }
  Ok((docs0fn,documented_params))
}


/// Same as extract_documented_generics, but shifts all docs by -1
/// Also allows splitting the last generic's doc into 2: belonging to the last generic (after ///!) and to the previous one
/// fn with_lifetimes_pos <
///   'a    ,/// a lifetime
///   S     ,
///   T     ,/// doc for T line 1
///          /// doc for T line 2
///          ///! a const generic
///   const N: usize, // it's a syntax error to add doc comments at the end
/// >(){}
pub fn extract_documented_generics_shift_up(generics: &'_ mut Generics,) -> Result<Vec<DocumentedIdent<'_>>, syn::Error> {
  let mut doc_gen = Vec::with_capacity(generics.params.len());

  let mut id_prev  :Option<     &Ident     > = None;
  let mut id_last  :Option<     &Ident     > = None;
  let mut id_only  :Option<     &Ident     > = None;
  let mut docs_last:       Vec::<Attribute> = vec![];
  let mut i   :usize = 0; // track the iter position for args with non-empty docs to allow merging pre doc→1st and 2nd gen→1st
  for (pos,param) in generics.params.iter_mut().with_position() {
    i += 1;
    let (id, attrs) = match param {
      syn::GenericParam::Lifetime(lif) => (&lif.lifetime.ident, &mut lif.attrs), // id=a attrs=..."doc" ...lit="a lifetime"
      syn::GenericParam::Type    (ty ) => (&ty          .ident, &mut ty .attrs), // id=T
      syn::GenericParam::Const   (con) => (&con         .ident, &mut con.attrs), // id=N
    };
    let mut docs = extract_doc_attrs(attrs);
    if !docs.is_empty() {
      match pos {
        IPos::Only   => {doc_gen.push(DocumentedIdent::new(id, docs));},
        IPos::First  => {doc_gen.push(DocumentedIdent::new(id, docs));}, // pre-gen docs go to the 1st par as in prefixed docs
        IPos::Middle => {let id_prev = id_prev.take().expect("saved prev id");
          if i==2 {if let Some(mut doc_gen1) = doc_gen.pop() {
          doc_gen1.docs.append(&mut docs); doc_gen.push(doc_gen1                           )  ;  // gen2 docs append to  gen1
          } else {                         doc_gen.push(DocumentedIdent::new(id_prev, docs));};  //   … or    become new gen1
          } else {                         doc_gen.push(DocumentedIdent::new(id_prev, docs));};
          },// gen3+ as usual
        IPos::Last   => {id_last = Some(id); docs_last = docs;break;},
      }; // ↓ don't set on last item, break before
    }; id_prev = Some(id); // save id even without docs since next docs might need to be split-attached to it
  }
  if let Some(id_last) = id_last { // on ///! split the docs between 2 generics, removing !
    let (docs2prev,docs2last) = split_doc_in2(docs_last);
    if ! docs2prev.is_empty() {doc_gen.push(DocumentedIdent::new(id_prev.expect("saved prev ident"), docs2prev))};
    if ! docs2last.is_empty() {doc_gen.push(DocumentedIdent::new(id_last                           , docs2last))};
  }
  Ok(doc_gen)
}
