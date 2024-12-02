use docpos::*;

#[docpos(struct)]
/// structPre line 1.
/// structPre line 2.
pub struct StructyPos { /// scructInner
  pub x	: i8	,/// doc @ x for x
       	    	 ///! doc @ x for y (after `///!`)
  pub y	: i8	,
}

#[docpos]
/// structPre line 1.
/// structPre line 2.
pub struct StructyPos2 { /// scructInner
  pub x	: i8	,/// doc @ x for x
       	    	 ///! doc @ x for y (after `///!`)
  pub y	: i8	,
}
