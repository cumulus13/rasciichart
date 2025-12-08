// examples/advanced.rs
// Advanced configuration examples

use rasciichart::*;

fn main() {
    println!("=== rasciichart - Advanced Examples ===\n");

    // Example 1: Full custom configuration
    println!("1. Fully Customized Chart:");
    let data = vec![10.0, 20.0, 15.0, 25.0, 30.0, 22.0, 18.0, 28.0];
    let config = Config::new()
        .with_height(20)
        .with_width(70)
        .with_min(0.0)
        .with_max(35.0)
        .with_label_ticks(7)
        .with_label_format("{:.1}".to_string());
    
    match plot_with_config(&data, config) {
        Ok(chart) => println!("{}\n", chart),
        Err(e) => eprintln!("Error: {}\n", e),
    }

    // Example 2: High precision labels
    println!("2. High Precision Labels:");
    let data2 = vec![1.234, 2.567, 3.891, 4.123, 5.456];
    let config2 = Config::new()
        .with_height(12)
        .with_width(50)
        .with_label_format("{:.2}".to_string());
    
    if let Ok(chart) = plot_with_config(&data2, config2) {
        println!("{}\n", chart);
    }

    // Example 3: Wide chart
    println!("3. Extra Wide Chart:");
    let data3: Vec<f64> = (0..100).map(|x| (x as f64 * 0.1).sin() * 5.0 + 10.0).collect();
    println!("{}\n", plot_sized(&data3, 15, 100));

    // Example 4: Tall chart
    println!("4. Extra Tall Chart:");
    let data4 = vec![1.0, 5.0, 2.0, 8.0, 3.0, 9.0, 4.0, 7.0];
    println!("{}\n", plot_sized(&data4, 25, 40));

    // Example 5: Few label ticks
    println!("5. Minimal Labels (3 ticks):");
    let data5 = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let config5 = Config::new()
        .with_height(15)
        .with_label_ticks(3);
    
    if let Ok(chart) = plot_with_config(&data5, config5) {
        println!("{}\n", chart);
    }

    // Example 6: Many label ticks
    println!("6. Many Labels (10 ticks):");
    let data6 = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0];
    let config6 = Config::new()
        .with_height(20)
        .with_label_ticks(10);
    
    if let Ok(chart) = plot_with_config(&data6, config6) {
        println!("{}\n", chart);
    }

    // Example 7: Custom symbols
    println!("7. Custom ASCII Symbols:");
    let data7 = vec![1.0, 3.0, 2.0, 4.0, 3.0, 5.0];
    let symbols = Symbols::ascii();
    let config7 = Config::new()
        .with_height(12)
        .with_symbols(symbols);
    
    if let Ok(chart) = plot_with_config(&data7, config7) {
        println!("{}\n", chart);
    }

    // Example 8: Error handling
    println!("8. Error Handling - Empty Data:");
    let empty_data: Vec<f64> = vec![];
    match plot_with_config(&empty_data, Config::new()) {
        Ok(chart) => println!("{}", chart),
        Err(e) => println!("Caught error: {}", e),
    }
    println!();

    println!("9. Error Handling - Invalid Range:");
    let config_invalid = Config::new()
        .with_min(10.0)
        .with_max(5.0);
    
    match config_invalid.validate() {
        Ok(_) => println!("Config is valid"),
        Err(e) => println!("Caught error: {}\n", e),
    }

    // Example 10: With NaN values
    println!("10. Handling NaN Values:");
    let data_with_nan = vec![1.0, 2.0, f64::NAN, 4.0, 5.0, f64::NAN, 7.0];
    println!("{}\n", plot(&data_with_nan));

    // Example 11: Negative values
    println!("11. Negative Values:");
    let data_negative = vec![-5.0, -2.0, 0.0, 2.0, 5.0, 3.0, -3.0];
    println!("{}\n", plot(&data_negative));

    // Example 12: Very small range - FIXED!
    println!("12. Very Small Value Range:");
    let data_small = vec![100.001, 100.002, 100.003, 100.002, 100.001];
    let config_small = Config::new()
        .with_height(12)
        .with_width(50);
    if let Ok(chart) = plot_with_config(&data_small, config_small) {
        println!("{}\n", chart);
    }
}