//! Rosetta Stone - Bidirectional prose ↔ AISP symbol mappings
//!
//! Based on AISP 5.1 Σ_512 glossary specification.
//! Ported from aisp-converter npm package.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Rosetta Stone mapping entry
#[derive(Debug, Clone)]
pub struct RosettaEntry {
    pub symbol: &'static str,
    pub patterns: &'static [&'static str],
    pub category: &'static str,
}

/// Complete Rosetta Stone mappings (AISP 5.1 Σ_512)
/// Ported from aisp-converter npm package
pub static ROSETTA: &[RosettaEntry] = &[
    // ═══════════════════════════════════════════════════════════════
    // QUANTIFIERS (∀:Quantifiers[128-191])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "∀",
        patterns: &["for all", "for every", "every", "all", "each", "any"],
        category: "quantifier",
    },
    RosettaEntry {
        symbol: "∃",
        patterns: &["there exists", "exists", "some", "at least one", "there is"],
        category: "quantifier",
    },
    RosettaEntry {
        symbol: "∃!",
        patterns: &[
            "exists unique",
            "exactly one",
            "unique",
            "one and only one",
            "exists exactly one",
        ],
        category: "quantifier",
    },
    RosettaEntry {
        symbol: "∄",
        patterns: &["does not exist", "no such", "none exists"],
        category: "quantifier",
    },
    // ═══════════════════════════════════════════════════════════════
    // LOGIC (Ω:Transmuters[0-63])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "∧",
        patterns: &["and", "both", "as well as", "together with", "also"],
        category: "logic",
    },
    RosettaEntry {
        symbol: "∨",
        patterns: &["or", "either", "alternatively", "otherwise"],
        category: "logic",
    },
    RosettaEntry {
        symbol: "¬",
        patterns: &["not", "negation", "isn't", "is not", "doesn't", "does not"],
        category: "logic",
    },
    RosettaEntry {
        symbol: "⇒",
        patterns: &[
            "implies",
            "if then",
            "therefore",
            "then",
            "consequently",
            "so",
            "hence",
        ],
        category: "logic",
    },
    RosettaEntry {
        symbol: "⇔",
        patterns: &[
            "if and only if",
            "iff",
            "equivalent to",
            "is equivalent",
            "exactly when",
        ],
        category: "logic",
    },
    RosettaEntry {
        symbol: "→",
        patterns: &["to", "returns", "maps to", "yields", "produces", "goes to"],
        category: "logic",
    },
    RosettaEntry {
        symbol: "↔",
        patterns: &["bidirectional", "two-way", "both ways"],
        category: "logic",
    },
    RosettaEntry {
        symbol: "⊕",
        patterns: &["xor", "exclusive or", "either but not both"],
        category: "logic",
    },
    // ═══════════════════════════════════════════════════════════════
    // COMPARISON
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: ">",
        patterns: &[
            "greater than",
            "more than",
            "exceeds",
            "above",
            "larger than",
        ],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "<",
        patterns: &["less than", "fewer than", "below", "smaller than", "under"],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "≥",
        patterns: &[
            "greater than or equal",
            "at least",
            "no less than",
            "minimum",
            ">=",
        ],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "≤",
        patterns: &[
            "less than or equal",
            "at most",
            "no more than",
            "maximum",
            "<=",
        ],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "≡",
        patterns: &[
            "identical to",
            "equals",
            "is equal to",
            "same as",
            "equivalent",
            "===",
            "==",
        ],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "≢",
        patterns: &[
            "not identical",
            "not equal",
            "differs from",
            "different from",
            "!==",
            "!=",
        ],
        category: "comparison",
    },
    RosettaEntry {
        symbol: "≈",
        patterns: &["approximately", "roughly", "about", "nearly"],
        category: "comparison",
    },
    // ═══════════════════════════════════════════════════════════════
    // DEFINITION (Ω:Transmuters)
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "≜",
        patterns: &[
            "defined as",
            "is defined as",
            "equals by definition",
            "is a",
            "means",
            "definition",
        ],
        category: "definition",
    },
    RosettaEntry {
        symbol: "≔",
        patterns: &["assigned", "set to", "becomes", "gets", "is assigned", ":="],
        category: "definition",
    },
    RosettaEntry {
        symbol: "↦",
        patterns: &["mapsto", "maps to", "sends to"],
        category: "definition",
    },
    // ═══════════════════════════════════════════════════════════════
    // FUNCTIONS (λ calculus)
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "λ",
        patterns: &[
            "lambda",
            "function",
            "anonymous function",
            "fn",
            "func",
            "=>",
        ],
        category: "function",
    },
    RosettaEntry {
        symbol: "∘",
        patterns: &["compose", "composed with", "followed by"],
        category: "function",
    },
    RosettaEntry {
        symbol: "fix",
        patterns: &["fixpoint", "recursive", "fixed point"],
        category: "function",
    },
    RosettaEntry {
        symbol: "μ",
        patterns: &["least fixpoint", "lfp", "mu"],
        category: "function",
    },
    // ═══════════════════════════════════════════════════════════════
    // SETS (Γ:Topologics[64-127])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "∈",
        patterns: &["in", "element of", "member of", "belongs to", "is in"],
        category: "set",
    },
    RosettaEntry {
        symbol: "∉",
        patterns: &["not in", "not element of", "not member of", "outside"],
        category: "set",
    },
    RosettaEntry {
        symbol: "⊆",
        patterns: &["subset", "subset of", "contained in", "part of"],
        category: "set",
    },
    RosettaEntry {
        symbol: "⊇",
        patterns: &["superset", "superset of", "contains"],
        category: "set",
    },
    RosettaEntry {
        symbol: "⊂",
        patterns: &["proper subset", "strict subset"],
        category: "set",
    },
    RosettaEntry {
        symbol: "⊃",
        patterns: &["proper superset", "strict superset"],
        category: "set",
    },
    RosettaEntry {
        symbol: "∪",
        patterns: &["union", "combined with", "merged with"],
        category: "set",
    },
    RosettaEntry {
        symbol: "∩",
        patterns: &["intersection", "overlapping with", "common to", "shared by"],
        category: "set",
    },
    RosettaEntry {
        symbol: "∅",
        patterns: &["empty", "empty set", "null", "nothing", "nil", "void"],
        category: "set",
    },
    RosettaEntry {
        symbol: "𝒫",
        patterns: &["powerset", "power set", "all subsets"],
        category: "set",
    },
    RosettaEntry {
        symbol: "∖",
        patterns: &["set difference", "minus", "except", "without"],
        category: "set",
    },
    RosettaEntry {
        symbol: "𝔾",
        patterns: &["graph", "network", "structure"],
        category: "set",
    },
    // ═══════════════════════════════════════════════════════════════
    // CONTRACTORS (Δ:Contractors[192-255])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "Δ",
        patterns: &["delta", "difference", "change", "increment"],
        category: "contractor",
    },
    RosettaEntry {
        symbol: "Pre",
        patterns: &["precondition", "requires", "before"],
        category: "contractor",
    },
    RosettaEntry {
        symbol: "Post",
        patterns: &["postcondition", "ensures", "after", "guarantees"],
        category: "contractor",
    },
    RosettaEntry {
        symbol: "Inv",
        patterns: &["invariant", "always true", "maintained"],
        category: "contractor",
    },
    // ═══════════════════════════════════════════════════════════════
    // INTENTS (Ψ:Intents[320-383])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "Ψ",
        patterns: &["intent", "goal", "purpose", "objective"],
        category: "intent",
    },
    RosettaEntry {
        symbol: "μ",
        patterns: &["fitness", "utility", "score", "metric"],
        category: "intent",
    },
    RosettaEntry {
        symbol: "Target",
        patterns: &["target", "aim", "destination"],
        category: "intent",
    },
    // ═══════════════════════════════════════════════════════════════
    // TYPES (𝔻:Domaines[256-319])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "ℕ",
        patterns: &[
            "natural",
            "natural number",
            "positive integer",
            "nat",
            "natural numbers",
            "unsigned",
        ],
        category: "type",
    },
    RosettaEntry {
        symbol: "ℤ",
        patterns: &[
            "integer",
            "int",
            "whole number",
            "integers",
            "signed integer",
        ],
        category: "type",
    },
    RosettaEntry {
        symbol: "ℝ",
        patterns: &[
            "real",
            "real number",
            "float",
            "decimal",
            "double",
            "number",
        ],
        category: "type",
    },
    RosettaEntry {
        symbol: "ℚ",
        patterns: &["rational", "rational number", "fraction"],
        category: "type",
    },
    RosettaEntry {
        symbol: "𝔹",
        patterns: &["boolean", "bool", "true or false", "binary", "flag"],
        category: "type",
    },
    RosettaEntry {
        symbol: "𝕊",
        patterns: &["string", "str", "text", "char sequence", "varchar"],
        category: "type",
    },
    RosettaEntry {
        symbol: "ℂ",
        patterns: &["complex", "complex number"],
        category: "type",
    },
    RosettaEntry {
        symbol: "List",
        patterns: &["list", "array", "sequence", "vector"],
        category: "type",
    },
    RosettaEntry {
        symbol: "Maybe",
        patterns: &["maybe", "optional", "nullable", "option"],
        category: "type",
    },
    RosettaEntry {
        symbol: "Either",
        patterns: &["either", "result", "union type"],
        category: "type",
    },
    // ═══════════════════════════════════════════════════════════════
    // TRUTH VALUES
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "⊤",
        patterns: &["true", "top", "yes", "valid", "correct", "success", "ok"],
        category: "truth",
    },
    RosettaEntry {
        symbol: "⊥",
        patterns: &[
            "false",
            "bottom",
            "no",
            "invalid",
            "incorrect",
            "failure",
            "crash",
            "error",
        ],
        category: "truth",
    },
    // ═══════════════════════════════════════════════════════════════
    // SPECIAL (proofs, assertions)
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "∎",
        patterns: &["qed", "proven", "end of proof", "proved", "done"],
        category: "special",
    },
    RosettaEntry {
        symbol: "⊢",
        patterns: &["proves", "entails", "derives", "turnstile", "yields"],
        category: "special",
    },
    RosettaEntry {
        symbol: "⊨",
        patterns: &["models", "satisfies", "validates"],
        category: "special",
    },
    RosettaEntry {
        symbol: "□",
        patterns: &["necessarily", "always", "box"],
        category: "special",
    },
    RosettaEntry {
        symbol: "◇",
        patterns: &["possibly", "eventually", "diamond"],
        category: "special",
    },
    // ═══════════════════════════════════════════════════════════════
    // MATH OPERATORS
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "+",
        patterns: &["plus", "added to", "sum of", "add"],
        category: "math",
    },
    RosettaEntry {
        symbol: "−",
        patterns: &["minus", "subtract", "subtracted from"],
        category: "math",
    },
    RosettaEntry {
        symbol: "×",
        patterns: &["times", "multiplied by", "product of", "multiply"],
        category: "math",
    },
    RosettaEntry {
        symbol: "÷",
        patterns: &["divided by", "over", "ratio of", "divide"],
        category: "math",
    },
    RosettaEntry {
        symbol: "²",
        patterns: &["squared", "square of", "to the power of 2"],
        category: "math",
    },
    RosettaEntry {
        symbol: "³",
        patterns: &["cubed", "cube of", "to the power of 3"],
        category: "math",
    },
    RosettaEntry {
        symbol: "√",
        patterns: &["square root", "sqrt", "root of"],
        category: "math",
    },
    RosettaEntry {
        symbol: "Σ",
        patterns: &["sum", "summation", "sigma"],
        category: "math",
    },
    RosettaEntry {
        symbol: "Π",
        patterns: &["product", "pi", "prod"],
        category: "math",
    },
    RosettaEntry {
        symbol: "∞",
        patterns: &["infinity", "infinite", "unbounded"],
        category: "math",
    },
    // ═══════════════════════════════════════════════════════════════
    // BLOCK MARKERS (⟦⟧:Delimiters[384-447])
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "⟦Ω⟧",
        patterns: &["meta block", "metadata", "foundation"],
        category: "block",
    },
    RosettaEntry {
        symbol: "⟦Σ⟧",
        patterns: &["types block", "type definitions", "glossary"],
        category: "block",
    },
    RosettaEntry {
        symbol: "⟦Γ⟧",
        patterns: &["rules block", "business rules", "constraints"],
        category: "block",
    },
    RosettaEntry {
        symbol: "⟦Λ⟧",
        patterns: &["functions block", "function definitions", "lambdas"],
        category: "block",
    },
    RosettaEntry {
        symbol: "⟦Χ⟧",
        patterns: &["errors block", "error handling", "exceptions"],
        category: "block",
    },
    RosettaEntry {
        symbol: "⟦Ε⟧",
        patterns: &["evidence block", "proof", "validation"],
        category: "block",
    },
    // ═══════════════════════════════════════════════════════════════
    // TUPLES & RECORDS
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "⟨",
        patterns: &["tuple start", "record start", "angle open"],
        category: "special",
    },
    RosettaEntry {
        symbol: "⟩",
        patterns: &["tuple end", "record end", "angle close"],
        category: "special",
    },
    // ═══════════════════════════════════════════════════════════════
    // QUALITY TIERS
    // ═══════════════════════════════════════════════════════════════
    RosettaEntry {
        symbol: "◊⁺⁺",
        patterns: &["platinum", "platinum tier", "optimal"],
        category: "tier",
    },
    RosettaEntry {
        symbol: "◊⁺",
        patterns: &["gold", "gold tier", "production ready"],
        category: "tier",
    },
    RosettaEntry {
        symbol: "◊",
        patterns: &["silver", "silver tier", "good"],
        category: "tier",
    },
    RosettaEntry {
        symbol: "◊⁻",
        patterns: &["bronze", "bronze tier", "acceptable"],
        category: "tier",
    },
    RosettaEntry {
        symbol: "⊘",
        patterns: &["reject", "rejected", "invalid tier"],
        category: "tier",
    },
];

lazy_static! {
    /// Rosetta entries sorted by longest pattern first (greedy matching)
    pub static ref ROSETTA_SORTED: Vec<&'static RosettaEntry> = {
        let mut entries: Vec<_> = ROSETTA.iter().collect();
        entries.sort_by(|a, b| {
            let max_a = a.patterns.iter().map(|p| p.len()).max().unwrap_or(0);
            let max_b = b.patterns.iter().map(|p| p.len()).max().unwrap_or(0);
            max_b.cmp(&max_a)
        });
        entries
    };

    /// Pattern to symbol lookup
    pub static ref PATTERN_TO_SYMBOL: HashMap<String, &'static str> = {
        let mut m = HashMap::new();
        for entry in ROSETTA {
            for pattern in entry.patterns {
                m.insert(pattern.to_lowercase(), entry.symbol);
            }
        }
        m
    };

    /// Symbol to primary pattern lookup
    pub static ref SYMBOL_TO_PATTERN: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        for entry in ROSETTA {
            if let Some(first) = entry.patterns.first() {
                m.insert(entry.symbol, *first);
            }
        }
        m
    };

    /// Compiled Rosetta entries for efficient matching
    pub static ref ROSETTA_COMPILED: Vec<CompiledRosettaEntry> = {
        ROSETTA_SORTED.iter().map(|entry| {
            let compiled_patterns = entry.patterns.iter().filter_map(|pattern| {
                let regex_str = format!(r"(?i)\b{}\b", escape_regex(pattern));
                Regex::new(&regex_str).ok()
            }).collect();
            
            CompiledRosettaEntry {
                symbol: entry.symbol,
                regexes: compiled_patterns,
            }
        }).collect()
    };
}

/// Pre-compiled Rosetta entry
pub struct CompiledRosettaEntry {
    pub symbol: &'static str,
    pub regexes: Vec<Regex>,
}

/// Find symbol for a prose pattern
pub fn prose_to_symbol(pattern: &str) -> Option<&'static str> {
    PATTERN_TO_SYMBOL
        .get(&pattern.to_lowercase().trim().to_string())
        .copied()
}

/// Find primary prose pattern for a symbol
pub fn symbol_to_prose(symbol: &str) -> Option<&'static str> {
    SYMBOL_TO_PATTERN.get(symbol).copied()
}

/// Get all symbols in a category
pub fn symbols_by_category(category: &str) -> Vec<&'static str> {
    ROSETTA
        .iter()
        .filter(|e| e.category == category)
        .map(|e| e.symbol)
        .collect()
}

/// Get all categories
pub fn get_all_categories() -> Vec<&'static str> {
    let mut categories: Vec<_> = ROSETTA.iter().map(|e| e.category).collect();
    categories.sort();
    categories.dedup();
    categories
}

/// Count total mappings
pub fn get_mapping_count() -> usize {
    ROSETTA.iter().map(|e| e.patterns.len()).sum()
}

/// Escape regex special characters
fn escape_regex(s: &str) -> String {
    let special = [
        '\\', '.', '*', '+', '?', '^', '$', '{', '}', '(', ')', '|', '[', ']',
    ];
    let mut result = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if special.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

/// Rosetta Stone converter
pub struct RosettaStone;

impl RosettaStone {
    /// Convert prose to AISP symbols using deterministic mappings
    /// Returns (converted_text, mapped_chars, unmapped_words)
    pub fn convert(input: &str) -> (String, usize, Vec<String>) {
        let mut result = input.to_string();
        let mut mapped_chars = 0;
        let _total_chars = input.len();

        // Apply Rosetta mappings (longest patterns first) using pre-compiled regexes
        for entry in ROSETTA_COMPILED.iter() {
            for regex in entry.regexes.iter() {
                let matches: Vec<_> = regex.find_iter(&result).collect();
                mapped_chars += matches.iter().map(|m| m.as_str().len()).sum::<usize>();
                result = regex.replace_all(&result, entry.symbol).to_string();
            }
        }

        // Clean up operators (remove extra spaces)
        result = Self::cleanup_operators(&result);

        // Convert assignment patterns
        result = Self::convert_assignments(&result);

        // Find unmapped words
        let unmapped = Self::find_unmapped_words(&result);

        (result.trim().to_string(), mapped_chars, unmapped)
    }

    /// Calculate conversion confidence
    pub fn confidence(input_len: usize, mapped_chars: usize) -> f64 {
        if input_len == 0 {
            return 1.0;
        }
        (mapped_chars as f64 / input_len as f64).min(1.0)
    }

    /// Clean up operators by removing extra spaces
    fn cleanup_operators(input: &str) -> String {
        let operators = ["≜", "≔", "⇒", "∈", "→", "⇔", "∧", "∨"];
        let mut result = input.to_string();

        for op in operators {
            let regex_str = format!(r"\s*{}\s*", escape_regex(op));
            if let Ok(regex) = Regex::new(&regex_str) {
                result = regex.replace_all(&result, op).to_string();
            }
        }

        result
    }

    /// Convert common assignment patterns
    fn convert_assignments(input: &str) -> String {
        let mut result = input.to_string();

        // Convert "const x = 5" to "x≜5"
        if let Ok(regex) = Regex::new(r"(?i)const\s+(\w+)\s*=\s*(\S+)") {
            result = regex.replace_all(&result, "$1≜$2").to_string();
        }

        // Convert "Define x as y" to "x≜y"
        if let Ok(regex) = Regex::new(r"(?i)Define\s+(\w+)\s+as\s+(\S+)") {
            result = regex.replace_all(&result, "$1≜$2").to_string();
        }

        // Convert "let x = y" to "x≜y"
        if let Ok(regex) = Regex::new(r"(?i)let\s+(\w+)\s*=\s*(\S+)") {
            result = regex.replace_all(&result, "$1≜$2").to_string();
        }

        result
    }

    /// Find words that weren't mapped to symbols
    fn find_unmapped_words(result: &str) -> Vec<String> {
        let ignore_words = [
            "the", "with", "that", "this", "from", "into", "when", "where", "which", "what",
        ];

        let word_regex = Regex::new(r"\b[a-zA-Z]{3,}\b").unwrap();
        let words: Vec<_> = word_regex
            .find_iter(result)
            .map(|m| m.as_str().to_lowercase())
            .collect();

        let mut unique: Vec<_> = words
            .into_iter()
            .filter(|w| !ignore_words.contains(&w.as_str()))
            .collect();

        unique.sort();
        unique.dedup();
        unique
    }

    /// Convert AISP symbols back to prose
    /// Maintains spacing for readability while preserving semantic meaning
    pub fn to_prose(input: &str) -> String {
        let mut result = input.to_string();

        // Sort by symbol length (longest first) to avoid partial replacements
        let mut entries: Vec<_> = ROSETTA.iter().collect();
        entries.sort_by(|a, b| b.symbol.len().cmp(&a.symbol.len()));

        for entry in entries {
            if let Some(primary) = entry.patterns.first() {
                // Add spaces around word replacements for readability
                let replacement = format!(" {} ", primary);
                result = result.replace(entry.symbol, &replacement);
            }
        }

        // Ensure spaces between letters that got concatenated
        // Handles cases like "adminimpliesallow" → "admin implies allow"
        result = Self::add_word_boundaries(&result);

        // Clean up multiple spaces and trim
        Self::normalize_whitespace(&result)
    }

    /// Add spaces between concatenated words
    fn add_word_boundaries(input: &str) -> String {
        // Add space between lowercase followed by uppercase
        let camel_case = Regex::new(r"([a-z])([A-Z])").unwrap();
        let result = camel_case.replace_all(input, "$1 $2");

        // Add space before words that follow certain patterns
        let word_join = Regex::new(r"([a-zA-Z])( )(for all|exists|implies|and|or|not|if|then|else|in|defined as|identical to|true|false|lambda|function|returns|boolean|integer|string|natural|real|proves|therefore|yields)( )").unwrap();
        let result = word_join.replace_all(&result, "$1 $3 ");

        result.to_string()
    }

    /// Normalize whitespace in text
    fn normalize_whitespace(input: &str) -> String {
        let multiple_spaces = Regex::new(r"\s+").unwrap();
        let result = multiple_spaces.replace_all(input, " ");

        // Clean up spaces around punctuation
        let space_before_punct = Regex::new(r"\s+([.,;:!?])").unwrap();
        let result = space_before_punct.replace_all(&result, "$1");

        // Clean up spaces after opening brackets
        let space_after_open = Regex::new(r"([(\[{])\s+").unwrap();
        let result = space_after_open.replace_all(&result, "$1");

        // Clean up spaces before closing brackets
        let space_before_close = Regex::new(r"\s+([)\]}])").unwrap();
        let result = space_before_close.replace_all(&result, "$1");

        result.trim().to_string()
    }

    /// Normalize text for semantic comparison (removes formatting differences)
    pub fn normalize_for_comparison(input: &str) -> String {
        let lowercase = input.to_lowercase();
        let normalized = Self::normalize_whitespace(&lowercase);

        // Remove punctuation for semantic comparison
        let punct_regex = Regex::new(r#"[.,;:!?"']"#).unwrap();
        punct_regex.replace_all(&normalized, "").trim().to_string()
    }

    /// Check semantic equivalence between two texts
    /// Returns similarity score from 0.0 to 1.0
    pub fn semantic_similarity(text1: &str, text2: &str) -> f64 {
        let norm1 = Self::normalize_for_comparison(text1);
        let norm2 = Self::normalize_for_comparison(text2);

        // Extract words
        let words1: HashSet<_> = norm1.split_whitespace().collect();
        let words2: HashSet<_> = norm2.split_whitespace().collect();

        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }

        // Jaccard similarity
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            1.0
        } else {
            intersection as f64 / union as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prose_to_symbol() {
        assert_eq!(prose_to_symbol("for all"), Some("∀"));
        assert_eq!(prose_to_symbol("exists"), Some("∃"));
        assert_eq!(prose_to_symbol("unknown"), None);
    }

    #[test]
    fn test_convert_basic() {
        let (result, _, _) = RosettaStone::convert("for all x in S");
        assert!(result.contains("∀"));
        assert!(result.contains("∈"));
    }

    #[test]
    fn test_convert_assignment() {
        let (result, _, _) = RosettaStone::convert("Define x as 5");
        assert!(result.contains("≜"));
    }

    #[test]
    fn test_mapping_count() {
        assert!(get_mapping_count() > 300);
    }

    #[test]
    fn test_to_prose_basic() {
        let prose = RosettaStone::to_prose("∀x∈S");
        assert!(prose.contains("for all"));
        assert!(prose.contains("in"));
    }

    #[test]
    fn test_to_prose_spacing() {
        let prose = RosettaStone::to_prose("x≜5∧y≜10");
        // Should have spaces for readability
        assert!(prose.contains("defined as"));
        assert!(prose.contains("and"));
    }

    #[test]
    fn test_round_trip_simple() {
        let original = "for all x in S";
        let (aisp, _, _) = RosettaStone::convert(original);
        let prose = RosettaStone::to_prose(&aisp);

        // Check semantic similarity
        let similarity = RosettaStone::semantic_similarity(original, &prose);
        assert!(
            similarity > 0.5,
            "Round trip lost too much meaning: {:.2}",
            similarity
        );
    }

    #[test]
    fn test_round_trip_complex() {
        let original = "Define x as 5 and for all y in S, if x equals y then return true";
        let (aisp, _, _) = RosettaStone::convert(original);
        let prose = RosettaStone::to_prose(&aisp);

        let similarity = RosettaStone::semantic_similarity(original, &prose);
        assert!(
            similarity > 0.4,
            "Complex round trip lost meaning: {:.2}",
            similarity
        );
    }

    #[test]
    fn test_semantic_similarity() {
        // Identical texts
        assert_eq!(
            RosettaStone::semantic_similarity("hello world", "hello world"),
            1.0
        );

        // Similar texts
        let sim = RosettaStone::semantic_similarity("for all x in set S", "for all x in S");
        assert!(sim > 0.7);

        // Different texts
        let sim = RosettaStone::semantic_similarity("apple banana cherry", "dog cat bird");
        assert!(sim < 0.2);
    }

    #[test]
    fn test_normalize_whitespace() {
        let result = RosettaStone::normalize_whitespace("  hello   world  ");
        assert_eq!(result, "hello world");

        let result = RosettaStone::normalize_whitespace("x ( a , b )");
        assert_eq!(result, "x (a, b)");
    }

    #[test]
    fn test_anti_drift_guarantee() {
        // AISP Anti-drift rule: Mean(s) ≡ Mean_0(s)
        // Symbols should maintain consistent meaning through round-trips
        let symbols_to_test = vec![
            ("∀", "for all"),
            ("∃", "exists"),
            ("⇒", "implies"),
            ("∈", "in"),
            ("≜", "defined as"),
            ("∧", "and"),
            ("∨", "or"),
        ];

        for (symbol, expected_prose) in symbols_to_test {
            let prose = RosettaStone::to_prose(symbol);
            assert!(
                prose.to_lowercase().contains(expected_prose),
                "Symbol {} should map to '{}', got '{}'",
                symbol,
                expected_prose,
                prose
            );
        }
    }
}
