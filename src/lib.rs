#![cfg_attr(not(debug_assertions),allow(non_snake_case,non_upper_case_globals,non_camel_case_types))]
#![cfg_attr(    debug_assertions ,allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros))]
#![doc= include_str!("../Readme.md")]
//! ## Documenting Generics
//! Generic parameters can be documented with doc comments just as the arguments
//! can be:
//! ```rust
//! use roxygen::roxygen;
//!
//! #[roxygen]
//! fn frobnicate<
//! /// some comment goes here
//! S,
//! T> (
//! /// the value being frobnicated
//! frobnicator: T,
//! /// the frobnicant
//! frobnicant: S) -> T
//! {
//!    todo!()
//! }
//! ```
//!
//! This generates an additional section for the generic parameters right
//! after the arguments section (if it exists).
//! All types of generic arguments, including lifetimes and const-generics
//! can be documented like this.
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Attribute, ItemFn, ItemStruct};
use util::{
    extract_documented_generics, extract_documented_parameters,
    extract_documented_generics_shift_up, extract_documented_parameters_shift_up, extract_fn_doc_attrs, make_doc_block,
    extract_struct_doc_attrs,
};
use util_strct::extract_doc_fields_shift_up;
mod util;
mod util_strct;

/// parameter section macro name
const PARAM_SECTION: &str = "parameters_section";
/// the name of this crate
const ROXYGEN_CRATE: &str = "roxygen";
/// the name of the main macro in this crate
const ROXYGEN_MACRO: &str = ROXYGEN_CRATE;

// helper macro "try" on a syn::Error, so that we can return it as a token stream
macro_rules! try2 {
    ($ex:expr) => {
        match $ex {
            Ok(val) => val,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

#[proc_macro_attribute]
/// the principal attribute inside this crate that lets us document function arguments
pub fn roxygen(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    try2!(function.attrs.iter_mut().try_for_each(|attr| {
        if is_roxygen_main(attr) {
            Err(syn::Error::new_spanned(
                attr,
                "Duplicate attribute. This attribute must only appear once.",
            ))
        } else {
            Ok(())
        }
    }));

    // extrac the doc attributes on the function itself
    let function_docs = try2!(extract_fn_doc_attrs(&mut function.attrs));

    let documented_params = try2!(extract_documented_parameters(
        function.sig.inputs.iter_mut()
    ));

    let documented_generics = try2!(extract_documented_generics(&mut function.sig.generics));

    let has_documented_params = !documented_params.is_empty();
    let has_documented_generics = !documented_generics.is_empty();

    if !has_documented_params && !has_documented_generics {
        return syn::Error::new_spanned(
            function.sig.ident,
            "Function has no documented parameters or generics.\nDocument at least one function parameter or generic.",
        )
        .into_compile_error()
        .into();
    }

    let parameter_doc_block = make_doc_block("Parameters", documented_params);
    let generics_doc_block = make_doc_block("Generics", documented_generics);

    let docs_before = function_docs.before_args_section;
    let docs_after = function_docs.after_args_section;
    let maybe_empty_doc_line = if !docs_after.is_empty() {
        Some(quote! {#[doc=""]})
    } else {
        None
    };

    quote! {
        #(#docs_before)*
        #parameter_doc_block
        #generics_doc_block
        #maybe_empty_doc_line
        #(#docs_after)*
        #function
    }
    .into()
}

#[proc_macro_attribute]
/// the principal attribute inside this crate that lets us document function arguments, but after them, not before
pub fn argdocpos(_attr: proc_macro::TokenStream
    ,            item : proc_macro::TokenStream,
    )                -> proc_macro::TokenStream {
    let mut function: ItemFn = parse_macro_input!(item as ItemFn);

    try2!(function.attrs.iter_mut().try_for_each(|attr| {
        if is_argdocpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
        } else                     {Ok(())}}));

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

#[proc_macro_attribute]
/// the principal attribute inside this crate that lets us document struct arguments, but after them, not before
pub fn structdocpos(_attr: proc_macro::TokenStream
    ,               item : proc_macro::TokenStream,
    )                -> proc_macro::TokenStream {
    let mut strct: ItemStruct = parse_macro_input!(item as ItemStruct);

    try2!(strct.attrs.iter_mut().try_for_each(|attr| {
        if is_structdocpos_main(attr) {Err(syn::Error::new_spanned(attr,"Duplicate attribute. This attribute must only appear once.",))
        } else                        {Ok(())}}));

    let strctn_docs = try2!(extract_struct_doc_attrs(&mut strct.attrs)); // extrac the doc attributes on the struct itself

    let (doc_params_to_strct, doc_fields) = try2!(extract_doc_fields_shift_up(strct.fields.iter_mut()));
    let maybe_empty_doc_par_to_fn: Vec<Attribute> = doc_params_to_strct.unwrap_or_else(|| vec![]);
    // let documented_generics     = try2!(extract_documented_generics_shift_up(&mut strct.generics));
    let has_doc_fields   = !doc_fields  .is_empty();
    // let has_documented_generics = !documented_generics.is_empty();

    // if !has_doc_fields && !has_documented_generics {
    if !has_doc_fields {return syn::Error::new_spanned(strct.ident,"Struct has no documented fields or generics. Document at least one.",).into_compile_error().into();}

    let parameter_doc_block = make_doc_block("Fields", doc_fields);
    // let generics_doc_block  = make_doc_block("Generics", documented_generics);

    let docs_before = strctn_docs.before_args_section;
    let docs_after  = strctn_docs.after_args_section;
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

// this is to expose the helper attribute #[arguments_section].
// The only logic about this attribute that this here function includes is
// to make sure that this attribute is not placed before the #[roxygen]
// attribute. All other logic is handled in the roxygen macro itself.
/// a helper attribute that dictates the placement of the section documenting
/// the function arguments
#[proc_macro_attribute]
pub fn parameters_section(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function: ItemFn = parse_macro_input!(item as ItemFn);

    // enforce that this macro comes after roxygen, which means it
    // cannot see the roxygen attribute
    let maybe_roxygen = function.attrs.iter().find(|attr| is_roxygen_main(attr));
    if let Some(attr) = maybe_roxygen {
        syn::Error::new_spanned(attr,"The #[roxygen] attribute must come before the parameters_section attribute.\nPlace it before any of the doc comments for the function.").into_compile_error().into()
    } else {
        function.to_token_stream().into()
    }
}

/// check whether an attribute is the arguments section attribute.
/// Stick this into it's own function so I can change the logic
//@note(geo) this logic won't work if the crate is renamed
#[inline(always)]
fn is_parameters_section(attr: &Attribute) -> bool {
    let path = attr.path();

    if path.is_ident(PARAM_SECTION) {
        true
    } else {
        // checks for (::)roxygen::param_section
        path.segments.len() == 2
            && path.segments[0].ident == ROXYGEN_CRATE
            && path.segments[1].ident == PARAM_SECTION
    }
}

/// check whether an attribute is the raw #[roxygen] main attribute.
/// Stuck into this function, so I can refactor this logic
//@note(geo) this logic won't work if the crate is renamed
#[inline(always)]
fn is_roxygen_main(attr: &Attribute) -> bool {
    let path = attr.path();

    if path.is_ident(ROXYGEN_MACRO) {
        true
    } else {
        // checks for (::)roxygen::roxygen
        path.segments.len() == 2
            && path.segments[0].ident == ROXYGEN_CRATE
            && path.segments[1].ident == ROXYGEN_MACRO
    }
}

/// check whether an attribute is the raw #[argdocpos] main attribute.
#[inline(always)]fn is_argdocpos_main   (attr: &Attribute) -> bool {attr.path().is_ident("argdocpos"   )}
#[inline(always)]fn is_structdocpos_main(attr: &Attribute) -> bool {attr.path().is_ident("structdocpos")}
