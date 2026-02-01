# rosetta-aisp

Bidirectional prose ↔ AISP symbolic notation conversion based on the Rosetta Stone mappings.

[![Crates.io](https://img.shields.io/crates/v/rosetta-aisp.svg)](https://crates.io/crates/rosetta-aisp)
[![Documentation](https://docs.rs/rosetta-aisp/badge.svg)](https://docs.rs/rosetta-aisp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **70+ Rosetta Stone mappings** for quantifiers, logic, sets, types, and more
- **3-tier conversion**: Minimal, Standard, and Full document levels
- **Round-trip support**: prose → AISP → prose with semantic preservation
- **Anti-drift guarantees**: Symbols maintain consistent meaning (Mean(s) ≡ Mean₀(s))
- **Based on AISP 5.1 Σ_512 glossary specification**

## Quick Start

```rust
use rosetta_aisp::{RosettaStone, AispConverter, ConversionTier};

// Simple prose to AISP conversion
let (aisp, confidence, unmapped) = RosettaStone::convert("for all x in S");
assert!(aisp.contains("∀"));  // "for all" → ∀
assert!(aisp.contains("∈"));  // "in" → ∈

// Convert back to prose
let prose = RosettaStone::to_prose(&aisp);
assert!(prose.contains("for all"));

// Full document conversion with auto tier detection
let result = AispConverter::convert("Define a type User with id and name", None);
println!("Tier: {}", result.tier);
println!("Output:\n{}", result.output);
```

## Conversion Tiers

| Tier | Description | Token Ratio |
|------|-------------|-------------|
| **Minimal** | Direct symbol substitution only | 0.5-1x |
| **Standard** | + Header, metadata, evidence blocks | 1.5-2x |
| **Full** | + Types, rules, errors, proofs | 4-8x |

```rust
use rosetta_aisp::{AispConverter, ConversionOptions, ConversionTier};

// Force a specific tier
let result = AispConverter::convert(
    "Define x as 5",
    Some(ConversionOptions {
        tier: Some(ConversionTier::Full),
        ..Default::default()
    }),
);
```

## Symbol Categories

| Category | Example Symbols | Prose Patterns |
|----------|-----------------|----------------|
| Quantifiers | ∀, ∃, ∃!, ∄ | "for all", "exists", "exactly one" |
| Logic | ∧, ∨, ¬, ⇒, ⇔ | "and", "or", "not", "implies" |
| Comparison | ≡, ≢, >, <, ≥, ≤ | "equals", "not equal", "greater than" |
| Definition | ≜, ≔, ↦ | "defined as", "assigned", "maps to" |
| Sets | ∈, ∉, ⊆, ∪, ∩ | "in", "not in", "subset", "union" |
| Types | ℕ, ℤ, ℝ, 𝔹, 𝕊 | "natural", "integer", "boolean", "string" |
| Truth | ⊤, ⊥ | "true", "false" |
| Blocks | ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧ | metadata, types, rules, functions |

## Round-Trip Guarantees

The library ensures semantic preservation through multiple conversion cycles:

```rust
use rosetta_aisp::RosettaStone;

let original = "for all users, if admin then allow access";
let (aisp, _, _) = RosettaStone::convert(original);
let prose = RosettaStone::to_prose(&aisp);

// Check semantic similarity
let similarity = RosettaStone::semantic_similarity(original, &prose);
assert!(similarity > 0.4); // Maintains meaning
```

## AISP Document Output Example

```aisp
𝔸5.1.user@2026-01-31
γ≔user.definitions
ρ≔⟨user,types,rules⟩

⟦Ω:Meta⟧{
  domain≜user
  version≜1.0.0
  ∀D∈AISP:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  User≜⟨id:ℕ,name:𝕊⟩
}

⟦Γ:Rules⟧{
  ∀u∈User:u.admin≡⊤⇒allow(u)
}

⟦Λ:Funcs⟧{
  ∀ users u, if u is admin⇒allow access.
}

⟦Ε⟧⟨δ≜0.82;φ≜100;τ≜◊⁺⁺;⊢valid;∎⟩
```

## License

MIT
