use roxygen::*;

#[docpos("fn")]
/// fn_Pre
pub fn tst( /// fn_Pos
  par1: f32, /// par1→par1
  par2: f32,
) {}

#[docpos]
/// fn_Pre
pub fn tst2( /// fn_Pos
  par1: f32, /// par1→par1
  par2: f32,
) {}
