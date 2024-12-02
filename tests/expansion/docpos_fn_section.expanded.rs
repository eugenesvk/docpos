use docpos::*;
/// fnOuter line 1
/// fnOuter line 2
/// fnInnter after the arguments section
///
/// **Parameters**:
///
/// * `p1`: p1→p1 line 1
/// * `p2`: p2→p2 line 1
///    p2→p2 line 2
fn foo(p1: u32, p2: String, _undocumented: i32) -> bool {
    p2.len() > p1 as usize
}
/// fnOuter line 1
/// fnOuter line 2
/// fnInnter after the arguments section
///
/// **Parameters**:
///
/// * `p1`: p1→p1 line 1
/// * `p2`: p2→p2 line 1
///    p2→p2 line 2
fn foo2(p1: u32, p2: String, _undocumented: i32) -> bool {
    p2.len() > p1 as usize
}
