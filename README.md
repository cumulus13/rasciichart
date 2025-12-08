# rasciichart

[![Crates.io](https://img.shields.io/crates/v/rasciichart.svg)](https://crates.io/crates/rasciichart)
[![Documentation](https://docs.rs/rasciichart/badge.svg)](https://docs.rs/rasciichart)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Beautiful ASCII line charts in Rust with smooth rendering, inspired by [asciichartpy](https://github.com/kroitor/asciichart).

## Features

- 📊 **Smooth line rendering** with Unicode box-drawing characters
- 🎨 **Highly customizable** - height, width, colors, symbols, labels
- 🚀 **Zero dependencies** - lightweight and fast
- 💪 **Type-safe** - leverages Rust's type system
- 📝 **Well documented** - comprehensive examples and API docs
- 🔧 **Helper functions** - for common use cases
- 🎯 **Production ready** - proper error handling and edge cases


## Demo

<p align="center">
  <img src="https://raw.githubusercontent.com/cumulus13/rasciichart/master/demo.gif" alt="rasciichart demo">
</p>


## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rasciichart = "0.2.0"
```

## Quick Start

```rust
use rasciichart::plot;

fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    println!("{}", plot(&data));
}
```

Output:
```
5.00 │     ╭╮
     │     ││
4.20 │     ││
     │    ╭╯╰╮
3.40 │    │  │
     │   ╭╯  ╰╮
2.60 │   │    │
     │   │    │
1.80 │  ╭╯    ╰╮
     │  │      │
1.00 │ │╯      ╰
```

## Examples

### Basic Usage

```rust
use rasciichart::plot;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot(&data));
```

### Custom Size

```rust
use rasciichart::plot_sized;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot_sized(&data, 15, 60)); // height: 15, width: 60
```

### Custom Range

```rust
use rasciichart::plot_range;

let data = vec![5.0, 6.0, 7.0, 8.0, 9.0];
println!("{}", plot_range(&data, 0.0, 10.0)); // min: 0, max: 10
```

### Without Labels

```rust
use rasciichart::plot_no_labels;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot_no_labels(&data));
```

### ASCII-only Characters

For better compatibility with terminals that don't support Unicode:

```rust
use rasciichart::plot_ascii;

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
println!("{}", plot_ascii(&data));
```

### Advanced Configuration

```rust
use rasciichart::{plot_with_config, Config};

let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let config = Config::new()
    .with_height(20)
    .with_width(80)
    .with_min(0.0)
    .with_max(10.0)
    .with_label_ticks(6)
    .with_label_format("{:.1}".to_string());

match plot_with_config(&data, config) {
    Ok(chart) => println!("{}", chart),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Generate Test Data

```rust
use rasciichart::{generate_sine, generate_cosine, plot};

// Sine wave
let sine_data = generate_sine(80, 2.0, 0.0);
println!("Sine wave:\n{}", plot(&sine_data));

// Cosine wave
let cosine_data = generate_cosine(80, 2.0, 0.0);
println!("\nCosine wave:\n{}", plot(&cosine_data));
```

### Multiple Series (Overlaid)

```rust
use rasciichart::plot_multiple;

let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let series2 = vec![5.0, 4.0, 3.0, 2.0, 1.0];
println!("{}", plot_multiple(&[&series1, &series2]));
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `height` | `usize` | `10` | Height of the chart in rows |
| `width` | `usize` | `80` | Width of the chart in columns |
| `offset` | `usize` | `3` | Left margin for labels |
| `min` | `Option<f64>` | `None` | Minimum Y-axis value (auto if None) |
| `max` | `Option<f64>` | `None` | Maximum Y-axis value (auto if None) |
| `show_labels` | `bool` | `true` | Show Y-axis labels |
| `label_ticks` | `usize` | `5` | Number of Y-axis label ticks |
| `label_format` | `String` | `"{:.2}"` | Format string for labels |
| `symbols` | `Symbols` | Unicode | Characters for drawing |

## API Reference

### Main Functions

- **`plot(series: &[f64]) -> String`** - Simple plot with defaults
- **`plot_sized(series: &[f64], height: usize, width: usize) -> String`** - Plot with custom size
- **`plot_range(series: &[f64], min: f64, max: f64) -> String`** - Plot with custom range
- **`plot_no_labels(series: &[f64]) -> String`** - Plot without Y-axis labels
- **`plot_ascii(series: &[f64]) -> String`** - Plot with ASCII-only characters
- **`plot_multiple(series: &[&[f64]]) -> String`** - Plot multiple series
- **`plot_with_config(series: &[f64], config: Config) -> Result<String>`** - Plot with full configuration

### Helper Functions

- **`generate_sine(points: usize, frequency: f64, phase: f64) -> Vec<f64>`** - Generate sine wave
- **`generate_cosine(points: usize, frequency: f64, phase: f64) -> Vec<f64>`** - Generate cosine wave
- **`generate_random_walk(points: usize, start: f64, volatility: f64) -> Vec<f64>`** - Generate random walk

### Types

- **`Config`** - Chart configuration with builder pattern
- **`Symbols`** - Custom drawing characters
- **`ChartError`** - Error types for the library

## Running Examples

The library includes several examples:

```bash
# Simple example
cargo run --example simple

# Advanced configuration
cargo run --example advanced

# Multiple series
cargo run --example multiple_series

# Real-time simulation
cargo run --example realtime

# Stock chart simulation
cargo run --example stock_chart
```

## Error Handling

The library provides proper error handling:

```rust
use rasciichart::{plot_with_config, Config, ChartError};

let data = vec![];
let config = Config::new();

match plot_with_config(&data, config) {
    Ok(chart) => println!("{}", chart),
    Err(ChartError::EmptyData) => println!("No data to plot"),
    Err(e) => println!("Error: {}", e),
}
```

## Edge Cases Handled

- Empty data sets
- Single data point
- NaN and Infinity values
- Invalid ranges (min >= max)
- Zero dimensions
- Very large or very small numbers

## Performance

The library is designed to be fast and memory-efficient:

- No heap allocations in hot paths
- Efficient string building
- Minimal copies of data
- O(n) time complexity where n is the number of data points

## Comparison with asciichartpy

| Feature | rasciichart | asciichartpy |
|---------|-------------|--------------|
| Language | Rust | Python |
| Dependencies | 0 | 0 |
| Type Safety | ✅ Strong | ❌ Dynamic |
| Performance | ⚡ Fast | 🐌 Slower |
| Error Handling | ✅ Result<T> | ❌ Exceptions |
| Unicode Support | ✅ Yes | ✅ Yes |
| ASCII Fallback | ✅ Yes | ✅ Yes |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Hadi Cahyadi** - [cumulus13@gmail.com](mailto:cumulus13@gmail.com)

[![Buy Me a Coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/cumulus13)

[![Donate via Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/cumulus13)

[Support me on Patreon](https://www.patreon.com/cumulus13)

## Acknowledgments

- Inspired by [asciichartpy](https://github.com/kroitor/asciichart) by Igor Kroitor
- Unicode box-drawing characters from the Unicode Standard

## Changelog

### 0.2.0 (2024)
- Complete rewrite with better API
- Added comprehensive error handling
- Added helper functions
- Improved documentation
- Added examples
- Better edge case handling

### 0.1.0 (2024)
- Initial release
- Basic plotting functionality

## Support

If you find this library useful, please give it a ⭐ on GitHub!