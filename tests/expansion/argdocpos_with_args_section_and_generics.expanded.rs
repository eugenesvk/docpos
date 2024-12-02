use docpos::*;
/// this is documentation
/// and this is too
///
/// **Parameters**:
///
/// * `bar`: this goes after the arguments section
/// * `baz`: this has one line of docs
/// * `_undocumented`: this has
///    two lines of docs
///
/// **Generics**:
///
/// * `a`: a lifetime
/// * `T`: documentation for parameter T
///    spans multiple lines
/// * `N`: a const generic
fn foo<'a, S, T, const N: usize>(bar: u32, baz: String, _undocumented: i32) -> bool {
    baz.len() > bar as usize
}
