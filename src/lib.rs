//! Pretty parser error reporting.
//!
//! # Examples
//! ```
//! use chic::Error;
//!
//! fn main() {
//!     let src = r#"This is an example
//! content of the slice
//! which will be annotated
//! with the list of annotations below.
//! "#;
//!
//!     let msg = Error::new("expected type, found `x`".to_string())
//!         .error(260, 0, 12, src.to_string(), "found `x`".to_string())
//!         .help("try using a foobs intead".to_string())
//!         .to_string();
//!
//!     println!("{}", msg);
//! }
//! ```

pub use error::Error;

mod error;
