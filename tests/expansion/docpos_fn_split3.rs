use docpos::*;

#[docpos]
/// fn_Pre
pub fn tst( /// fn_Pos
  ///!          fn→par1
  par1: f32,
) {}

#[docpos(fn)]
/// fn_Pre
pub fn tst2( /// fn_Pos
  ///!          fn→par1
  par1: f32,
) {}
