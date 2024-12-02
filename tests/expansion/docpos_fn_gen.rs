use docpos::*;

#[docpos]
/// fnPre line 1.
/// fnPre line 2.
#[parameters_section]
/// fnPre line 3 after the parameters/generics section
fn with_lifetimes</// pre-a-lifetime (preserved)
  'a	,/// pos-a-lifetime
  S 	,/// documentation for parameter S
    	 /// spans multiple lines
  T 	,/// T self
       ///! T→N const generic
  const N: usize,>(/// fnInner
  par1: u32,
  par2: String, /// par2 (par1 is not documented)
  _undocumented: i32,
) -> bool {par2.len() > par1 as usize}
