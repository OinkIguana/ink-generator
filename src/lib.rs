#![feature(generators, generator_trait, iterator_find_map, nll)]
#![recursion_limit = "128"]

#[cfg(feature = "compiler")]
mod error;
#[cfg(feature = "compiler")]
mod parser;
#[cfg(feature = "compiler")]
mod prettyprinter;

#[cfg(feature = "runtime")]
#[macro_use]
pub mod runtime;

#[cfg(feature = "compiler")]
pub use self::error::Error;
#[cfg(feature = "compiler")]
pub use self::parser::{parse, Ink};
#[cfg(feature = "compiler")]
pub use self::prettyprinter::pretty_print;
