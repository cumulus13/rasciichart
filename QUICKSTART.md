# Quick Start Guide - rasciichart

## 📦 Installation in 30 Seconds

### 1. Create a New Project

```bash
cargo new my_charts
cd my_charts
```

### 2. Add Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
rasciichart = "0.2.0"
```

### 3. Write the First Code

Edit `src/main.rs`:

```rust
use rasciichart::plot;

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    println!("{}", plot(&data));
}
```

### 4. Run!

```bash
cargo run
```

## 🎯 5 Most Common Examples

### 1. Simple Plot

```rust
use rasciichart::plot;

let temps = vec![20.5, 22.0, 23.5, 21.0, 19.5];
println!("Suhu Harian:\n{}", plot(&temps));
```

### 2. Custom Size

```rust
use rasciichart::plot_sized;

let sales = vec![100.0, 150.0, 120.0, 180.0, 200.0];
println!("{}", plot_sized(&sales, 15, 60));
//                         height ↑   ↑ width
```

### 3. Set Min/Max Range

```rust
use rasciichart::plot_range;

let scores = vec![75.0, 82.0, 90.0, 85.0, 88.0];
println!("{}", plot_range(&scores, 0.0, 100.0));
//                         data ↑      ↑ min  ↑ max
```

### 4. No Labels (Clean)

```rust
use rasciichart::plot_no_labels;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot_no_labels(&data));
```

### 5. Full Customization

```rust
use rasciichart::{plot_with_config, Config};

let data = vec![10.0, 20.0, 15.0, 25.0, 30.0];

let config = Config::new()
    .with_height(20)        // Chart height
    .with_width(80)         // Chart width
    .with_min(0.0)          // Minimum value of Y
    .with_max(40.0)         // Maximum value Y
    .with_label_ticks(6);   // Total label

match plot_with_config(&data, config) {
    Ok(chart) => println!("{}", chart),
    Err(e) => eprintln!("Error: {}", e),
}
```

## 🧪 Generate Test Data

```rust
use rasciichart::{generate_sine, plot};

// Generate sine wave dengan 80 points
let sine_data = generate_sine(80, 1.0, 0.0);
println!("{}", plot(&sine_data));
```

## 💡 Use Cases Nyata

### Monitoring CPU Usage

```rust
use rasciichart::plot_sized;

fn monitor_cpu() {
    let cpu_usage = vec![
        45.2, 48.1, 52.3, 49.8, 51.2,
        55.7, 60.3, 58.9, 54.2, 50.1
    ];
    
    println!("CPU Usage (%):");
    println!("{}", plot_sized(&cpu_usage, 12, 50));
}
```

### Stock Price Tracker

```rust
use rasciichart::{plot_with_config, Config};

fn show_stock_price() {
    let prices = vec![
        150.5, 152.3, 151.8, 153.2, 155.0,
        154.5, 156.8, 158.2, 157.0, 159.5
    ];
    
    let config = Config::new()
        .with_height(15)
        .with_width(60)
        .with_label_format("{:.1}".to_string());
    
    println!("AAPL Stock Price:");
    if let Ok(chart) = plot_with_config(&prices, config) {
        println!("{}", chart);
    }
}
```

### Server Response Time

```rust
use rasciichart::plot_range;

fn show_response_times() {
    let response_ms = vec![
        45.0, 52.0, 48.0, 55.0, 62.0,
        58.0, 51.0, 49.0, 53.0, 50.0
    ];
    
    println!("Server Response Time (ms):");
    println!("{}", plot_range(&response_ms, 0.0, 100.0));
}
```

### Sales Dashboard

```rust
use rasciichart::plot_sized;

fn sales_report() {
    let monthly_sales = vec![
        150_000.0, 165_000.0, 180_000.0, 172_000.0,
        185_000.0, 195_000.0, 210_000.0, 205_000.0,
        220_000.0, 235_000.0, 230_000.0, 250_000.0
    ];
    
    println!("Sales Report 2024:");
    println!("{}", plot_sized(&monthly_sales, 15, 80));
    
    // Calculate stats
    let total: f64 = monthly_sales.iter().sum();
    let avg = total / monthly_sales.len() as f64;
    
    println!("\nTotal: ${:.2}", total);
    println!("Average: ${:.2}", avg);
}
```

## 🎨 Styling Options

### ASCII-only (for compatibility)

```rust
use rasciichart::plot_ascii;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot_ascii(&data));
```

Output using `-, |, +` instead of `─, │, ╮, ╯`.

## 🚨 Error Handling

```rust
use rasciichart::{plot_with_config, Config, ChartError};

let data = vec![1.0, 2.0, 3.0];
let config = Config::new().with_min(10.0).with_max(5.0); // Invalid!

match plot_with_config(&data, config) {
    Ok(chart) => println!("{}", chart),
    Err(ChartError::InvalidRange) => {
        println!("Error: Min must be less than max");
    },
    Err(ChartError::EmptyData) => {
        println!("Error: No data to plot");
    },
    Err(e) => println!("Error: {}", e),
}
```

## 📊 Comparison Table

| Function | Purpose | When to Use |
|----------|---------|-------------|
| `plot()` | Simple, dengan defaults | Quick visualization |
| `plot_sized()` | Custom dimensions | Control size |
| `plot_range()` | Fixed Y-axis range | Compare different datasets |
| `plot_no_labels()` | Clean output | Embedding in other UIs |
| `plot_ascii()` | ASCII characters only | Compatibility |
| `plot_with_config()` | Full control | Production use |

## 🔥 Tips & Tricks

### 1. Smooth Lines with Many Data Points

```rust
let data: Vec<f64> = (0..200)
    .map(|x| (x as f64 * 0.1).sin())
    .collect();

println!("{}", plot_sized(&data, 15, 100));
```

### 2. Handle Missing Data (NaN)

```rust
let data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
println!("{}", plot(&data)); // NaN will be skipped
```

### 3. Dynamic Y-axis

```rust
// Auto-scale to data
let config = Config::new()
    .with_height(15)
    .with_width(60);
    // min and max are not set = auto

let result = plot_with_config(&data, config);
```

### 4. Fixed Y-axis for Comparison

```rust
let dataset1 = vec![50.0, 55.0, 52.0, 58.0];
let dataset2 = vec![45.0, 48.0, 46.0, 50.0];

// Use the same range to compare
println!("Dataset 1:\n{}", plot_range(&dataset1, 40.0, 60.0));
println!("\nDataset 2:\n{}", plot_range(&dataset2, 40.0, 60.0));
```

## 🎓 Next Steps

1. **Explore Examples**: `cargo run --example simple`
2. **Read Full Docs**: `cargo doc --open`
3. **Check Advanced Guide**: Read README.md
4. **Join Community**: GitHub Issues

## 📚 Cheat Sheet

```rust
// Import
use rasciichart::*;

// Basic
plot(&data)                              // Simple plot
plot_sized(&data, 15, 60)               // Custom size
plot_range(&data, 0.0, 100.0)           // Fixed range
plot_no_labels(&data)                   // No Y-axis labels
plot_ascii(&data)                       // ASCII only

// Advanced
let config = Config::new()
    .with_height(20)
    .with_width(80)
    .with_min(0.0)
    .with_max(100.0)
    .with_labels(true)
    .with_label_ticks(6)
    .with_label_format("{:.1}".to_string())
    .with_ascii_symbols();

plot_with_config(&data, config)?

// Generators
generate_sine(points, freq, phase)
generate_cosine(points, freq, phase)
generate_random_walk(points, start, volatility)
```

## 🆘 Common Issues

### Issue: Chart is too small

```rust
// Solution: Increase dimensions
plot_sized(&data, 20, 100) // Bigger chart
```

### Issue: Labels overlapping

```rust
// Solution: Reduce label ticks
let config = Config::new().with_label_ticks(4);
```

### Issue: Data is not visible

```rust
// Solution: Set explicit range
plot_range(&data, min_value, max_value)
```

---

**Happy Charting! 📊**

Need help? Open issue on GitHub or email cumulus13@gmail.com