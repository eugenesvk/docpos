use docpos::roxygen;

#[roxygen]
/// long documention
/// ...
#[docpos::parameters_section]
/// # Examples
/// ...
fn foo(
    /// some docs
    first: i32,
    second: f32,
) {
}
