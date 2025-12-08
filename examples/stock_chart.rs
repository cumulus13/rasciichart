// examples/stock_chart.rs
// Stock market chart simulation

use rasciichart::*;

fn main() {
    println!("=== rasciichart - Stock Market Simulation ===\n");

    // Example 1: Simple stock price over time
    println!("1. Stock Price - Daily Close:");
    let stock_prices = vec![
        100.0, 102.5, 101.0, 103.0, 105.5, 104.0, 106.5, 108.0,
        107.0, 109.5, 112.0, 110.5, 113.0, 115.5, 114.0, 116.5,
        118.0, 117.0, 119.5, 122.0
    ];
    
    let config = Config::new()
        .with_height(15)
        .with_width(60)
        .with_label_format("{:.1}".to_string());
    
    if let Ok(chart) = plot_with_config(&stock_prices, config) {
        println!("{}", chart);
        println!("\nStart: ${:.2} | End: ${:.2} | Change: ${:.2} ({:.2}%)\n",
            stock_prices[0],
            stock_prices[stock_prices.len() - 1],
            stock_prices[stock_prices.len() - 1] - stock_prices[0],
            (stock_prices[stock_prices.len() - 1] / stock_prices[0] - 1.0) * 100.0
        );
    }

    // Example 2: Volatile stock
    println!("2. Volatile Tech Stock:");
    let volatile_stock = vec![
        50.0, 55.0, 52.0, 58.0, 62.0, 59.0, 65.0, 70.0,
        68.0, 75.0, 72.0, 78.0, 82.0, 79.0, 85.0, 88.0,
        84.0, 90.0, 95.0, 92.0, 98.0, 102.0, 99.0, 105.0
    ];
    println!("{}\n", plot_sized(&volatile_stock, 15, 70));

    // Example 3: Market crash scenario
    println!("3. Market Crash Scenario:");
    let mut crash_data = vec![150.0; 10]; // Stable period
    
    // Crash period
    for i in 0..15 {
        crash_data.push(150.0 - (i as f64 * 5.0));
    }
    
    // Recovery period
    for i in 0..20 {
        crash_data.push(75.0 + (i as f64 * 2.0));
    }
    
    println!("{}\n", plot_sized(&crash_data, 18, 80));

    // Example 4: Bull market
    println!("4. Bull Market (Steady Growth):");
    let bull_market: Vec<f64> = (0..40)
        .map(|x| {
            let base = 100.0;
            let growth = x as f64 * 1.5;
            let noise = ((x * 7) % 11) as f64 - 5.0;
            base + growth + noise
        })
        .collect();
    println!("{}\n", plot_sized(&bull_market, 15, 70));

    // Example 5: Sideways market
    println!("5. Sideways/Range-bound Market:");
    let sideways: Vec<f64> = (0..50)
        .map(|x| {
            let base = 100.0;
            let oscillation = (x as f64 * 0.5).sin() * 8.0;
            let noise = ((x * 13) % 7) as f64 - 3.0;
            base + oscillation + noise
        })
        .collect();
    println!("{}\n", plot_sized(&sideways, 12, 70));

    // Example 6: Multiple stocks comparison
    println!("6. Portfolio - Three Stocks:");
    
    // Conservative stock (bonds-like)
    let conservative: Vec<f64> = (0..30)
        .map(|x| 100.0 + x as f64 * 0.3)
        .collect();
    
    // Moderate stock
    let moderate: Vec<f64> = (0..30)
        .map(|x| {
            100.0 + x as f64 * 0.8 + ((x * 3) % 5) as f64 - 2.0
        })
        .collect();
    
    // Aggressive stock
    let aggressive: Vec<f64> = (0..30)
        .map(|x| {
            100.0 + x as f64 * 1.5 + ((x * 7) % 11) as f64 - 5.0
        })
        .collect();
    
    println!("Conservative (Low Risk):\n{}\n", plot_sized(&conservative, 10, 60));
    println!("Moderate (Medium Risk):\n{}\n", plot_sized(&moderate, 10, 60));
    println!("Aggressive (High Risk):\n{}\n", plot_sized(&aggressive, 10, 60));

    // Example 7: Dividend reinvestment effect
    println!("7. Dividend Reinvestment Effect:");
    let mut with_dividends = vec![100.0];
    let mut without_dividends = vec![100.0];
    
    for month in 1..36 {
        // Base growth
        let growth_rate = 1.005; // 0.5% per month
        
        // Without dividends
        without_dividends.push(without_dividends[month - 1] * growth_rate);
        
        // With dividends (quarterly)
        if month % 3 == 0 {
            with_dividends.push(with_dividends[month - 1] * growth_rate * 1.01); // +1% dividend
        } else {
            with_dividends.push(with_dividends[month - 1] * growth_rate);
        }
    }
    
    println!("Without Dividends:\n{}\n", plot_sized(&without_dividends, 12, 70));
    println!("With Dividends:\n{}\n", plot_sized(&with_dividends, 12, 70));

    // Example 8: Market indices comparison
    println!("8. Market Indices Simulation:");
    let sp500: Vec<f64> = (0..40)
        .map(|x| 3000.0 + x as f64 * 15.0 + ((x * 5) % 20) as f64 - 10.0)
        .collect();
    
    let nasdaq: Vec<f64> = (0..40)
        .map(|x| 12000.0 + x as f64 * 80.0 + ((x * 7) % 40) as f64 - 20.0)
        .collect();
    
    println!("S&P 500:\n{}\n", plot_sized(&sp500, 12, 70));
    println!("NASDAQ:\n{}\n", plot_sized(&nasdaq, 12, 70));

    // Example 9: Earnings report impact
    println!("9. Stock Price Around Earnings Report:");
    let mut earnings_impact = vec![];
    
    // Before earnings
    for i in 0..10 {
        earnings_impact.push(100.0 + (i as f64 * 0.5));
    }
    
    // Earnings surprise - big jump
    earnings_impact.push(115.0);
    
    // After earnings - stabilization
    for i in 0..15 {
        earnings_impact.push(115.0 + ((i * 3) % 5) as f64 - 2.0);
    }
    
    println!("{}\n", plot_sized(&earnings_impact, 15, 60));

    // Example 10: Statistical analysis
    println!("10. Statistical Summary:");
    let analysis_data = vec![
        45.5, 47.2, 46.8, 48.5, 50.1, 49.5, 51.2, 52.8,
        51.5, 53.2, 54.8, 53.5, 55.1, 56.8, 55.5, 57.2
    ];
    
    println!("{}\n", plot_sized(&analysis_data, 12, 50));
    
    // Calculate statistics
    let mean = analysis_data.iter().sum::<f64>() / analysis_data.len() as f64;
    let min = analysis_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = analysis_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let range = max - min;
    
    println!("Statistics:");
    println!("  Mean:  ${:.2}", mean);
    println!("  Min:   ${:.2}", min);
    println!("  Max:   ${:.2}", max);
    println!("  Range: ${:.2}", range);
    println!("  Volatility: {:.2}%", (range / mean) * 100.0);
}