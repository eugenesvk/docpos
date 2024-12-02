use roxygen::*;

#[docpos("fn")]
fn foo( /// fnInner line 1
  /// fnInner line 2
  // regular comment
  p1	: u32   	, /// p1→p2 line 1
  p2	: String	, /// p2→p2 line 2
  /// p2→p2 line 2
  _undocumented: i32,
) -> bool {p2.len() > p1 as usize}

#[docpos]
fn foo2( /// fnInner line 1
  /// fnInner line 2
  // regular comment
  p1	: u32   	, /// p1→p2 line 1
  p2	: String	, /// p2→p2 line 2
  /// p2→p2 line 2
  _undocumented: i32,
) -> bool {p2.len() > p1 as usize}
