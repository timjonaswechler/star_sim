//! Pattern engine for procedural name generation.
//!
//! This module provides the core pattern parsing and generation engine that powers
//! the name generation system. It can parse complex pattern strings with nested
//! groups, alternatives, and transformations.

use std::collections::HashMap;
use rand::Rng;
use rand::distributions::{WeightedIndex, Distribution};
use super::phonetic_rules::PhoneticRules;

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
    pub fn new(group_type: GroupType) -> Self {
        Self {
            group_type,
            wrappers: Vec::new(),
            set: vec![vec![]],
        }
    }

    pub fn wrap(&mut self, wrapper: Wrapper) {
        self.wrappers.push(wrapper);
    }

    pub fn split(&mut self) {
        self.set.push(Vec::new());
    }

    fn add_node(&mut self, mut node: GeneratorNode) {
        for wrapper in self.wrappers.drain(..).rev() {
            node = match wrapper {
                Wrapper::Capitalizer => GeneratorNode::Capitalizer(Box::new(node)),
                Wrapper::Reverser => GeneratorNode::Reverser(Box::new(node)),
            };
        }

        if let Some(current_sequence) = self.set.last_mut() {
            current_sequence.push(node);
        }
    }

    pub fn add_char(&mut self, ch: char, symbol_map: &HashMap<&'static str, Vec<&'static str>>) {
        let node_to_add = match self.group_type {
            GroupType::Literal => GeneratorNode::Literal(ch.to_string()),
            GroupType::Symbol => {
                let symbol_key = ch.to_string();
                if let Some(expansions) = symbol_map.get(symbol_key.as_str()) {
                    let choices = expansions
                        .iter()
                        .map(|s| GeneratorNode::Literal(s.to_string()))
                        .collect();
                    GeneratorNode::Random(choices)
                } else {
                    GeneratorNode::Literal(symbol_key)
                }
            }
        };

        self.add_node(node_to_add);
    }

    pub fn produce(self) -> GeneratorNode {
        let choices: Vec<GeneratorNode> =
            self.set.into_iter().map(GeneratorNode::Sequence).collect();

        match choices.len() {
            0 => GeneratorNode::Literal("".to_string()),
            1 => choices.into_iter().next().unwrap(),
            _ => GeneratorNode::Random(choices),
        }
    }
}

/// Core pattern type that can parse and generate names from pattern strings
///
/// Patterns support various features:
/// - Symbol replacement: `<s>`, `<v>`, `<c>` etc.
/// - Literal groups: `(hello|world)`  
/// - Symbol groups: `<consonant|vowel>`
/// - Wrappers: `!` for capitalization, `~` for reversal
/// - Alternatives: `|` for choice between options
///
/// # Examples
///
/// ```rust
/// use star_sim::utilities::name_generator::pattern::Pattern;
/// use star_sim::utilities::name_generator::symbols::SYMBOL_MAP;
/// use rand::thread_rng;
///
/// let mut rng = thread_rng();
/// let pattern = Pattern::parse("<!s><v><c>", &SYMBOL_MAP, false).unwrap();
/// let name = pattern.generate(&mut rng);
/// ```
#[derive(Debug, Clone)]
pub struct Pattern {
    root_node: GeneratorNode,
}

impl Pattern {
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        self.root_node.generate(rng)
    }
    
    pub fn generate_with_context(&self, rng: &mut impl Rng, context: &mut String, rules: Option<&PhoneticRules>) -> String {
        self.root_node.generate_with_context(rng, context, rules)
    }

    pub fn parse(
        pattern_str: &str,
        symbol_map: &HashMap<&'static str, Vec<&'static str>>,
        collapse_triples: bool,
    ) -> Result<Self, String> {
        use std::mem;

        let mut stack: Vec<GroupBuilder> = Vec::new();
        let mut top = GroupBuilder::new(GroupType::Symbol);

        for ch in pattern_str.chars() {
            match ch {
                '<' | '(' => {
                    let new_group_type = if ch == '<' {
                        GroupType::Symbol
                    } else {
                        GroupType::Literal
                    };
                    let old_top = mem::replace(&mut top, GroupBuilder::new(new_group_type));
                    stack.push(old_top);
                }

                '>' | ')' => {
                    if stack.is_empty() {
                        return Err(format!("Unbalanced brackets: unexpected '{}'", ch));
                    }

                    if ch == '>' && !matches!(top.group_type, GroupType::Symbol) {
                        return Err("Unexpected '>' in literal group.".to_string());
                    }
                    if ch == ')' && !matches!(top.group_type, GroupType::Literal) {
                        return Err("Unexpected ')' in symbol group.".to_string());
                    }

                    let produced_node = top.produce();
                    top = stack.pop().unwrap();
                    top.add_node(produced_node);
                }

                '|' => top.split(),

                '!' | '~' => {
                    if matches!(top.group_type, GroupType::Symbol) {
                        let wrapper = if ch == '!' {
                            Wrapper::Capitalizer
                        } else {
                            Wrapper::Reverser
                        };
                        top.wrap(wrapper);
                    } else {
                        top.add_char(ch, symbol_map);
                    }
                }

                _ => {
                    top.add_char(ch, symbol_map);
                }
            }
        }

        if !stack.is_empty() {
            return Err("Missing closing brackets".to_string());
        }

        let mut final_node = top.produce();

        if collapse_triples {
            final_node = GeneratorNode::Collapser(Box::new(final_node));
        }

        Ok(Self {
            root_node: final_node,
        })
    }
}

impl GeneratorNode {
    pub fn generate(&self, rng: &mut impl Rng) -> String {
        let mut context = String::new();
        self.generate_with_context(rng, &mut context, None)
    }
    
    pub fn generate_with_context(&self, rng: &mut impl Rng, context: &mut String, rules: Option<&PhoneticRules>) -> String {
        match self {
            GeneratorNode::Literal(s) => s.clone(),
            GeneratorNode::Sequence(nodes) => {
                let mut result = String::new();
                for node in nodes {
                    let part = node.generate_with_context(rng, context, rules);
                    result.push_str(&part);
                    context.push_str(&part);
                }
                result
            }
            GeneratorNode::Random(nodes) => {
                if nodes.is_empty() {
                    return "".to_string();
                }
                
                // Use weighted selection if rules are provided
                let chosen_node = if let Some(rules) = rules {
                    self.weighted_choice(nodes, context, rules, rng)
                } else {
                    // Fallback to uniform random selection
                    let index = rng.gen_range(0..nodes.len());
                    &nodes[index]
                };
                
                chosen_node.generate_with_context(rng, context, rules)
            }
            GeneratorNode::Capitalizer(node) => {
                let s = node.generate_with_context(rng, context, rules);
                if s.is_empty() {
                    return s;
                }
                let mut chars = s.chars();
                let first = chars.next().unwrap();
                let rest = chars.as_str().to_lowercase();
                format!("{}{}", first.to_uppercase(), rest)
            }
            GeneratorNode::Reverser(node) => {
                let s = node.generate_with_context(rng, context, rules);
                s.chars().rev().collect::<String>()
            }
            GeneratorNode::Collapser(node) => {
                let s = node.generate_with_context(rng, context, rules);
                let mut out = String::with_capacity(s.len());
                let mut count = 0;
                let mut last_char = '\0';

                for current_char in s.chars() {
                    if current_char == last_char {
                        count += 1;
                    } else {
                        count = 0;
                    }

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
    
    /// Perform weighted selection based on phonetic rules
    fn weighted_choice<'a>(&self, nodes: &'a [GeneratorNode], context: &str, rules: &PhoneticRules, rng: &mut impl Rng) -> &'a GeneratorNode {
        // Preview what each node would generate
        let options: Vec<String> = nodes.iter()
            .map(|node| node.preview_content())
            .collect();
        
        // Calculate weights based on phonetic rules
        let weights: Vec<f32> = options.iter()
            .map(|option| rules.calculate_weight(context, option))
            .collect();
        
        // Ensure we have at least one non-zero weight
        let has_valid_weight = weights.iter().any(|&w| w > 0.0);
        if !has_valid_weight {
            // If all weights are zero, fall back to uniform selection
            let index = rng.gen_range(0..nodes.len());
            return &nodes[index];
        }
        
        // Use weighted selection
        match WeightedIndex::new(&weights) {
            Ok(dist) => {
                let index = dist.sample(rng);
                &nodes[index]
            }
            Err(_) => {
                // Fallback to uniform selection if weighted selection fails
                let index = rng.gen_range(0..nodes.len());
                &nodes[index]
            }
        }
    }
    
    /// Preview the content this node would generate without actually generating it
    fn preview_content(&self) -> String {
        match self {
            GeneratorNode::Literal(s) => s.clone(),
            GeneratorNode::Sequence(nodes) => {
                // For sequences, preview the first few characters
                nodes.iter()
                    .take(1) // Just take the first node for preview
                    .map(|node| node.preview_content())
                    .collect::<Vec<_>>()
                    .join("")
            }
            GeneratorNode::Random(nodes) => {
                // For random nodes, preview the first option
                if nodes.is_empty() {
                    "".to_string()
                } else {
                    nodes[0].preview_content()
                }
            }
            GeneratorNode::Capitalizer(node) => {
                let s = node.preview_content();
                if s.is_empty() {
                    return s;
                }
                let mut chars = s.chars();
                let first = chars.next().unwrap();
                let rest = chars.as_str().to_lowercase();
                format!("{}{}", first.to_uppercase(), rest)
            }
            GeneratorNode::Reverser(node) => {
                let s = node.preview_content();
                s.chars().rev().collect::<String>()
            }
            GeneratorNode::Collapser(node) => {
                node.preview_content() // Preview doesn't apply collapsing
            }
        }
    }
}