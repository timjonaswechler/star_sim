use std::collections::HashMap;

use lazy_static::lazy_static;
use rand::Rng;
// Middle Earth
const MIDDLE_EARTH: &'static str = "(bil|bal|ban|hil|ham|hal|hol|hob|wil|me|or|ol|od|gor|for|fos|tol|ar|fin|ere|leo|vi|bi|bren|thor)(|go|orbis|apol|adur|mos|ri|i|na|ole|n)(|tur|axia|and|bo|gil|bin|bras|las|mac|grim|wise|l|lo|fo|co|ra|via|da|ne|ta|y|wen|thiel|phin|dir|dor|tor|rod|on|rdo|dis)";

// Japanese Names (Constrained)
const JAPANESE_NAMES_CONSTRAINED: &'static str = "(aka|aki|bashi|gawa|kawa|furu|fuku|fuji|hana|hara|haru|hashi|hira|hon|hoshi|ichi|iwa|kami|kawa|ki|kita|kuchi|kuro|marui|matsu|miya|mori|moto|mura|nabe|naka|nishi|no|da|ta|o|oo|oka|saka|saki|sawa|shita|shima|i|suzu|taka|take|to|toku|toyo|ue|wa|wara|wata|yama|yoshi|kei|ko|zawa|zen|sen|ao|gin|kin|ken|shiro|zaki|yuki|asa)(||||||||||bashi|gawa|kawa|furu|fuku|fuji|hana|hara|haru|hashi|hira|hon|hoshi|chi|wa|ka|kami|kawa|ki|kita|kuchi|kuro|marui|matsu|miya|mori|moto|mura|nabe|naka|nishi|no|da|ta|o|oo|oka|saka|saki|sawa|shita|shima|suzu|taka|take|to|toku|toyo|ue|wa|wara|wata|yama|yoshi|kei|ko|zawa|zen|sen|ao|gin|kin|ken|shiro|zaki|yuki|sa)";

// Japanese Names (Diverse)
const JAPANESE_NAMES_DIVERSE: &'static str = "(a|i|u|e|o|||||)(ka|ki|ki|ku|ku|ke|ke|ko|ko|sa|sa|sa|shi|shi|shi|su|su|se|so|ta|ta|chi|chi|tsu|te|to|na|ni|ni|nu|nu|ne|no|no|ha|hi|fu|fu|he|ho|ma|ma|ma|mi|mi|mi|mu|mu|mu|mu|me|mo|mo|mo|ya|yu|yu|yu|yo|ra|ra|ra|ri|ru|ru|ru|re|ro|ro|ro|wa|wa|wa|wa|wo|wo)(ka|ki|ki|ku|ku|ke|ke|ko|ko|sa|sa|sa|shi|shi|shi|su|su|se|so|ta|ta|chi|chi|tsu|te|to|na|ni|ni|nu|nu|ne|no|no|ha|hi|fu|fu|he|ho|ma|ma|ma|mi|mi|mi|mu|mu|mu|mu|me|mo|mo|mo|ya|yu|yu|yu|yo|ra|ra|ra|ri|ru|ru|ru|re|ro|ro|ro|wa|wa|wa|wa|wo|wo)(|(ka|ki|ki|ku|ku|ke|ke|ko|ko|sa|sa|sa|shi|shi|shi|su|su|se|so|ta|ta|chi|chi|tsu|te|to|na|ni|ni|nu|nu|ne|no|no|ha|hi|fu|fu|he|ho|ma|ma|ma|mi|mi|mi|mu|mu|mu|mu|me|mo|mo|mo|ya|yu|yu|yu|yo|ra|ra|ra|ri|ru|ru|ru|re|ro|ro|ro|wa|wa|wa|wa|wo|wo)|(ka|ki|ki|ku|ku|ke|ke|ko|ko|sa|sa|sa|shi|shi|shi|su|su|se|so|ta|ta|chi|chi|tsu|te|to|na|ni|ni|nu|nu|ne|no|no|ha|hi|fu|fu|he|ho|ma|ma|ma|mi|mi|mi|mu|mu|mu|mu|me|mo|mo|mo|ya|yu|yu|yu|yo|ra|ra|ra|ri|ru|ru|ru|re|ro|ro|ro|wa|wa|wa|wa|wo|wo)(|(ka|ki|ki|ku|ku|ke|ke|ko|ko|sa|sa|sa|shi|shi|shi|su|su|se|so|ta|ta|chi|chi|tsu|te|to|na|ni|ni|nu|nu|ne|no|no|ha|hi|fu|fu|he|ho|ma|ma|ma|mi|mi|mi|mu|mu|mu|mu|me|mo|mo|mo|ya|yu|yu|yu|yo|ra|ra|ra|ri|ru|ru|ru|re|ro|ro|ro|wa|wa|wa|wa|wo|wo)))(|||n)";

// Chinese Names
const CHINESE_NAMES: &'static str = "(zh|x|q|sh|h)(ao|ian|uo|ou|ia)(|(l|w|c|p|b|m)(ao|ian|uo|ou|ia)(|n)|-(l|w|c|p|b|m)(ao|ian|uo|ou|ia)(|(d|j|q|l)(a|ai|iu|ao|i)))";

// Greek Names
const GREEK_NAMES: &'static str = "<s<v|V>(tia)|s<v|V>(os)|B<v|V>c(ios)|B<v|Vc|C>v(ios|os)>";

// Old Latin Place Names
const OLD_LATIN_PLACE_NAMES: &'static str = "sv(nia|lia|cia|sia)";

// Dragons (Pern)
const DRAGONS_PERN: &'static str = "<<s|ss>|<VC|vC|B|BVs|Vs>v|V|v|<v(l|n|r)|vc>>(th)";

// Fantasy (Vowels, R, etc.)
const FANTASY_VOWELS_R: &'static str = "(|(<B>|s|h|ty|ph|r))(i|ae|ya|ae|eu|ia|i|eo|ai|a)(lo|la|sri|da|dai|the|sty|lae|due|li|lly|ri|na|ral|sur|rith)(|(su|nu|sti|llo|ria|))(|(n|ra|p|m|lis|cal|deu|dil|suir|phos|ru|dru|rin|raap|rgue))";

// Fantasy (S, A, etc.)
const FANTASY_S_A: &'static str = "(cham|chan|jisk|lis|frich|isk|lass|mind|sond|sund|ass|chad|lirt|und|mar|lis|il|<BVC>)(jask|ast|ista|adar|irra|im|ossa|assa|osia|ilsa|<vCv>)(|(an|ya|la|sta|sda|sya|st|nya))";

// Fantasy (H, L, etc.)
const FANTASY_H_L: &'static str = "(ch|ch't|sh|cal|val|ell|har|shar|shal|rel|laen|ral|jh't|alr|ch|ch't|av)(|(is|al|ow|ish|ul|el|ar|iel))(aren|aeish|aith|even|adur|ulash|alith|atar|aia|erin|aera|ael|ira|iel|ahur|ishul)";

// Fantasy (N, L, etc.)
const FANTASY_N_L: &'static str = "(ethr|qil|mal|er|eal|far|fil|fir|ing|ind|il|lam|quel|quar|quan|qar|pal|mal|yar|um|ard|enn|ey)(|(<vc>|on|us|un|ar|as|en|ir|ur|at|ol|al|an))(uard|wen|arn|on|il|ie|on|iel|rion|rian|an|ista|rion|rian|cil|mol|yon)";

// Fantasy (K, N, etc.)
const FANTASY_K_N: &'static str = "(taith|kach|chak|kank|kjar|rak|kan|kaj|tach|rskal|kjol|jok|jor|jad|kot|kon|knir|kror|kol|tul|rhaok|rhak|krol|jan|kag|ryr)(<vc>|in|or|an|ar|och|un|mar|yk|ja|arn|ir|ros|ror)(|(mund|ard|arn|karr|chim|kos|rir|arl|kni|var|an|in|ir|a|i|as))";

// Fantasy (J, G, Z, etc.)
const FANTASY_J_G_Z: &'static str = "(aj|ch|etz|etzl|tz|kal|gahn|kab|aj|izl|ts|jaj|lan|kach|chaj|qaq|jol|ix|az|biq|nam)(|(<vc>|aw|al|yes|il|ay|en|tom||oj|im|ol|aj|an|as))(aj|am|al|aqa|ende|elja|ich|ak|ix|in|ak|al|il|ek|ij|os|al|im)";

// Fantasy (K, J, Y, etc.)
const FANTASY_K_J_Y: &'static str = "(yi|shu|a|be|na|chi|cha|cho|ksa|yi|shu)(th|dd|jj|sh|rr|mk|n|rk|y|jj|th)(us|ash|eni|akra|nai|ral|ect|are|el|urru|aja|al|uz|ict|arja|ichi|ural|iru|aki|esh)";

// Fantasy (S, E, etc.)
const FANTASY_S_E: &'static str = "(syth|sith|srr|sen|yth|ssen|then|fen|ssth|kel|syn|est|bess|inth|nen|tin|cor|sv|iss|ith|sen|slar|ssil|sthen|svis|s|ss|s|ss)(|(tys|eus|yn|of|es|en|ath|elth|al|ell|ka|ith|yrrl|is|isl|yr|ast|iy))(us|yn|en|ens|ra|rg|le|en|ith|ast|zon|in|yn|ys)";

lazy_static! {
    // SYMBOL_MAP ist jetzt eine globale, statische Referenz auf eine HashMap.
    // Sie wird nur einmal erstellt, wenn zum ersten Mal darauf zugegriffen wird.
    pub static ref SYMBOL_MAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("s", vec!["ach", "ack", "ad", "age", "ald", "ale", "an", "ang", "ar", "ard",
            "as", "ash", "at", "ath", "augh", "aw", "ban", "bel", "bur", "cer",
            "cha", "che", "dan", "dar", "del", "den", "dra", "dyn", "ech", "eld",
            "elm", "em", "en", "end", "eng", "enth", "er", "ess", "est", "et",
            "gar", "gha", "hat", "hin", "hon", "ia", "ight", "ild", "im", "ina",
            "ine", "ing", "ir", "is", "iss", "it", "kal", "kel", "kim", "kin",
            "ler", "lor", "lye", "mor", "mos", "nal", "ny", "nys", "old", "om",
            "on", "or", "orm", "os", "ough", "per", "pol", "qua", "que", "rad",
            "rak", "ran", "ray", "ril", "ris", "rod", "roth", "ryn", "sam",
            "say", "ser", "shy", "skel", "sul", "tai", "tan", "tas", "ther",
            "tia", "tin", "ton", "tor", "tur", "um", "und", "unt", "urn", "usk",
            "ust", "ver", "ves", "vor", "war", "wor", "yer"]);
        m.insert("v", vec!["a", "e", "i", "o", "u", "y"]);
        m.insert("V", vec!["a", "e", "i", "o", "u", "y", "ae", "ai", "au", "ay", "ea", "ee",
            "ei", "eu", "ey", "ia", "ie", "oe", "oi", "oo", "ou", "ui"]);
        m.insert("c", vec!["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r",
            "s", "t", "v", "w", "x", "y", "z"]);
        m.insert("B", vec!["b", "bl", "br", "c", "ch", "chr", "cl", "cr", "d", "dr", "f", "g",
            "h", "j", "k", "l", "ll", "m", "n", "p", "ph", "qu", "r", "rh", "s",
            "sch", "sh", "sl", "sm", "sn", "st", "str", "sw", "t", "th", "thr",
            "tr", "v", "w", "wh", "y", "z", "zh"]);
        m.insert("C", vec!["b", "c", "ch", "ck", "d", "f", "g", "gh", "h", "k", "l", "ld", "ll",
            "lt", "m", "n", "nd", "nn", "nt", "p", "ph", "q", "r", "rd", "rr",
            "rt", "s", "sh", "ss", "st", "t", "th", "v", "w", "y", "z"]);
        m.insert("D", vec!["b", "bl", "br", "cl", "d", "f", "fl", "fr", "g", "gh", "gl", "gr",
            "h", "j", "k", "kl", "m", "n", "p", "th", "w"]);
        m
    };
}

impl NameBuilder {
    /// Create a new NameBuilder from a custom pattern string.
    pub fn new(pattern: &str) -> Result<Self, String> {
        let pattern = Pattern::parse(pattern, &SYMBOL_MAP, false)?;
        Ok(Self { pattern })
    }

    // === Stellar System Specific Patterns ===

    /// Generate star names with Greek letter designations
    pub fn star_names() -> Self {
        Self {
            pattern: Pattern::parse("<!s<v|V>", &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate planet names with designation suffixes
    pub fn planet_names() -> Self {
        Self {
            pattern: Pattern::parse("<!svc>", &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate system names (clusters, nebulae, etc.)
    pub fn system_names() -> Self {
        Self {
            pattern: Pattern::parse(
                "<!svc>( system| cluster| nebula| sector| quadrant)",
                &SYMBOL_MAP,
                false,
            )
            .unwrap(),
        }
    }

    /// Generate simple fantasy names (short and pronounceable)
    pub fn simple_fantasy() -> Self {
        Self {
            pattern: Pattern::parse("<!svc>", &SYMBOL_MAP, false).unwrap(),
        }
    }
    /// Create a new NameBuilder from a custom pattern string with collapse triples enabled.
    pub fn new_collapsed(pattern: &str) -> Result<Self, String> {
        let pattern = Pattern::parse(pattern, &SYMBOL_MAP, true)?;
        Ok(Self { pattern })
    }

    /// Generate a name using the configured pattern.
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        self.pattern.generate(rng)
    }

    // === Predefined Pattern Methods ===

    /// Generate Middle Earth style names (e.g., "Gandalf", "Aragorn")
    pub fn middle_earth() -> Self {
        Self {
            pattern: Pattern::parse(MIDDLE_EARTH, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Japanese names (constrained syllables)
    pub fn japanese_constrained() -> Self {
        Self {
            pattern: Pattern::parse(JAPANESE_NAMES_CONSTRAINED, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Japanese names (diverse syllables)
    pub fn japanese_diverse() -> Self {
        Self {
            pattern: Pattern::parse(JAPANESE_NAMES_DIVERSE, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Chinese style names
    pub fn chinese() -> Self {
        Self {
            pattern: Pattern::parse(CHINESE_NAMES, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Greek style names
    pub fn greek() -> Self {
        Self {
            pattern: Pattern::parse(GREEK_NAMES, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Old Latin place names
    pub fn old_latin_places() -> Self {
        Self {
            pattern: Pattern::parse(OLD_LATIN_PLACE_NAMES, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate Pern dragon names
    pub fn dragons_pern() -> Self {
        Self {
            pattern: Pattern::parse(DRAGONS_PERN, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate dragon rider names
    pub fn dragon_riders() -> Self {
        Self {
            pattern: Pattern::parse(DRAGON_RIDERS, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with vowels and R sounds
    pub fn fantasy_vowels_r() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_VOWELS_R, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with S and A sounds
    pub fn fantasy_s_a() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_S_A, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with H and L sounds
    pub fn fantasy_h_l() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_H_L, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with N and L sounds
    pub fn fantasy_n_l() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_N_L, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with K and N sounds
    pub fn fantasy_k_n() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_K_N, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with J, G, and Z sounds
    pub fn fantasy_j_g_z() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_J_G_Z, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with K, J, and Y sounds
    pub fn fantasy_k_j_y() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_K_J_Y, &SYMBOL_MAP, false).unwrap(),
        }
    }

    /// Generate fantasy names with S and E sounds
    pub fn fantasy_s_e() -> Self {
        Self {
            pattern: Pattern::parse(FANTASY_S_E, &SYMBOL_MAP, false).unwrap(),
        }
    }
}

/// Convenience function to generate a name using a pattern string.
/// This is a simplified API for common use cases.
pub fn generate_name(pattern: &str, rng: &mut impl Rng) -> Result<String, String> {
    let parsed_pattern = Pattern::parse(pattern, &SYMBOL_MAP, false)?;
    Ok(parsed_pattern.generate(rng))
}

/// Convenience function to generate a name with collapse triples enabled.
pub fn generate_name_collapsed(pattern: &str, rng: &mut impl Rng) -> Result<String, String> {
    let parsed_pattern = Pattern::parse(pattern, &SYMBOL_MAP, true)?;
    Ok(parsed_pattern.generate(rng))
}

impl GeneratorNode {
    /// Generiert rekursiv einen String aus dem AST, beginnend bei diesem Knoten.
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        match self {
            // Basis-Fall: Ein Literal gibt einfach seinen Wert zurück.
            GeneratorNode::Literal(s) => s.clone(),

            // Eine Sequenz generiert jedes Kind der Reihe nach und fügt die Ergebnisse zusammen.
            GeneratorNode::Sequence(nodes) => nodes.iter().map(|node| node.generate(rng)).collect(),

            // Ein Random-Knoten wählt ein zufälliges Kind aus und generiert nur dieses.
            GeneratorNode::Random(nodes) => {
                if nodes.is_empty() {
                    "".to_string()
                } else {
                    let index = rng.gen_range(0..nodes.len());
                    nodes[index].generate(rng)
                }
            }

            // Wrapper: Generiere zuerst den inneren Knoten, dann transformiere das Ergebnis.
            GeneratorNode::Capitalizer(node) => {
                let s = node.generate(rng);
                if s.is_empty() {
                    return s;
                }
                let mut chars = s.chars();
                // .unwrap() ist hier sicher, da wir auf is_empty() geprüft haben.
                let first = chars.next().unwrap();
                let rest = chars.as_str().to_lowercase();
                format!("{}{}", first.to_uppercase(), rest)
            }

            GeneratorNode::Reverser(node) => {
                let s = node.generate(rng);
                // Die korrekte Art, einen String in Rust umzudrehen (funktioniert mit Unicode).
                s.chars().rev().collect::<String>()
            }

            GeneratorNode::Collapser(node) => {
                let s = node.generate(rng);
                let mut out = String::with_capacity(s.len());
                let mut count = 0;
                let mut last_char = '\0'; // Ein Zeichen, das nicht im String vorkommt.

                for current_char in s.chars() {
                    if current_char == last_char {
                        count += 1;
                    } else {
                        count = 0;
                    }

                    // Bestimme, wie viele Wiederholungen erlaubt sind.
                    let max_count = match current_char {
                        'a' | 'h' | 'i' | 'j' | 'q' | 'u' | 'v' | 'w' | 'x' | 'y' => 1,
                        _ => 2,
                    };

                    if count < max_count {
                        out.push(current_char);
                    }

                    last_char = current_char;
                }
                out
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Wrapper {
    Capitalizer,
    Reverser,
}
#[derive(Debug, Clone)]
pub enum GroupType {
    Symbol,
    Literal,
}

#[derive(Debug, Clone)]
pub enum GeneratorNode {
    Literal(String),
    Sequence(Vec<GeneratorNode>),
    Random(Vec<GeneratorNode>),
    Reverser(Box<GeneratorNode>),
    Capitalizer(Box<GeneratorNode>),
    Collapser(Box<GeneratorNode>),
}

#[derive(Debug, Clone)]
pub struct GroupBuilder {
    group_type: GroupType,
    wrappers: Vec<Wrapper>,
    set: Vec<Vec<GeneratorNode>>,
}

impl GroupBuilder {
    /// Erstellt einen neuen, leeren GroupBuilder.
    pub fn new(group_type: GroupType) -> Self {
        Self {
            group_type,
            wrappers: Vec::new(),
            // Wir starten immer mit einer Sequenz.
            // Weitere werden durch `split` hinzugefügt.
            set: vec![vec![]],
        }
    }

    /// Fügt einen Wrapper hinzu, der auf den NÄCHSTEN hinzugefügten Generator
    /// angewendet wird. Entspricht '!' (Capitalizer) und '~' (Reverser).
    ///
    /// TypeScript-Äquivalent: `wrap(type)`
    pub fn wrap(&mut self, wrapper: Wrapper) {
        self.wrappers.push(wrapper);
    }

    /// Erstellt eine neue alternative Sequenz. Entspricht dem '|'-Zeichen.
    ///
    /// TypeScript-Äquivalent: `split()`
    pub fn split(&mut self) {
        // Fügt einfach eine neue, leere Sequenz zu unserem Set von Auswahlmöglichkeiten hinzu.
        self.set.push(Vec::new());
    }

    /// Fügt einen Generator-Knoten zur aktuellen Sequenz hinzu und wendet
    /// alle zuvor gesammelten Wrapper an.
    fn add_node(&mut self, mut node: GeneratorNode) {
        // Die Wrapper werden in umgekehrter Reihenfolge angewendet, so als
        // würden wir sie von einem Stapel nehmen (was `wrappers.pop()` im TS tut).
        // `drain(..)` leert den Vektor und gibt einen Iterator zurück.
        for wrapper in self.wrappers.drain(..).rev() {
            // Wir "wickeln" den bestehenden Knoten in einen neuen Knoten ein.
            node = match wrapper {
                Wrapper::Capitalizer => GeneratorNode::Capitalizer(Box::new(node)),
                Wrapper::Reverser => GeneratorNode::Reverser(Box::new(node)),
            };
        }

        // Füge den finalen (ggf. gewrappten) Knoten zur letzten (aktuellen) Sequenz hinzu.
        // `last_mut` gibt eine veränderbare Referenz auf das letzte Element.
        if let Some(current_sequence) = self.set.last_mut() {
            current_sequence.push(node);
        }
    }

    /// Dies ist die Haupt-Add-Methode, die die Logik für `GroupSymbol` und `GroupLiteral`
    /// aus dem TypeScript-Code vereint.
    ///
    /// TypeScript-Äquivalent: `add(a: string)` in `GroupSymbol` und `GroupLiteral`.
    pub fn add_char(&mut self, ch: char, symbol_map: &HashMap<&'static str, Vec<&'static str>>) {
        let node_to_add = match self.group_type {
            // Wenn wir in einer ( ... )-Gruppe sind, wird jedes Zeichen zu einem Literal.
            GroupType::Literal => GeneratorNode::Literal(ch.to_string()),

            // Wenn wir in einer < ... >-Gruppe sind, wird das Zeichen als Symbol behandelt.
            GroupType::Symbol => {
                let symbol_key = ch.to_string();
                // Versuche, das Symbol in der Map zu finden.
                if let Some(expansions) = symbol_map.get(symbol_key.as_str()) {
                    // Wenn gefunden, erstelle einen `Random`-Knoten mit allen
                    // möglichen Erweiterungen als `Literal`-Knoten.
                    let choices = expansions
                        .iter()
                        .map(|s| GeneratorNode::Literal(s.to_string()))
                        .collect();
                    GeneratorNode::Random(choices)
                } else {
                    // Wenn das Symbol nicht in der Map existiert, behandeln wir es
                    // einfach als Literal (z.B. bei 'v' in '<...v...>')
                    GeneratorNode::Literal(symbol_key)
                }
            }
        };

        // Rufe die interne Methode auf, um den erstellten Knoten hinzuzufügen
        // und eventuelle Wrapper anzuwenden.
        self.add_node(node_to_add);
    }

    /// Finalisiert die Gruppe und gibt einen einzigen, fertigen `GeneratorNode` zurück.
    ///
    /// TypeScript-Äquivalent: `produce()`
    pub fn produce(self) -> GeneratorNode {
        // Wandle jede innere `Vec<GeneratorNode>` (unsere Sequenzen) in einen
        // expliziten `GeneratorNode::Sequence`-Knoten um.
        let choices: Vec<GeneratorNode> =
            self.set.into_iter().map(GeneratorNode::Sequence).collect();

        match choices.len() {
            // Sollte dank unseres Konstruktors nicht passieren, aber sicher ist sicher.
            0 => GeneratorNode::Literal("".to_string()),
            // Wenn es nur eine Wahl gab (kein '|' in der Gruppe), brauchen wir keinen
            // `Random`-Knoten. Wir geben die Sequenz direkt zurück.
            1 => choices.into_iter().next().unwrap(),
            // Wenn es mehrere Wahlen gab, werden sie in einen `Random`-Knoten verpackt.
            _ => GeneratorNode::Random(choices),
        }
    }
}

/// Repräsentiert ein vollständig geparstes Namensgenerierungs-Muster.
/// Diese Struktur ist das Ergebnis des Parsens und enthält den Wurzelknoten
/// des Abstract Syntax Tree (AST).
#[derive(Debug, Clone)]
pub struct Pattern {
    root_node: GeneratorNode,
}

impl Pattern {
    /// Startet den Generierungsprozess vom Wurzelknoten des Patterns.
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        self.root_node.generate(rng)
    }
    /// Parst einen Muster-String und erstellt daraus den AST.
    /// Dies ist die Rust-Entsprechung des `Generator`-Konstruktors aus dem TS-Code.
    pub fn parse(
        pattern_str: &str,
        symbol_map: &HashMap<&'static str, Vec<&'static str>>,
        collapse_triples: bool,
    ) -> Result<Self, String> {
        use std::mem;

        // Ein Stapel, um die übergeordneten Gruppen zu speichern, wenn wir in eine
        // neue, verschachtelte Gruppe eintauchen.
        let mut stack: Vec<GroupBuilder> = Vec::new();

        // Die aktuell bearbeitete Gruppe. Die oberste Ebene verhält sich wie eine
        // Symbol-Gruppe (z.B. erlaubt es 's' oder 'v' ohne Klammern).
        let mut top = GroupBuilder::new(GroupType::Symbol);

        // Wir iterieren über jedes Zeichen des Eingabe-Strings.
        for ch in pattern_str.chars() {
            match ch {
                // Beginnt eine Symbol-Gruppe '<...>' oder eine Literal-Gruppe '(...)'
                '<' | '(' => {
                    let new_group_type = if ch == '<' {
                        GroupType::Symbol
                    } else {
                        GroupType::Literal
                    };
                    // Tausche die aktuelle 'top'-Gruppe gegen eine neue, leere aus
                    // und schiebe die alte 'top'-Gruppe auf den Stack.
                    // Das ist ein idiomatischer Weg in Rust, um Ownership-Regeln zu handhaben.
                    let old_top = mem::replace(&mut top, GroupBuilder::new(new_group_type));
                    stack.push(old_top);
                }

                // Beendet eine Symbol-Gruppe '>' oder eine Literal-Gruppe ')'
                '>' | ')' => {
                    // Prüfe auf Fehler: Kann keine Gruppe schließen, wenn keine offen ist.
                    if stack.is_empty() {
                        return Err(format!("Unbalanced brackets: unexpected '{}'", ch));
                    }

                    // Prüfe auf Fehler: Falscher Klammer-Typ
                    if ch == '>' && !matches!(top.group_type, GroupType::Symbol) {
                        return Err("Unexpected '>' in literal group.".to_string());
                    }
                    if ch == ')' && !matches!(top.group_type, GroupType::Literal) {
                        return Err("Unexpected ')' in symbol group.".to_string());
                    }

                    // Finalisiere die aktuelle Gruppe zu einem GeneratorNode.
                    let produced_node = top.produce();

                    // Hole die übergeordnete Gruppe vom Stack und mache sie zur neuen 'top'-Gruppe.
                    // .unwrap() ist hier sicher, da wir `is_empty()` geprüft haben.
                    top = stack.pop().unwrap();

                    // Füge den soeben erstellten Knoten zur wiederhergestellten,
                    // übergeordneten Gruppe hinzu.
                    top.add_node(produced_node);
                }

                // Teilt die aktuelle Gruppe in eine weitere Alternative auf.
                '|' => top.split(),

                // Wrappers: '!' für Capitalizer, '~' für Reverser.
                // Diese funktionieren nur in Symbol-Gruppen '<...>'.
                '!' | '~' => {
                    if matches!(top.group_type, GroupType::Symbol) {
                        let wrapper = if ch == '!' {
                            Wrapper::Capitalizer
                        } else {
                            Wrapper::Reverser
                        };
                        top.wrap(wrapper);
                    } else {
                        // In einer Literal-Gruppe ist '!' oder '~' nur ein normales Zeichen.
                        top.add_char(ch, symbol_map);
                    }
                }

                // Alle anderen Zeichen werden zur aktuellen Gruppe hinzugefügt.
                _ => {
                    top.add_char(ch, symbol_map);
                }
            }
        }

        // Nach der Schleife: Wenn der Stack nicht leer ist, fehlen schließende Klammern.
        if !stack.is_empty() {
            return Err("Missing closing brackets".to_string());
        }

        // Finalisiere die oberste Gruppe, um den finalen AST-Wurzelknoten zu erhalten.
        let mut final_node = top.produce();

        // Wickel optional einen Collapser um das Endergebnis.
        if collapse_triples {
            final_node = GeneratorNode::Collapser(Box::new(final_node));
        }

        // Wenn alles gut ging, gib die neue Pattern-Instanz zurück.
        Ok(Self {
            root_node: final_node,
        })
    }
}

/// Ergonomic name generator builder with predefined patterns and custom pattern support.
///
/// This builder provides convenient methods for generating names using predefined patterns
/// or custom patterns without needing to import pattern constants.
#[derive(Debug, Clone)]
pub struct NameBuilder {
    pattern: Pattern,
}
