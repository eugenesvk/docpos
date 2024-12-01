use roxygen::*;

#[structdocpos]
/// structPre line 1.
/// structPre line 2.
pub struct StructyPos { /// scructInner
  pub x	: i8	,/// doc @ x for x
       	    	 ///! doc @ x for y (after `///!`)
  pub y	: i8	,
}
