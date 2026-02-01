//! # rosetta-aisp
//!
//! Bidirectional prose ↔ AISP symbolic notation conversion based on the Rosetta Stone mappings.
//!
//! This crate provides deterministic, lossless conversion between natural language prose
//! and AISP (AI Symbolic Programming) notation based on the AISP 5.1 Σ_512 glossary specification.
//!
//! ## Features
//!
//! - **Rosetta Stone mappings**: 70+ symbol mappings for quantifiers, logic, sets, types, and more
//! - **3-tier conversion**: Minimal, Standard, and Full conversion levels
//! - **Round-trip support**: Convert prose → AISP → prose with semantic preservation
//! - **Anti-drift guarantees**: Symbols maintain consistent meaning through conversions
//!
//! ## Quick Start
//!
//! ```rust
//! use rosetta_aisp::{RosettaStone, AispConverter, ConversionTier};
//!
//! // Simple prose to AISP conversion
//! let (aisp, confidence, unmapped) = RosettaStone::convert("for all x in S");
//! assert!(aisp.contains("∀"));
//! assert!(aisp.contains("∈"));
//!
//! // Convert back to prose
//! let prose = RosettaStone::to_prose(&aisp);
//! assert!(prose.contains("for all"));
//!
//! // Full document conversion with auto tier detection
//! let result = AispConverter::convert("Define a type User with id and name", None);
//! println!("Tier: {}", result.tier);
//! println!("Output: {}", result.output);
//! ```
//!
//! ## Conversion Tiers
//!
//! - **Minimal**: Direct symbol substitution only (0.5-1x tokens)
//! - **Standard**: Adds header, metadata, and evidence blocks (1.5-2x tokens)
//! - **Full**: Complete AISP document with types, rules, and proofs (4-8x tokens)

mod converter;
mod rosetta;

pub use converter::{
    AispConverter, ConversionOptions, ConversionResult, ConversionTier, TokenStats,
};
pub use rosetta::{
    get_all_categories, get_mapping_count, prose_to_symbol, symbol_to_prose, symbols_by_category,
    CompiledRosettaEntry, RosettaEntry, RosettaStone, ROSETTA, ROSETTA_COMPILED, ROSETTA_SORTED,
};

/// Prelude for convenient imports
pub mod prelude {
    pub use crate::converter::{
        AispConverter, ConversionOptions, ConversionResult, ConversionTier, TokenStats,
    };
    pub use crate::rosetta::{prose_to_symbol, symbol_to_prose, RosettaStone};
}
