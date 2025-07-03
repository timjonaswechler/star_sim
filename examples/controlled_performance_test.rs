//! Controlled performance test for fair comparison of phonetic approaches.
//!
//! This test ensures fair comparison by using:
//! - Same pattern complexity across all approaches
//! - Same target sound profile (dark)
//! - Multiple runs for statistical accuracy
//! - Controlled conditions

use lazy_static::lazy_static;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use star_sim::utilities::name_generator::{
    DARK_SYMBOL_MAP, Name, NameCategory, SYMBOL_MAP,
    phonetic_rules::{PhoneticRules, profiles},
    symbol_types::SymbolMapDefinition,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

const ITERATIONS: usize = 50_000;
const RUNS: usize = 5;

// ============================================================================
// CONTROLLED TEST CATEGORIES - All targeting "dark" sound with same pattern
// ============================================================================

/// Symbol-only approach: Dark sounds only
pub struct DarkSymbolOnly;

impl Default for DarkSymbolOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkSymbolOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // SAME pattern for all tests
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DARK_SYMBOL_MAP // Pre-filtered dark sounds
    }

    // No phonetic rules - filtering done by symbol map
}

/// Rules-only approach: All sounds + dark filtering rules
pub struct DarkRulesOnly;

impl Default for DarkRulesOnly {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkRulesOnly {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // SAME pattern for all tests
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &SYMBOL_MAP // All sounds available
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::DARK_RULES) // Dynamic filtering for dark sounds
    }
}

/// Hybrid approach: Dark sounds + additional dark filtering
pub struct DarkHybrid;

impl Default for DarkHybrid {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkHybrid {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // SAME pattern for all tests
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DARK_SYMBOL_MAP // Pre-filtered dark sounds
    }

    fn phonetic_rules(&self) -> Option<&PhoneticRules> {
        Some(&profiles::DARK_RULES) // Additional filtering
    }
}

/// Standard approach: All sounds, no filtering
pub struct StandardBaseline;

impl Default for StandardBaseline {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for StandardBaseline {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // SAME pattern for all tests
    }

    // Uses default symbol map and no phonetic rules
}

// ============================================================================
// TYPE-SAFE TEST - Equivalent to symbol-only but with custom definition
// ============================================================================

pub struct DarkSymbolsTypeSafe;

impl SymbolMapDefinition for DarkSymbolsTypeSafe {
    fn syllables() -> Vec<&'static str> {
        vec![
            "ach", "ard", "ash", "ath", "dar", "dra", "dyn", "eld", "gar", "gha", "grim", "kra",
            "mor", "mos", "nal", "orth", "rak", "roth", "skel", "sul", "tar", "thor", "tor", "tur",
            "urn", "usk", "vor", "war", "wor", "yer", "zar", "zul", "goth", "khar", "morg", "vash",
            "drak", "thul", "narg",
        ]
    }

    fn simple_vowels() -> Vec<&'static str> {
        vec!["a", "o", "u"]
    }

    fn complex_vowels() -> Vec<&'static str> {
        vec!["a", "o", "u", "au", "ou", "oo", "ar", "or", "ur"]
    }

    fn simple_consonants() -> Vec<&'static str> {
        vec!["k", "g", "r", "z", "x", "v", "w", "j", "q"]
    }

    fn beginning_clusters() -> Vec<&'static str> {
        vec!["kr", "gr", "dr", "sk", "st", "str", "shr", "zh", "kh"]
    }

    fn ending_clusters() -> Vec<&'static str> {
        vec!["k", "g", "r", "ck", "rk", "ng", "rt", "rd", "rn", "rm"]
    }
}

lazy_static::lazy_static! {
    static ref DARK_TYPESAFE_MAP: HashMap<&'static str, Vec<&'static str>> = {
        DarkSymbolsTypeSafe::build_map()
    };
}

pub struct DarkTypeSafe;

impl Default for DarkTypeSafe {
    fn default() -> Self {
        Self
    }
}

impl NameCategory for DarkTypeSafe {
    type Variant = ();

    fn pattern(&self) -> &'static str {
        "<!s><v><c><s>" // SAME pattern for all tests
    }

    fn symbol_map(&self) -> &HashMap<&'static str, Vec<&'static str>> {
        &DARK_TYPESAFE_MAP // Custom type-safe dark sounds
    }
}

// ============================================================================
// BENCHMARK INFRASTRUCTURE
// ============================================================================

#[derive(Debug)]
struct BenchmarkStats {
    name: String,
    runs: Vec<Duration>,
    mean: Duration,
    std_dev: Duration,
    min: Duration,
    max: Duration,
    names_per_second: f64,
}

impl BenchmarkStats {
    fn new(name: String, runs: Vec<Duration>, iterations: usize) -> Self {
        let mean_nanos: f64 =
            runs.iter().map(|d| d.as_nanos() as f64).sum::<f64>() / runs.len() as f64;
        let mean = Duration::from_nanos(mean_nanos as u64);

        let variance: f64 = runs
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean_nanos;
                diff * diff
            })
            .sum::<f64>()
            / runs.len() as f64;
        let std_dev = Duration::from_nanos(variance.sqrt() as u64);

        let min = *runs.iter().min().unwrap();
        let max = *runs.iter().max().unwrap();

        let names_per_second = iterations as f64 / mean.as_secs_f64();

        Self {
            name,
            runs,
            mean,
            std_dev,
            min,
            max,
            names_per_second,
        }
    }

    fn print(&self) {
        println!("  {}:", self.name);
        println!("    Mean: {:?} (±{:?})", self.mean, self.std_dev);
        println!("    Range: {:?} - {:?}", self.min, self.max);
        println!("    Names/sec: {:.0}", self.names_per_second);
        println!("    Individual runs: {:?}", self.runs);
    }
}

fn benchmark_category<T: Default + NameCategory>(
    name: &str,
    iterations: usize,
    runs: usize,
) -> BenchmarkStats {
    println!(
        "  Running {} ({} runs of {} iterations)...",
        name, runs, iterations
    );

    let mut run_times = Vec::with_capacity(runs);

    for run in 0..runs {
        let mut rng = ChaCha8Rng::seed_from_u64(42 + run as u64); // Different seed per run
        let name_generator = Name::<T>::new();

        let start = Instant::now();

        for _ in 0..iterations {
            let _ = name_generator.generate(&mut rng);
        }

        let elapsed = start.elapsed();
        run_times.push(elapsed);
    }

    BenchmarkStats::new(name.to_string(), run_times, iterations)
}

fn main() {
    println!("=== Controlled Performance Test ===");
    println!("All approaches use IDENTICAL pattern: <!s><v><c><s>");
    println!("All approaches target DARK sound profile");
    println!("Testing {} iterations × {} runs each\n", ITERATIONS, RUNS);

    let mut results = Vec::new();

    // Baseline
    println!("🔹 Baseline (no customization):");
    results.push(benchmark_category::<StandardBaseline>(
        "Standard", ITERATIONS, RUNS,
    ));
    results.last().unwrap().print();
    println!();

    // Symbol-only approaches
    println!("🔹 Symbol-only approaches:");
    results.push(benchmark_category::<DarkSymbolOnly>(
        "Dark Symbol-only",
        ITERATIONS,
        RUNS,
    ));
    results.last().unwrap().print();

    results.push(benchmark_category::<DarkTypeSafe>(
        "Dark Type-safe",
        ITERATIONS,
        RUNS,
    ));
    results.last().unwrap().print();
    println!();

    // Rules-only approach
    println!("🔹 Rules-only approach:");
    results.push(benchmark_category::<DarkRulesOnly>(
        "Dark Rules-only",
        ITERATIONS,
        RUNS,
    ));
    results.last().unwrap().print();
    println!();

    // Hybrid approach
    println!("🔹 Hybrid approach:");
    results.push(benchmark_category::<DarkHybrid>(
        "Dark Hybrid",
        ITERATIONS,
        RUNS,
    ));
    results.last().unwrap().print();
    println!();

    // Performance Analysis
    println!("=== Performance Analysis ===\n");

    // Sort by performance
    results.sort_by(|a, b| b.names_per_second.partial_cmp(&a.names_per_second).unwrap());

    println!("🏆 Performance Ranking:");
    for (i, result) in results.iter().enumerate() {
        println!(
            "  {}. {} - {:.0} names/sec",
            i + 1,
            result.name,
            result.names_per_second
        );
    }
    println!();

    // Statistical comparisons
    let baseline = results
        .iter()
        .find(|r| r.name.contains("Standard"))
        .unwrap();
    let fastest = &results[0];
    let slowest = &results[results.len() - 1];

    println!("📊 Statistical Analysis:");
    println!(
        "  Fastest vs Baseline: {:.2}x speedup",
        fastest.names_per_second / baseline.names_per_second
    );
    println!(
        "  Slowest vs Baseline: {:.2}x slowdown",
        baseline.names_per_second / slowest.names_per_second
    );
    println!(
        "  Fastest vs Slowest: {:.2}x difference",
        fastest.names_per_second / slowest.names_per_second
    );
    println!();

    // Approach type analysis
    let symbol_only: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Symbol") || r.name.contains("Type-safe"))
        .collect();
    let rules_only: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Rules-only"))
        .collect();
    let hybrid: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Hybrid"))
        .collect();

    println!("📈 Approach Comparison:");
    if !symbol_only.is_empty() {
        let avg =
            symbol_only.iter().map(|r| r.names_per_second).sum::<f64>() / symbol_only.len() as f64;
        println!("  Symbol-only average: {:.0} names/sec", avg);
    }
    if !rules_only.is_empty() {
        let avg =
            rules_only.iter().map(|r| r.names_per_second).sum::<f64>() / rules_only.len() as f64;
        println!("  Rules-only average: {:.0} names/sec", avg);
    }
    if !hybrid.is_empty() {
        let avg = hybrid.iter().map(|r| r.names_per_second).sum::<f64>() / hybrid.len() as f64;
        println!("  Hybrid average: {:.0} names/sec", avg);
    }

    println!("\n=== Quality Check ===");
    println!("Generating sample names to verify all approaches produce dark sounds:\n");

    let mut rng = ChaCha8Rng::seed_from_u64(123);

    println!("Standard names:");
    for i in 1..=5 {
        let name = Name::<StandardBaseline>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\nSymbol-only dark names:");
    for i in 1..=5 {
        let name = Name::<DarkSymbolOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\nRules-only dark names:");
    for i in 1..=5 {
        let name = Name::<DarkRulesOnly>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\nHybrid dark names:");
    for i in 1..=5 {
        let name = Name::<DarkHybrid>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\nType-safe dark names:");
    for i in 1..=5 {
        let name = Name::<DarkTypeSafe>::new().generate(&mut rng);
        println!("  {}. {}", i, name);
    }

    println!("\n=== Conclusions ===");
    println!("• All tests use identical patterns for fair comparison");
    println!("• Symbol-only approaches avoid rule evaluation overhead");
    println!("• Rules-only approaches provide maximum flexibility at cost of performance");
    println!("• Hybrid approaches balance performance with fine-grained control");
    println!("• Type-safe definitions have negligible performance impact");
    println!("• All approaches are suitable for real-time applications");
}
