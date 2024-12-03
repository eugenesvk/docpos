// helper docs to illustrate various deep-nested syn elements in a syntax-highlighted way
pub fn extract_struct_doc_attrs(attrs: &mut Vec<Attribute>) -> Result<StructDocs, syn::Error> {
  Attribute {
    pound_token      	: Pound,
    style            	: AttrStyle::Outer,
    bracket_token    	: Bracket,
    meta             	: Meta::NameValue{
      path           	: Path {
        leading_colon	: None,
        segments     	: [PathSegment{
          ident      	: Ident {ident:"doc",span:#0bytes(0..1)},
          arguments  	: PathArguments::None}]},
      eq_token       	: Eq,
      value          	: Expr::Lit{
        attrs        	: [],
        lit          	: Lit::Str{token:"preMyEnum"}
      }
    }
  }
}

