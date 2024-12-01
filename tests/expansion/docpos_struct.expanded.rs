use roxygen::*;
/// structPre line 1.
/// structPre line 2.
/// scructInner
///
/// **Fields**:
/// * `x`: doc @ x for x
/// * `y`: doc @ x for y (after `///!`)
///
#[structdocpos]
pub struct StructyPos {pub x:i8, pub y:i8,}
