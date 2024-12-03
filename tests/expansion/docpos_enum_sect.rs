use docpos::*;

#[docpos(enum_sect)]
enum EnumyPos { /// enumPos line1
                /// enumPos line2
  V1	,/// v1→v1 line 1
    	 /// v1→v1 line 2
  V2	,/// v2→v2 line 1
  V3	,// won't be shown
  V4	,// won't be shown
    	 ///! v4→v5 line 1
  V5	,//
}
