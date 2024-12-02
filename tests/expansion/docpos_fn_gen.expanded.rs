use docpos::*;
/// fnPre line 1.
/// fnPre line 2.
/// fnInner
///
/// **Parameters**:
///
/// * `par2`: par2 (par1 is not documented)
///
/// **Generics**:
///
/// * `a`: pre-a-lifetime (preserved)
///    pos-a-lifetime
/// * `S`: documentation for parameter S
///    spans multiple lines
/// * `T`: T self
/// * `N`: T→N const generic
///
/// fnPre line 3 after the parameters/generics section
fn with_lifetimes<'a, S, T, const N: usize>(
    par1: u32,
    par2: String,
    _undocumented: i32,
) -> bool {
    par2.len() > par1 as usize
}