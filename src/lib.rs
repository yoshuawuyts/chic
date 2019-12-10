//! Pretty parser error reporting.
//!
//! # Examples
//!
//! ```
//! use chic::Error;
//!
//! let src = r#"This is an example
//! content of the slice
//! which will be annotated
//! with the list of annotations below.
//! "#;
//!
//! let msg = Error::new("expected type, found `x`")
//!     .error(260, 0, 12, src, "found `x`")
//!     .help("try using a foobs intead")
//!     .to_string();
//!
//! println!("{}", msg);
//! ```
//!
//! Outputs:
//!
//! ```txt
//! error: expected type, found `x`
//!     |
//! 260 | This is an example
//!     | ^^^^^^^^^^^^ found `x`
//!     |
//!     = help: try using a foobs instead
//! ```
//!
//! Or convert an `io::Cursor` to an annotated error:
//!
//! ```
//! use std::io::Cursor;
//! use chic::Error;
//!
//! let cursor = Cursor::new(
//!     r#"This is an example
//! content of the slice
//! which will be annotated
//! with the list of annotations below.
//! "#,
//! );
//!
//! let line = 1;
//! let start = cursor.position() as usize;
//! let end = cursor.get_ref().len() as usize;
//! let code = cursor.into_inner();
//!
//! let msg = Error::new("expected type, found `x`")
//!     .error(line, start, end, code, "found `x`")
//!     .help("try using a foobs instead")
//!     .to_string();
//!
//! println!("{}", msg);
//! ```

pub use error::Error;
pub use warning::Warning;

mod error;
mod warning;
