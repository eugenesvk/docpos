use roxygen::*;

#[docpos("enum")]
/// enumPre line 1
/// enumPre line 2
pub enum EnumyPos { /// enumPos line1
  /// enumPos line2
  V1, /// v1→v1 line 1
      /// v1→v1 line 2
      ///! v1→v2 line 1
      /// v1→v2 line 2
  V2, //
}

#[docpos]
/// enumPre line 1
/// enumPre line 2
pub enum EnumyPos2 { /// enumPos line1
  /// enumPos line2
  V1, /// v1→v1 line 1
      /// v1→v1 line 2
      ///! v1→v2 line 1
      /// v1→v2 line 2
  V2, //
}
