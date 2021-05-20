// Copyright 2015 Ted Mielczarek. See the COPYRIGHT
// file at the top-level directory of this distribution.

//! A parser for the minidump file format.
//!
//! The `minidump` module provides a parser for the
//! [minidump][minidump] file format as produced by Microsoft's
//! [`MinidumpWriteDump`][minidumpwritedump] API and the
//! [Google Breakpad][breakpad] library.
//!
//! The primary API for this module is the [`Minidump`][struct_minidump]
//! struct, which can be instantiated by calling the [`Minidump::read`][read] or
//! [`Minidump::read_path`][read_path] methods.
//!
//! [minidump]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms680369%28v=vs.85%29.aspx
//! [minidumpwritedump]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms680360%28v=vs.85%29.aspx
//! [breakpad]: https://chromium.googlesource.com/breakpad/breakpad/+/master/
//! [struct_minidump]: struct.Minidump.html
//! [read]: struct.Minidump.html#method.read
//! [read_path]: struct.Minidump.html#method.read_path

#![warn(missing_debug_implementations)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub use scroll::Endian;

mod context;
mod iostuff;
mod minidump;

pub use minidump_common::format;
pub use minidump_common::traits::Module;

pub use crate::iostuff::Readable;
pub use crate::minidump::*;

#[cfg(test)]
pub mod synth_minidump;
pub mod system_info;
