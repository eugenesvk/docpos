use docpos::*;

#[docpos]
/// fnOuter line 1
/// fnOuter line 2
#[parameters_section]
fn foo( /// fnInnter after the arguments section
  p1           	: u32   	,/// p1→p1 line 1
  p2           	: String	,/// p2→p2 line 1
               	        	 /// p2→p2 line 2
  _undocumented	: i32   	,
) -> bool {p2.len() > p1 as usize}

#[docpos(fn)]
/// fnOuter line 1
/// fnOuter line 2
#[parameters_section]
fn foo2( /// fnInnter after the arguments section
  p1           	: u32   	,/// p1→p1 line 1
  p2           	: String	,/// p2→p2 line 1
               	        	 /// p2→p2 line 2
  _undocumented	: i32   	,
) -> bool {p2.len() > p1 as usize}
