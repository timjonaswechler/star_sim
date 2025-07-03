//! Type-safe symbol map definitions.
//!
//! This module provides a type-safe approach to defining symbol maps,
//! ensuring that all required symbols (s, v, V, c, B, C) are always defined.

use std::collections::HashMap;

/// Trait for type-safe symbol map definitions
/// 
/// All symbol maps must implement this trait to ensure they define
/// all required symbols used by the pattern system.
pub trait SymbolMapDefinition {
    /// Simple syllables - used for `<s>`
    fn syllables() -> Vec<&'static str>;
    
    /// Simple vowels - used for `<v>`
    fn simple_vowels() -> Vec<&'static str>;
    
    /// Complex vowels and diphthongs - used for `<V>`
    fn complex_vowels() -> Vec<&'static str>;
    
    /// Simple consonants - used for `<c>`
    fn simple_consonants() -> Vec<&'static str>;
    
    /// Beginning consonant clusters - used for `<B>`
    fn beginning_clusters() -> Vec<&'static str>;
    
    /// Ending consonant clusters - used for `<C>`
    fn ending_clusters() -> Vec<&'static str>;
    
    /// Build the complete symbol map
    fn build_map() -> HashMap<&'static str, Vec<&'static str>> {
        let mut map = HashMap::new();
        map.insert("s", Self::syllables());
        map.insert("v", Self::simple_vowels());
        map.insert("V", Self::complex_vowels());
        map.insert("c", Self::simple_consonants());
        map.insert("B", Self::beginning_clusters());
        map.insert("C", Self::ending_clusters());
        map
    }
}

/// Standard symbol map definition
pub struct StandardSymbols;

impl SymbolMapDefinition for StandardSymbols {
    fn syllables() -> Vec<&'static str> {
        vec![
            "ach", "ack", "ad", "age", "ald", "ale", "an", "ang", "ar", "ard", "as", "ash",
            "at", "ath", "augh", "aw", "ban", "bel", "bur", "cer", "cha", "che", "dan", "dar",
            "del", "den", "dra", "dyn", "ech", "eld", "elm", "em", "en", "end", "eng", "enth",
            "er", "ess", "est", "et", "gar", "gha", "hat", "hin", "hon", "ia", "ight", "ild",
            "im", "ina", "ine", "ing", "ir", "is", "iss", "it", "kal", "kel", "kim", "kin",
            "ler", "lor", "lye", "mor", "mos", "nal", "ny", "nys", "old", "om", "on", "or",
            "orm", "os", "ough", "per", "pol", "qua", "que", "rad", "rak", "ran", "ray", "ril",
            "ris", "rod", "roth", "ryn", "sam", "say", "ser", "shy", "skel", "sul", "tai",
            "tan", "tas", "ther", "tia", "tin", "ton", "tor", "tur", "um", "und", "unt", "urn",
            "usk", "ust", "ver", "ves", "vor", "war", "wor", "yer",
        ]
    }
    
    fn simple_vowels() -> Vec<&'static str> {
        vec!["a", "e", "i", "o", "u", "y"]
    }
    
    fn complex_vowels() -> Vec<&'static str> {
        vec![
            "a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ay", "ea", "ee", "ei", "eu", "ey",
            "ia", "ie", "oe", "oi", "oo", "ou", "ui",
        ]
    }
    
    fn simple_consonants() -> Vec<&'static str> {
        vec![
            "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t",
            "v", "w", "x", "y", "z",
        ]
    }
    
    fn beginning_clusters() -> Vec<&'static str> {
        vec![
            "b", "bl", "br", "c", "ch", "chr", "cl", "cr", "d", "dr", "f", "g", "h", "j", "k",
            "l", "ll", "m", "n", "p", "ph", "qu", "r", "rh", "s", "sch", "sh", "sl", "sm",
            "sn", "st", "str", "sw", "t", "th", "thr", "tr", "v", "w", "wh", "y", "z", "zh",
        ]
    }
    
    fn ending_clusters() -> Vec<&'static str> {
        vec![
            "b", "c", "ch", "ck", "d", "f", "g", "gh", "h", "k", "l", "ld", "ll", "lt", "m",
            "n", "nd", "nn", "nt", "p", "ph", "q", "r", "rd", "rr", "rt", "s", "sh", "ss",
            "st", "t", "th", "v", "w", "y", "z",
        ]
    }
}

/// Dark/mysterious symbol map definition
pub struct DarkSymbols;

impl SymbolMapDefinition for DarkSymbols {
    fn syllables() -> Vec<&'static str> {
        vec![
            "ach", "ard", "ash", "ath", "dar", "dra", "dyn", "eld", "gar", "gha",
            "grim", "kra", "mor", "mos", "nal", "orth", "rak", "roth", "skel", "sul",
            "tar", "thor", "tor", "tur", "urn", "usk", "vor", "war", "wor", "yer",
            "zar", "zul", "goth", "khar", "morg", "vash", "drak", "thul", "narg",
        ]
    }
    
    fn simple_vowels() -> Vec<&'static str> {
        vec!["a", "o", "u"]
    }
    
    fn complex_vowels() -> Vec<&'static str> {
        vec!["a", "o", "u", "au", "ou", "oo", "ar", "or", "ur"]
    }
    
    fn simple_consonants() -> Vec<&'static str> {
        vec!["k", "g", "r", "th", "gh", "z", "x", "v", "w", "j", "q"]
    }
    
    fn beginning_clusters() -> Vec<&'static str> {
        vec!["kr", "gr", "dr", "th", "gh", "sk", "st", "str", "shr", "zh", "kh"]
    }
    
    fn ending_clusters() -> Vec<&'static str> {
        vec!["k", "g", "r", "th", "gh", "ck", "rk", "ng", "rt", "rd", "rn", "rm"]
    }
}

/// Bright/cheerful symbol map definition
pub struct BrightSymbols;

impl SymbolMapDefinition for BrightSymbols {
    fn syllables() -> Vec<&'static str> {
        vec![
            "al", "an", "ar", "bel", "cel", "del", "el", "em", "en", "er", "est",
            "il", "in", "ir", "is", "lar", "ler", "lin", "lir", "lor", "lya",
            "mel", "ner", "ray", "ren", "ril", "rin", "ser", "tal", "tel", "tin",
            "vel", "ver", "yas", "yel", "yen", "yer", "aer", "ael", "lyra",
        ]
    }
    
    fn simple_vowels() -> Vec<&'static str> {
        vec!["e", "i", "a", "y"]
    }
    
    fn complex_vowels() -> Vec<&'static str> {
        vec!["e", "i", "a", "y", "ae", "ai", "ay", "ea", "ee", "ei", "ey", "ia", "ie"]
    }
    
    fn simple_consonants() -> Vec<&'static str> {
        vec!["l", "r", "n", "s", "t", "d", "f", "h", "m", "p", "b"]
    }
    
    fn beginning_clusters() -> Vec<&'static str> {
        vec!["fl", "fr", "sl", "sm", "sn", "sw", "bl", "br", "cl", "cr", "dr", "tr", "pr", "pl"]
    }
    
    fn ending_clusters() -> Vec<&'static str> {
        vec!["l", "r", "n", "s", "t", "d", "ll", "nn", "ss", "st", "nt", "nd", "rs", "ls"]
    }
}

/// Exotic/alien symbol map definition
pub struct ExoticSymbols;

impl SymbolMapDefinition for ExoticSymbols {
    fn syllables() -> Vec<&'static str> {
        vec![
            "azh", "bzh", "czh", "qyx", "xyr", "zyx", "vyx", "wyx", "yxz", "zxr",
            "quas", "thyx", "xyph", "zyth", "vash", "xoth", "yith", "zeph", "quix",
            "thrix", "xylos", "zynth", "vorth", "xalm", "yeph", "zulm", "qorth",
            "thyxa", "xynth", "zorth", "vylm", "xeph", "yrix", "zulth", "qynx",
        ]
    }
    
    fn simple_vowels() -> Vec<&'static str> {
        vec!["a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ey", "oy", "uy"]
    }
    
    fn complex_vowels() -> Vec<&'static str> {
        vec![
            "a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ay", "ea", "ee", "ei", "eu", "ey",
            "ia", "ie", "oe", "oi", "oo", "ou", "ui", "uy", "yx", "yae", "yoi"
        ]
    }
    
    fn simple_consonants() -> Vec<&'static str> {
        vec!["x", "z", "q", "v", "w", "j", "k", "g", "f", "h", "p", "b", "c", "d", "l", "m", "n", "r", "s", "t", "y"]
    }
    
    fn beginning_clusters() -> Vec<&'static str> {
        vec![
            "xr", "zr", "qr", "vr", "wr", "jr", "xy", "zy", "qy", "vy", "wy", "jy",
            "xth", "zth", "qth", "vth", "wth", "jth", "psy", "xyl", "zyl"
        ]
    }
    
    fn ending_clusters() -> Vec<&'static str> {
        vec![
            "x", "z", "q", "v", "w", "j", "xy", "zy", "qy", "vy", "wy", "jy",
            "xth", "zth", "qth", "vth", "wth", "jth", "nx", "nz", "nq", "nv", "nw", "nj"
        ]
    }
}

/// Macro to create a lazy static symbol map from a type
#[macro_export]
macro_rules! create_symbol_map {
    ($name:ident, $type:ty) => {
        lazy_static::lazy_static! {
            pub static ref $name: std::collections::HashMap<&'static str, Vec<&'static str>> = {
                <$type>::build_map()
            };
        }
    };
}

pub use create_symbol_map;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_symbols_defined() {
        let required_symbols = ["s", "v", "V", "c", "B", "C"];
        
        // Test standard symbols
        let standard_map = StandardSymbols::build_map();
        for symbol in &required_symbols {
            assert!(standard_map.contains_key(symbol), "Standard map missing symbol: {}", symbol);
            assert!(!standard_map[symbol].is_empty(), "Standard map has empty symbol: {}", symbol);
        }
        
        // Test dark symbols
        let dark_map = DarkSymbols::build_map();
        for symbol in &required_symbols {
            assert!(dark_map.contains_key(symbol), "Dark map missing symbol: {}", symbol);
            assert!(!dark_map[symbol].is_empty(), "Dark map has empty symbol: {}", symbol);
        }
        
        // Test bright symbols
        let bright_map = BrightSymbols::build_map();
        for symbol in &required_symbols {
            assert!(bright_map.contains_key(symbol), "Bright map missing symbol: {}", symbol);
            assert!(!bright_map[symbol].is_empty(), "Bright map has empty symbol: {}", symbol);
        }
        
        // Test exotic symbols
        let exotic_map = ExoticSymbols::build_map();
        for symbol in &required_symbols {
            assert!(exotic_map.contains_key(symbol), "Exotic map missing symbol: {}", symbol);
            assert!(!exotic_map[symbol].is_empty(), "Exotic map has empty symbol: {}", symbol);
        }
    }
    
    #[test]
    fn test_symbol_map_differences() {
        let standard_map = StandardSymbols::build_map();
        let dark_map = DarkSymbols::build_map();
        let bright_map = BrightSymbols::build_map();
        
        // Maps should be different
        assert_ne!(standard_map["v"], dark_map["v"]); // Dark has fewer vowels
        assert_ne!(standard_map["v"], bright_map["v"]); // Bright has different vowels
        assert_ne!(dark_map["v"], bright_map["v"]); // Dark and bright are different
        
        // Dark should prefer dark sounds
        assert!(dark_map["v"].contains(&"a"));
        assert!(dark_map["v"].contains(&"o"));
        assert!(dark_map["v"].contains(&"u"));
        
        // Bright should prefer bright sounds
        assert!(bright_map["v"].contains(&"e"));
        assert!(bright_map["v"].contains(&"i"));
        assert!(bright_map["v"].contains(&"y"));
    }
    
    #[test]
    fn test_macro_usage() {
        // Test that the macro creates valid symbol maps
        create_symbol_map!(TEST_STANDARD, StandardSymbols);
        create_symbol_map!(TEST_DARK, DarkSymbols);
        
        assert_eq!(TEST_STANDARD.len(), 6);
        assert_eq!(TEST_DARK.len(), 6);
        
        assert!(TEST_STANDARD.contains_key("s"));
        assert!(TEST_DARK.contains_key("s"));
    }
}