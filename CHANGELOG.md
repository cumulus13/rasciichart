# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.6] - 2024-12-08

### Fixed
- **PROPERLY fixed real-time flicker** by using format!() to build entire output including clear command in single string
- Fixed example 8 and 9 spacing in advanced.rs by adding println!() after error messages
- Implemented correct technique from terminal experts: "Don't clear then write - build everything in buffer first"

### Changed
- Real-time example now uses `format!()` with `\x1B[2J\x1B[H` included in output string
- Simpler code that actually works correctly
- Documentation updated with correct anti-flicker explanation based on real research

## [0.2.3] - 2024-12-08

### Fixed
- **Improved real-time example to achieve ZERO flicker** by building entire screen in buffer before single write operation
- Fixed missing newlines in advanced example error handling cases (#8 and #9)

### Changed
- Real-time example now uses `render_screen()` function that buffers entire output
- Pre-allocates String buffer with 2048 bytes capacity for better performance
- Reduced write operations from multiple to single per frame

## [0.2.2] - 2024-12-08

### Fixed
- **CRITICAL**: Fixed character direction bug where corner symbols (╮╯╰╭) were flipped, causing jagged and unnatural looking charts
  - Corrected logic for top_right (╮) and bottom_right (╯) placement
  - Corrected logic for top_left (╭) and bottom_left (╰) placement
  - Charts now display smooth, natural curves instead of jagged lines
- Added tests for ascending and descending lines to prevent regression

## [0.2.1] - 2024-12-08

### Fixed
- **CRITICAL**: Fixed bug where very small data ranges (e.g., 1.001 to 1.003) would create charts with thousands of rows instead of respecting the configured height
- Fixed screen flickering in real-time example by using proper ANSI escape codes and buffer flushing
- Fixed compiler warnings:
  - Removed unnecessary `mut` in advanced.rs example
  - Removed unnecessary parentheses in stock_chart.rs example
- Improved NaN and Infinity handling by filtering non-finite values before min/max calculation
- Better handling of edge case where all values are identical

### Changed
- Simplified height calculation logic to always use configured height instead of computing from range
- Improved real-time example with smoother updates and better visual feedback
- Enhanced test coverage for edge cases

## [0.2.0] - 2024-12-08

### Added
- Comprehensive error handling with `ChartError` enum
- Result-based API with `plot_with_config()` returning `Result<String, ChartError>`
- Helper functions:
  - `plot()` - Simple plot with defaults
  - `plot_sized()` - Plot with custom dimensions
  - `plot_no_labels()` - Plot without Y-axis labels
  - `plot_range()` - Plot with custom min/max
  - `plot_ascii()` - Plot with ASCII-only characters
  - `plot_multiple()` - Plot multiple series
- Data generation helpers:
  - `generate_sine()` - Generate sine wave data
  - `generate_cosine()` - Generate cosine wave data
  - `generate_random_walk()` - Generate random walk data
- `Symbols` struct for customizable drawing characters
- ASCII-only symbol support via `Symbols::ascii()`
- Configurable Y-axis label ticks
- Custom label format strings
- Proper handling of edge cases:
  - Empty datasets
  - Single data points
  - NaN and Infinity values
  - Invalid ranges
  - Zero dimensions
- Comprehensive unit tests
- Full examples:
  - `simple.rs` - Basic usage patterns
  - `advanced.rs` - Advanced configuration
  - `multiple_series.rs` - Multiple data series
  - `realtime.rs` - Real-time simulation
  - `stock_chart.rs` - Stock market simulation
- Complete documentation with rustdoc
- README with usage examples
- PROJECT_STRUCTURE guide
- CHANGELOG

### Changed
- Improved `Config` builder pattern with validation
- Better Y-axis label positioning algorithm
- Enhanced line rendering smoothness
- Optimized canvas initialization
- Changed offset type from `f64` to `usize` for clarity
- Improved ratio calculation for better scaling

### Fixed
- Edge case handling for flat lines
- Proper handling of NaN and Infinity in data
- Y-axis label alignment issues
- Chart rendering with very small ranges
- Off-by-one errors in coordinate calculations

### Documentation
- Added comprehensive rustdoc comments
- Added usage examples for all public functions
- Created detailed README.md
- Added PROJECT_STRUCTURE.md guide
- Added inline code examples

### Testing
- Added 15+ unit tests covering:
  - Basic functionality
  - Edge cases
  - Error conditions
  - Data generation
  - Configuration validation

## [0.1.0] - 2024-11-15

### Added
- Initial release
- Basic ASCII chart plotting
- Smooth line rendering with box-drawing characters
- Configurable height and width
- Y-axis labels
- Simple configuration system
- Basic tests

[Unreleased]: https://github.com/cumulus13/rasciichart/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/cumulus13/rasciichart/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/cumulus13/rasciichart/releases/tag/v0.1.0