//! # tera-rand
//!
//! A crate of random data generation functions for the [Tera] template engine.
//!
//! For a CLI tool implementation using this crate, see [tera-rand-cli].
//!
//! # Example usage
//! Import the function you would like to use, and register it with [`Tera::register_function`].
//! Your Tera templates may then use that function:
//!
//! ```edition2021
//! use tera::{Context, Tera};
//! use tera_rand::random_string;
//! use tera_rand::random_ipv4_cidr;
//!
//! let mut tera: Tera = Tera::default();
//! tera.register_function("random_string", random_string);
//! tera.register_function("random_ipv4_cidr", random_ipv4_cidr);
//!
//! let context: Context = Context::new();
//! // generate a random String and a random IPv4 CIDR address in a JSON template string
//! let rendered_json: String = tera
//!     .render_str(
//!         r#"{"hostname": "{{ random_string() }}", "subnet": "{{ random_ipv4_cidr() }}"}"#,
//!         &context
//!     )
//!     .unwrap();
//! ```
//!
//! Tera takes the template String, inserts random values wherever the functions were called, and
//! spits out a rendered String, which in this case might look like this:
//! ```json
//! {"hostname": "VtNCOgwH", "subnet": "196.119.240.0/23"}
//! ```
//!
//! Some functions provide customization parameters. For example, [`random_string`] provides a
//! `length` parameter to specify the length of the generated String, and [`random_ipv4_cidr`]
//! provides `length_start` and `length_end` parameters to limit the possible prefix lengths. The
//! template in the above example could be modified to this:
//! ```json
//! {
//!     "hostname": "{{ random_string(length=12) }}",
//!     "subnet": "{{ random_ipv4_cidr(length_start=28, length_end=30) }}"
//! }
//! ```
//! and the generated JSON could look like this:
//! ```json
//! {
//!     "hostname": "YCHcsV6bRkVW",
//!     "subnet": "171.150.226.224/29"
//! }
//! ```
//!
//! [Tera]: https://github.com/Keats/tera
//! [tera-rand-cli]: https://docs.rs/tera-rand-cli
//! [`Tera::register_function`]: https://docs.rs/tera/latest/tera/struct.Tera.html#method.register_function
//! [`random_string`]: crate::random_string
//! [`random_ipv4_cidr`]: crate::random_ipv4_cidr
#![warn(missing_debug_implementations, missing_docs)]

mod common;
mod error;

// public functions live in separate modules for maintainability,
// but expose them in the root module for searchability

mod file;
pub use file::*;

mod net;
pub use net::*;

mod primitives;
pub use primitives::*;

mod string;
pub use string::*;

mod uuid;
pub use crate::uuid::*;
