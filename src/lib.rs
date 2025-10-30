//! libnm (Network Manager) raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod enumerations;
mod functions;
mod objects;

pub use enumerations::*;
pub use functions::*;
pub use objects::*;
