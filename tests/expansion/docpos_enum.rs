use docpos::*;

#[docpos("enum")]
pub enum EnumyPos { /// enumPos line1
                    /// enumPos line2
  pub V1	,/// v1→v1 line 1
        	 /// v1→v1 line 2
  pub V2	,/// v2→v2 line 1
  pub V3	,// won't be shown
  pub V4	,// won't be shown
        	 ///! v4→v5 line 1
  pub V5	,//
}


#[docpos]
pub enum EnumyPos2 { /// enumPos line1
                    /// enumPos line2
  pub V1	,/// v1→v1 line 1
        	 /// v1→v1 line 2
  pub V2	,/// v2→v2 line 1
  pub V3	,// won't be shown
  pub V4	,// won't be shown
        	 ///! v4→v5 line 1
  pub V5	,//
}