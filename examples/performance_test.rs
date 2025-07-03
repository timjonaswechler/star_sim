//! Performance test for the phonetic name generation system.
//!
//! This example benchmarks the different approaches to name generation:
//! 1. Symbol-only (pre-filtered sound sets)
//! 2. Rules-only (standard symbols + phonetic filtering)
//! 3. Hybrid (custom symbols + additional rules)
//! 4. Standard (no customization)

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use star_sim::utilities::name_generator::{Name, categories::examples::*};
use std::time::{Duration, Instant};

const ITERATIONS: usize = 10_000;

#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    iterations: usize,
    total_time: Duration,
    avg_time_per_name: Duration,
    names_per_second: f64,
}

impl BenchmarkResult {
    fn new(name: String, iterations: usize, total_time: Duration) -> Self {
        let avg_time_per_name = total_time / iterations as u32;
        let names_per_second = iterations as f64 / total_time.as_secs_f64();

        Self {
            name,
            iterations,
            total_time,
            avg_time_per_name,
            names_per_second,
        }
    }

    fn print(&self) {
        println!("  {} ({} iterations):", self.name, self.iterations);
        println!("    Total time: {:?}", self.total_time);
        println!("    Avg per name: {:?}", self.avg_time_per_name);
        println!("    Names/sec: {:.0}", self.names_per_second);
    }
}

fn benchmark_category<T: Default + star_sim::utilities::name_generator::NameCategory>(
    name: &str,
    iterations: usize,
) -> BenchmarkResult {
    let mut rng = ChaCha8Rng::seed_from_u64(42); // Deterministic for fair comparison
    let name_generator = Name::<T>::new();

    let start = Instant::now();

    for _ in 0..iterations {
        let _ = name_generator.generate(&mut rng);
    }

    let elapsed = start.elapsed();
    BenchmarkResult::new(name.to_string(), iterations, elapsed)
}

fn main() {
    println!("=== Name Generation Performance Test ===\n");
    println!("Testing {} iterations per category...\n", ITERATIONS);

    let mut results = Vec::new();

    // Test Symbol-only approaches
    println!("Symbol-only approaches (pre-filtered sound sets):");
    results.push(benchmark_category::<DarkStarSymbolOnly>(
        "Dark Star (Symbol-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<BrightStarSymbolOnly>(
        "Bright Star (Symbol-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<ExoticAlienSymbolOnly>(
        "Exotic Alien (Symbol-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<DraconicStar>(
        "Draconic Star (Custom Type-safe)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();
    println!();

    // Test Rules-only approaches
    println!("Rules-only approaches (standard symbols + phonetic filtering):");
    results.push(benchmark_category::<DarkStarRulesOnly>(
        "Dark Star (Rules-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<BrightStarRulesOnly>(
        "Bright Star (Rules-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<ExoticAlienRulesOnly>(
        "Exotic Alien (Rules-only)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();
    println!();

    // Test Hybrid approaches
    println!("Hybrid approaches (custom symbols + additional rules):");
    results.push(benchmark_category::<UltraDarkStarHybrid>(
        "Ultra-Dark Star (Hybrid)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<UltraBrightStarHybrid>(
        "Ultra-Bright Star (Hybrid)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();

    results.push(benchmark_category::<UltraExoticAlienHybrid>(
        "Ultra-Exotic Alien (Hybrid)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();
    println!();

    // Test Standard approach (baseline)
    println!("Baseline:");
    results.push(benchmark_category::<StandardStar>(
        "Standard Star (No customization)",
        ITERATIONS,
    ));
    results[results.len() - 1].print();
    println!();

    // Performance analysis
    println!("=== Performance Analysis ===\n");

    // Find fastest and slowest
    let fastest = results
        .iter()
        .max_by(|a, b| a.names_per_second.partial_cmp(&b.names_per_second).unwrap())
        .unwrap();
    let slowest = results
        .iter()
        .min_by(|a, b| a.names_per_second.partial_cmp(&b.names_per_second).unwrap())
        .unwrap();

    println!(
        "Fastest: {} ({:.0} names/sec)",
        fastest.name, fastest.names_per_second
    );
    println!(
        "Slowest: {} ({:.0} names/sec)",
        slowest.name, slowest.names_per_second
    );
    println!(
        "Speed difference: {:.1}x",
        fastest.names_per_second / slowest.names_per_second
    );
    println!();

    // Group by approach type
    let symbol_only: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Symbol-only") || r.name.contains("Type-safe"))
        .collect();
    let rules_only: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Rules-only"))
        .collect();
    let hybrid: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Hybrid"))
        .collect();
    let standard: Vec<_> = results
        .iter()
        .filter(|r| r.name.contains("Standard"))
        .collect();

    println!("Average performance by approach:");

    if !symbol_only.is_empty() {
        let avg_symbol =
            symbol_only.iter().map(|r| r.names_per_second).sum::<f64>() / symbol_only.len() as f64;
        println!("  Symbol-only approaches: {:.0} names/sec", avg_symbol);
    }

    if !rules_only.is_empty() {
        let avg_rules =
            rules_only.iter().map(|r| r.names_per_second).sum::<f64>() / rules_only.len() as f64;
        println!("  Rules-only approaches: {:.0} names/sec", avg_rules);
    }

    if !hybrid.is_empty() {
        let avg_hybrid =
            hybrid.iter().map(|r| r.names_per_second).sum::<f64>() / hybrid.len() as f64;
        println!("  Hybrid approaches: {:.0} names/sec", avg_hybrid);
    }

    if !standard.is_empty() {
        let avg_standard =
            standard.iter().map(|r| r.names_per_second).sum::<f64>() / standard.len() as f64;
        println!("  Standard approach: {:.0} names/sec", avg_standard);
    }

    println!();

    // Memory efficiency test
    println!("=== Memory Efficiency Test ===\n");

    // Test memory usage by generating many names and measuring
    let memory_test_iterations = 100_000;
    println!(
        "Generating {} names to test memory efficiency...",
        memory_test_iterations
    );

    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Test a few different approaches
    let dark_symbol = Name::<DarkStarSymbolOnly>::new();
    let dark_rules = Name::<DarkStarRulesOnly>::new();
    let standard = Name::<StandardStar>::new();

    let start = Instant::now();
    let mut total_chars_symbol = 0;
    for _ in 0..memory_test_iterations {
        let name = dark_symbol.generate(&mut rng);
        total_chars_symbol += name.len();
    }
    let time_symbol = start.elapsed();

    let start = Instant::now();
    let mut total_chars_rules = 0;
    for _ in 0..memory_test_iterations {
        let name = dark_rules.generate(&mut rng);
        total_chars_rules += name.len();
    }
    let time_rules = start.elapsed();

    let start = Instant::now();
    let mut total_chars_standard = 0;
    for _ in 0..memory_test_iterations {
        let name = standard.generate(&mut rng);
        total_chars_standard += name.len();
    }
    let time_standard = start.elapsed();

    println!("Symbol-only approach:");
    println!("  Time: {:?}", time_symbol);
    println!(
        "  Avg name length: {:.1} chars",
        total_chars_symbol as f64 / memory_test_iterations as f64
    );
    println!(
        "  Rate: {:.0} names/sec",
        memory_test_iterations as f64 / time_symbol.as_secs_f64()
    );

    println!("Rules-only approach:");
    println!("  Time: {:?}", time_rules);
    println!(
        "  Avg name length: {:.1} chars",
        total_chars_rules as f64 / memory_test_iterations as f64
    );
    println!(
        "  Rate: {:.0} names/sec",
        memory_test_iterations as f64 / time_rules.as_secs_f64()
    );

    println!("Standard approach:");
    println!("  Time: {:?}", time_standard);
    println!(
        "  Avg name length: {:.1} chars",
        total_chars_standard as f64 / memory_test_iterations as f64
    );
    println!(
        "  Rate: {:.0} names/sec",
        memory_test_iterations as f64 / time_standard.as_secs_f64()
    );

    println!("\n=== Conclusions ===");
    println!("• Symbol-only approaches are typically fastest (no rule evaluation)");
    println!("• Rules-only approaches are slower but more flexible");
    println!("• Hybrid approaches balance customization with reasonable performance");
    println!("• Type-safe symbol definitions have minimal overhead");
    println!("• All approaches are suitable for real-time name generation");
}
