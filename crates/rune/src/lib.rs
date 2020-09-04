//! <div align="center">
//! <a href="https://rune-rs.github.io/rune/">
//!     <b>Read the Book 📖</b>
//! </a>
//! </div>
//!
//! <br>
//!
//! <div align="center">
//! <a href="https://github.com/rune-rs/rune/actions">
//!     <img alt="Build Status" src="https://github.com/rune-rs/rune/workflows/Build/badge.svg">
//! </a>
//!
//! <a href="https://github.com/rune-rs/rune/actions">
//!     <img alt="Book Status" src="https://github.com/rune-rs/rune/workflows/Book/badge.svg">
//! </a>
//!
//! <a href="https://discord.gg/v5AeNkT">
//!     <img alt="Chat on Discord" src="https://img.shields.io/discord/558644981137670144.svg?logo=discord&style=flat-square">
//! </a>
//! </div>
//!
//! <br>
//!
//! An embeddable dynamic programming language for Rust.
//!
//! ## Contributing
//!
//! If you want to help out, there should be a number of optimization tasks
//! available in [Future Optimizations][future-optimizations]. Or have a look at
//! [Open Issues].
//!
//! Create an issue about the optimization you want to work on and communicate that
//! you are working on it.
//!
//! <br>
//!
//! ## Features of Rune
//!
//! * Clean [Rust Integration 💻][support-rust-integration].
//! * Memory safe through [reference counting 📖][support-reference-counted].
//! * [Template strings 📖][support-templates].
//! * [Try operators 📖][support-try].
//! * Pattern matching [📖][support-patterns].
//! * [Structs and enums 📖][support-structs] with associated data and functions.
//! * Dynamic [vectors 📖][support-dynamic-vectors], [objects 📖][support-anon-objects], and [tuples 📖][support-anon-tuples] with built-in [serde support 💻][support-serde].
//! * First-class [async support 📖][support-async].
//! * [Generators 📖][support-generators].
//! * Dynamic [instance functions 📖][support-instance-functions].
//! * Stack isolation between function calls.
//! * Stack-based C FFI, like Lua's (TBD).
//!
//! <br>
//!
//! ## Rune Scripts
//!
//! You can run Rune programs with the bundled CLI:
//!
//! ```text
//! cargo run -- scripts/hello_world.rn
//! ```
//!
//! If you want to see detailed diagnostics of your program while it's running,
//! you can use:
//!
//! ```text
//! cargo run -- scripts/hello_world.rn --dump-unit --trace --dump-vm
//! ```
//!
//! See `--help` for more information.
//!
//! [future-optimizations]: https://github.com/rune-rs/rune/blob/master/FUTURE_OPTIMIZATIONS.md
//! [Open Issues]: https://github.com/rune-rs/rune/issues
//! [support-rust-integration]: https://github.com/rune-rs/rune/tree/master/crates/rune-modules
//! [support-reference-counted]: https://rune-rs.github.io/rune/4_2_variables.html
//! [support-templates]: https://rune-rs.github.io/rune/4_6_template_strings.html
//! [support-try]: https://rune-rs.github.io/rune/6_try_operator.html
//! [support-patterns]: https://rune-rs.github.io/rune/4_5_pattern_matching.html
//! [support-structs]: https://rune-rs.github.io/rune/5_6_structs.html
//! [support-async]: https://rune-rs.github.io/rune/8_async.html
//! [support-generators]: https://rune-rs.github.io/rune/7_generators.html
//! [support-instance-functions]: https://rune-rs.github.io/rune/4_7_instance_functions.html
//! [support-dynamic-vectors]: https://rune-rs.github.io/rune/5_2_vectors.html
//! [support-anon-objects]: https://rune-rs.github.io/rune/5_3_objects.html
//! [support-anon-tuples]: https://rune-rs.github.io/rune/5_4_tuples.html
//! [support-serde]: https://github.com/rune-rs/rune/blob/master/crates/rune-modules/src/json.rs

#![deny(missing_docs)]

pub mod ast;
mod compile;
mod compiler;
mod error;
mod index;
mod index_scopes;
mod items;
mod lexer;
mod loops;
mod options;
mod parser;
mod query;
#[cfg(feature = "runtime")]
mod runtime;
mod scopes;
mod source;
mod token;
mod traits;
mod warning;

pub use crate::error::{CompileError, Error, ParseError, Result};
pub use crate::lexer::Lexer;
pub use crate::options::Options;
pub use crate::parser::Parser;
#[cfg(feature = "runtime")]
pub use crate::runtime::{termcolor, Runtime};
pub use crate::source::Source;
pub use crate::token::{Kind, Token};
pub use crate::traits::{Parse, Resolve};
pub use crate::warning::{Warning, Warnings};
pub use runestick::unit::Span;
use runestick::Context;

/// Helper function to compile the given source.
///
/// Discards any warnings produced.
pub fn compile(context: &Context, source: &str) -> Result<(runestick::Unit, Warnings)> {
    let unit = parse_all::<ast::DeclFile>(&source)?;
    let (unit, warnings) = unit.compile(context)?;
    Ok((unit, warnings))
}

/// The result from parsing a string.
pub struct ParseAll<'a, T> {
    /// The source parsed.
    ///
    /// Is needed to resolve things on the item through [Resolve::resolve]
    /// later.
    pub source: Source<'a>,
    /// The item parsed.
    pub item: T,
}

impl<'a, T> ParseAll<'a, T>
where
    T: Resolve<'a>,
{
    /// Resolve the item encapsulated in the parse.
    pub fn resolve(&self) -> Result<T::Output, ParseError> {
        self.item.resolve(self.source)
    }
}

/// Parse the given input as the given type that implements
/// [Parse][crate::traits::Parse].
///
/// This required the whole input to be parsed.
///
/// Returns the wrapped source and the parsed type.
pub fn parse_all<T>(source: &str) -> Result<ParseAll<T>, ParseError>
where
    T: crate::traits::Parse,
{
    let mut parser = Parser::new(source);
    let ast = parser.parse::<T>()?;

    if let Some(token) = parser.lexer.next()? {
        return Err(ParseError::ExpectedEof {
            actual: token.kind,
            span: token.span,
        });
    }

    Ok(ParseAll {
        source: Source { source },
        item: ast,
    })
}

mod collections {
    pub use hashbrown::{hash_map, HashMap};
    pub use hashbrown::{hash_set, HashSet};
}
