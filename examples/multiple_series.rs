// examples/multiple_series.rs
// Examples with multiple data series

use rasciichart::*;

fn main() {
    println!("=== rasciichart - Multiple Series Examples ===\n");

    // Example 1: Two opposite trends
    println!("1. Two Opposite Trends:");
    let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let series2 = vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    
    println!("Series 1 (Increasing):\n{}\n", plot(&series1));
    println!("Series 2 (Decreasing):\n{}\n", plot(&series2));
    println!("Combined View:\n{}\n", plot_multiple(&[&series1, &series2]));

    // Example 2: Sine and Cosine
    println!("2. Sine and Cosine Waves:");
    let sine = generate_sine(60, 2.0, 0.0);
    let cosine = generate_cosine(60, 2.0, 0.0);
    
    println!("Sine:\n{}\n", plot_sized(&sine, 12, 60));
    println!("Cosine:\n{}\n", plot_sized(&cosine, 12, 60));

    // Example 3: Different frequencies
    println!("3. Different Frequency Sine Waves:");
    let freq1 = generate_sine(80, 1.0, 0.0);
    let freq2 = generate_sine(80, 2.0, 0.0);
    let freq3 = generate_sine(80, 3.0, 0.0);
    
    println!("Frequency 1x:\n{}\n", plot_sized(&freq1, 10, 80));
    println!("Frequency 2x:\n{}\n", plot_sized(&freq2, 10, 80));
    println!("Frequency 3x:\n{}\n", plot_sized(&freq3, 10, 80));

    // Example 4: Phase shifted waves
    println!("4. Phase Shifted Waves:");
    let phase0 = generate_sine(60, 1.0, 0.0);
    let phase90 = generate_sine(60, 1.0, std::f64::consts::PI / 2.0);
    let phase180 = generate_sine(60, 1.0, std::f64::consts::PI);
    
    println!("Phase 0°:\n{}\n", plot_sized(&phase0, 10, 60));
    println!("Phase 90°:\n{}\n", plot_sized(&phase90, 10, 60));
    println!("Phase 180°:\n{}\n", plot_sized(&phase180, 10, 60));

    // Example 5: Random walks
    println!("5. Three Random Walks:");
    let walk1 = generate_random_walk(50, 0.0, 1.0);
    let walk2 = generate_random_walk(50, 0.0, 1.0);
    let walk3 = generate_random_walk(50, 0.0, 1.0);
    
    println!("Walk 1:\n{}\n", plot_sized(&walk1, 12, 50));
    println!("Walk 2:\n{}\n", plot_sized(&walk2, 12, 50));
    println!("Walk 3:\n{}\n", plot_sized(&walk3, 12, 50));

    // Example 6: Scaled comparisons
    println!("6. Comparing Different Scales:");
    let small = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let medium = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let large = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    
    println!("Small scale (0.1-0.5):\n{}\n", plot_sized(&small, 10, 40));
    println!("Medium scale (1-5):\n{}\n", plot_sized(&medium, 10, 40));
    println!("Large scale (10-50):\n{}\n", plot_sized(&large, 10, 40));

    // Example 7: Stock-like data
    println!("7. Stock Price Simulation:");
    let mut stock_price = 100.0;
    let mut prices = vec![stock_price];
    
    for i in 1..50 {
        // Simulate price movement
        let change = ((i * 7) % 13) as f64 - 6.0;
        stock_price += change * 0.5;
        prices.push(stock_price);
    }
    
    println!("{}\n", plot_sized(&prices, 15, 50));

    // Example 8: Exponential growth
    println!("8. Exponential Growth:");
    let exponential: Vec<f64> = (0..30)
        .map(|x| 1.0 * 1.1f64.powi(x))
        .collect();
    println!("{}\n", plot_sized(&exponential, 15, 50));

    // Example 9: Logarithmic curve
    println!("9. Logarithmic Curve:");
    let logarithmic: Vec<f64> = (1..40)
        .map(|x| (x as f64).ln())
        .collect();
    println!("{}\n", plot_sized(&logarithmic, 12, 40));

    // Example 10: Polynomial curve
    println!("10. Quadratic Curve:");
    let quadratic: Vec<f64> = (0..30)
        .map(|x| {
            let t = x as f64 / 5.0;
            t * t
        })
        .collect();
    println!("{}\n", plot_sized(&quadratic, 15, 50));
}