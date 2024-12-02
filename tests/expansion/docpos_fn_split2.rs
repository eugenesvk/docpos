use docpos::*;

#[docpos]
/// fn_Pre
pub fn tst( // / missing pos shouldn't break par
  par1: f32, /// par1→par1
             ///! par1→par2
  par2: f32,
) {}

#[docpos(fn)]
/// fn_Pre
pub fn tst2( // / missing pos shouldn't break par
  par1: f32, /// par1→par1
             ///! par1→par2
  par2: f32,
) {}
