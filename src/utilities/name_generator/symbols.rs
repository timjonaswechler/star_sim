//! Symbol definitions for name pattern generation.
//!
//! This module contains the symbol mappings used in name generation patterns.
//! Each symbol represents a set of possible character combinations that can be
//! used in procedural name generation.
//!
//! Symbol maps are now defined using type-safe definitions to ensure all
//! required symbols (s, v, V, c, B, C) are always present.

use super::symbol_types::*;

// Create type-safe symbol maps using the macro
create_symbol_map!(SYMBOL_MAP, StandardSymbols);
create_symbol_map!(DARK_SYMBOL_MAP, DarkSymbols);
create_symbol_map!(BRIGHT_SYMBOL_MAP, BrightSymbols);
create_symbol_map!(EXOTIC_SYMBOL_MAP, ExoticSymbols);