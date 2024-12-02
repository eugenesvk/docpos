use docpos::*;

#[roxygen]
/// here are some comments
/// and some more
#[parameters_section]
/// and some more
/// but this next argument section should not be here
#[parameters_section]
pub fn add(
    /// some comments
    first: i32,
    second: i32,
) -> i32 {
    first + second
}

pub fn main() {}
