# docpos - compact-documenting items after defining them: functions, structs, enums…

(a fork of the "passively-maintained" [roxygen](https://github.com/geo-ant/roxygen) with a few more supported items and allowing for a pos-position)

![build](https://github.com/eugenesvk/docpos/actions/workflows/build.yml/badge.svg?branch=main)
![tests](https://github.com/eugenesvk/docpos/actions/workflows/tests.yml/badge.svg?branch=main)
[![crates](https://img.shields.io/crates/v/docpos)](https://crates.io/crates/docpos)
![maintenance-status](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

The `#[docpos]` attribute allows you to add doc-comments after an item, not before (as in regular Rust) in enums and structs, and also document function parameters (a _compile error_ in current Rust). Generic lifetimes, types, and constants of the function [can also be documented](https://docs.rs/roxygen/latest/roxygen/). 

You can now write a much more readable and compact table:
```rust
use std::path::PathBuf;
use docpos::*;

#[docpos] pub struct StructyPos { /// "inner" scruct docs
  pub field1       	:        String  	,/// pos-doc for `field1` (in regular Rust this would be a doc for `field2_longer`)
  pub field2_longer	: Option<String> 	,/// pos-doc for `field2_longer`
                   	                 	 /// pos-doc for `field2_longer` line 2
                   	                 	 ///! pre-doc for `paths` at `field2_longer` (after `///!`)
  pub paths        	: Vec   <PathBuf>	, // no doc comments allowed here, use `///!` in the previous field
}
```
instead of 
```rust
use std::path::PathBuf;

/// "Outer" scruct docs wasting a line
pub struct StructyPre {
  /// pre-doc for `field1`, another line wasted on a short comment
  pub field1: String	,
  /// pre-doc for `field2_longer`, breaking the vertical flow of fields
  /// pre-doc for `field2_longer`, making the disconnect bigger
  pub field2_longer: Option<String>,
  /// pre-doc for `paths`
  pub paths: Vec<PathBuf>,
  // oh, and it's not vertically aligned anymore since you can't use an elastic-tabstoppy plugin to help with that
}
```

Macro can be used with an explicit argument `#[docpos("struct")]` or let the macro try each supported type via `#[docpos]`, though the latter will generate errors for each type.

Similarly, for a function, you can add doc-comments to parameters:

```rust
use docpos::*;

#[argdocpos]
fn sum_image_rows_pos( /// sum the rows of an image
  image_data: &    [f32], /// the image data in row-major format
  nrows     :       u32 , /// the number of rows in the image
  ncols     :       u32 , /// the number of columns in the image
  // everything after ///! will become part of the last parameter's comments!
                          ///! an out buffer into which the resulting
                          /// sums are placed. Must have space 
                          /// for exactly `nrows` elements
  sums      : &mut [f32], // doc comments are illegal here, so use "///!"-split comment syntax from ↑
) -> Result<(),String> {
    todo!()
} 
```

(Or use the main roxygen's crate macro `roxygen` for reguler pre-doc-comment support)

You have to document at least one parameter (or generic), but you don't have
to document all of them. The example above will produce documentation as 
if you had written a doc comment for the function like so:

```rust
/// sum the rows of an image
///
/// **Parameters**: 
///
/// * `image_data`: the image data in row-major format
/// * `nrows`: the number of rows in the image
/// * `ncols`: the number of columns in the image
/// * `sums`: an out buffer into which the resulting
///    sums are placed. Must have space 
///    for exactly `nrows` elements
fn sum_image_rows(
  image_data: &[f32],
  nrows: u32,
  ncols: u32,
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

⚠️  **Renaming** the macros exported from this crate (`use ... as ...`) or renaming the
crate itself (in your `Cargo.toml`) will make all of this stop working properly.

## Placing the Parameters-Section

By default, the section documenting the parameters will go at the end
of the top-level function documentation. However, this crate allows to explicitly
place the section by using a custom attribute like so:

```rust
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
  second: f32
  )
{}
```

## Considerations

It's a [long standing issue](https://github.com/rust-lang/rust/issues/57525)
whether and how to add this capability to `rustdoc`. Firstly, there's no
general consensus on how exactly to document function parameters. However, 
I've seen the presented style used a lot, with minor variations.
Secondly, the standard library [doesn't need this](https://github.com/rust-lang/rust/issues/57525#issuecomment-453633783)
style of documentation at all. So before you stick this macro on every function,
do consider

* taking inspiration from how the standard library deals with function parameters,
* using fewer function parameters,
* using more descriptive parameters names,
* using _types_ to communicate intent,
* sticking function parameters in a `struct`.

Here is [an elegant way](https://www.reddit.com/r/rust/comments/1gb782e/comment/ltpk16x/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button),
how the example above can be reworked without using per parameter documentation:

```rust
/// Sums the rows of an image.
///
/// The rows of `image_data`, an `nrows` by `ncols`
/// matrix in row-major ordering, are summed into `sums`
/// which must have exactly `nrows` elements.
fn sum_image_rows(
  image_data: &[f32],
  nrows: u32,
  ncols: u32,
  sums: &mut [f32]) -> Result<(),String> {
    todo!()
}
```

All that being said, I've realized that sometimes I still want to document
function parameters.

### Compile Times

Macros will always increase your compile time to some degree, but I don't think
this is a giant issue (after the docpos dependency itself was compiled, that is):
firstly, this macro is to be used _sparingly_. Secondly, this macro just does 
some light parsing and shuffling around of the documentation tokens. It 
introduces no additional code. Thus, it doesn't
make your actual code more or less complex and should not affect compile
times much (after this crate was compiled once), but I haven't
measured it... so take it with a grain of sodium-chloride.

### Known issues
  - waste of space in the output: for structs and enums, the default doc sections persists even though it has no comments
  - unrealized potential to waste less space in the output: short comments could've been "inlined" to the function signature as regular comments instead of having the whole new section
  - rather limited support: functions (parameters and generics), structs (fields), enums (variants)
  - invalid enum variant's visibility qualifiers `enum Vis {pub V1}` are not rejected due to [syn design decision](https://stackoverflow.com/a/75356349)
