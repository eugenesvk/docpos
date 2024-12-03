# Changelog
All notable changes to this project will be documented in this file

[unreleased]: https://github.com/eugenesvk/docpos/compare/0.2.0...HEAD
## [Unreleased]
<!-- - ✨ __Added__ -->
  <!-- + new features -->
<!-- - Δ __Changed__ -->
  <!-- + changes in existing functionality -->
<!-- - 🐞 __Fixed__ -->
  <!-- + bug fixes -->
<!-- - 💩 __Deprecated__ -->
  <!-- + soon-to-be removed features -->
<!-- - 🗑️ __Removed__ -->
  <!-- + now removed features -->
<!-- - 🔒 __Security__ -->
  <!-- + vulnerabilities -->

[0.2.0]: https://github.com/eugenesvk/docpos/releases/tag/0.2.0
## [0.2.0]
  - Δ __Changed__
    + `#[docpos("fn")]` to accept bare identifiers `fn` instead of quoted `"fn"`
    + enum by default doesn't add its own section since the default empty section can't be removed(?), needs `#[docpos(enum_sect)]`
    + structs by default don't add their own section since the default empty section can't be removed(?), needs `#[docpos(struct_sect)]`

[0.1.0]: https://github.com/eugenesvk/docpos/releases/tag/0.1.0
## [0.1.0]
  - ✨ __Added__
    + Support for `enum` and `struct`
