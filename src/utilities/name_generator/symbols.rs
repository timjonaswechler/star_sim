//! Symbol definitions for name pattern generation.
//!
//! This module contains the symbol mappings used in name generation patterns.
//! Each symbol represents a set of possible character combinations that can be
//! used in procedural name generation.

use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Symbol mappings for pattern generation
    /// 
    /// Contains mappings from single character symbols to arrays of possible expansions.
    /// Used by the pattern parser to replace symbols like `<s>`, `<v>`, `<c>` with 
    /// appropriate character sequences.
    pub static ref SYMBOL_MAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        
        // Syllables - used for <s>
        m.insert(
            "s",
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
            ],
        );
        
        // Simple vowels - used for <v>
        m.insert("v", vec!["a", "e", "i", "o", "u", "y"]);
        
        // Complex vowels and diphthongs - used for <V>
        m.insert(
            "V",
            vec![
                "a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ay", "ea", "ee", "ei", "eu", "ey",
                "ia", "ie", "oe", "oi", "oo", "ou", "ui",
            ],
        );
        
        // Simple consonants - used for <c>
        m.insert(
            "c",
            vec![
                "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t",
                "v", "w", "x", "y", "z",
            ],
        );
        
        // Beginning consonant clusters - used for <B>
        m.insert(
            "B",
            vec![
                "b", "bl", "br", "c", "ch", "chr", "cl", "cr", "d", "dr", "f", "g", "h", "j", "k",
                "l", "ll", "m", "n", "p", "ph", "qu", "r", "rh", "s", "sch", "sh", "sl", "sm",
                "sn", "st", "str", "sw", "t", "th", "thr", "tr", "v", "w", "wh", "y", "z", "zh",
            ],
        );
        
        // Ending consonant clusters - used for <C>
        m.insert(
            "C",
            vec![
                "b", "c", "ch", "ck", "d", "f", "g", "gh", "h", "k", "l", "ld", "ll", "lt", "m",
                "n", "nd", "nn", "nt", "p", "ph", "q", "r", "rd", "rr", "rt", "s", "sh", "ss",
                "st", "t", "th", "v", "w", "y", "z",
            ],
        );
        
        m
    };
}