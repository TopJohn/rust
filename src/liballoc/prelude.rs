// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The alloc Prelude
//!
//! The purpose of this module is to alleviate imports of commonly-used
//! items of the `alloc` crate by adding a glob import to the top of modules:
//!
//! ```
//! # #![allow(unused_imports)]
//! # #![feature(alloc)]
//! extern crate alloc;
//! use alloc::prelude::*;
//! ```

#![stable(feature = "alloc_prelude", since = "1.29.0")]

#[stable(feature = "alloc_prelude", since = "1.29.0")] pub use borrow::ToOwned;
#[stable(feature = "alloc_prelude", since = "1.29.0")] pub use boxed::Box;
#[stable(feature = "alloc_prelude", since = "1.29.0")] pub use slice::SliceConcatExt;
#[stable(feature = "alloc_prelude", since = "1.29.0")] pub use string::{String, ToString};
#[stable(feature = "alloc_prelude", since = "1.29.0")] pub use vec::Vec;
