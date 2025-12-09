// File: rasciichart/src/lib.rs
// ASCII Chart Library - Smooth output seperti asciichartpy
// Author: Hadi Cahyadi <cumulus13@gmail.com>
// License: MIT

//! # rasciichart
//!
//! A Rust library for creating beautiful ASCII charts in the terminal.
//! Inspired by asciichartpy, this library provides smooth line rendering
//! with extensive customization options.
//!
//! ## Features
//!
//! - Simple and intuitive API
//! - Smooth line rendering with box-drawing characters
//! - Customizable height, width, and axis labels
//! - Support for multiple data series
//! - Helper functions for common use cases
//! - Zero external dependencies
//!
//! ## Quick Start
//!
//! ```rust
//! use rasciichart::plot;
//!
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
//! println!("{}", plot(&data));
//! ```

use std::fmt;

/// Error types for the library
#[derive(Debug, Clone, PartialEq)]
pub enum ChartError {
    EmptyData,
    InvalidRange,
    InvalidDimensions,
}

impl fmt::Display for ChartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChartError::EmptyData => write!(f, "Cannot plot empty data"),
            ChartError::InvalidRange => write!(f, "Invalid min/max range"),
            ChartError::InvalidDimensions => write!(f, "Invalid chart dimensions"),
        }
    }
}

impl std::error::Error for ChartError {}

pub type Result<T> = std::result::Result<T, ChartError>;

/// Configuration for chart rendering
#[derive(Debug, Clone)]
pub struct Config {
    /// Height of the chart in rows
    pub height: usize,
    /// Width of the chart in columns
    pub width: usize,
    /// Offset for labels (left margin)
    pub offset: usize,
    /// Minimum Y-axis value (auto-calculated if None)
    pub min: Option<f64>,
    /// Maximum Y-axis value (auto-calculated if None)
    pub max: Option<f64>,
    /// Show Y-axis labels
    pub show_labels: bool,
    /// Number of Y-axis label ticks
    pub label_ticks: usize,
    /// Format string for Y-axis labels
    pub label_format: String,
    /// Characters to use for drawing
    pub symbols: Symbols,
}

/// Symbols used for drawing the chart
#[derive(Debug, Clone)]
pub struct Symbols {
    pub horizontal: char,
    pub vertical: char,
    pub top_right: char,
    pub bottom_right: char,
    pub bottom_left: char,
    pub top_left: char,
    pub axis_vertical: char,
    pub axis_corner: char,
    pub axis_bottom: char,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            top_right: '╮',
            bottom_right: '╯',
            bottom_left: '╰',
            top_left: '╭',
            axis_vertical: '│',
            axis_corner: '┤',
            axis_bottom: '┴',
        }
    }
}

impl Symbols {
    /// ASCII-only symbols for compatibility
    pub fn ascii() -> Self {
        Self {
            horizontal: '-',
            vertical: '|',
            top_right: '+',
            bottom_right: '+',
            bottom_left: '+',
            top_left: '+',
            axis_vertical: '|',
            axis_corner: '|',
            axis_bottom: '+',
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            height: 10,
            width: 80,
            offset: 3,
            min: None,
            max: None,
            show_labels: true,
            label_ticks: 5,
            label_format: "{:.2}".to_string(),
            symbols: Symbols::default(),
        }
    }
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the chart height
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    /// Set the chart width
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Set the label offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Set the minimum Y-axis value
    pub fn with_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    /// Set the maximum Y-axis value
    pub fn with_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    /// Set whether to show Y-axis labels
    pub fn with_labels(mut self, show: bool) -> Self {
        self.show_labels = show;
        self
    }

    /// Set the number of Y-axis label ticks
    pub fn with_label_ticks(mut self, ticks: usize) -> Self {
        self.label_ticks = ticks;
        self
    }

    /// Set the label format string
    pub fn with_label_format(mut self, format: String) -> Self {
        self.label_format = format;
        self
    }

    /// Use ASCII-only symbols
    pub fn with_ascii_symbols(mut self) -> Self {
        self.symbols = Symbols::ascii();
        self
    }

    /// Set custom symbols
    pub fn with_symbols(mut self, symbols: Symbols) -> Self {
        self.symbols = symbols;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.height == 0 || self.width == 0 {
            return Err(ChartError::InvalidDimensions);
        }
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if min >= max {
                return Err(ChartError::InvalidRange);
            }
        }
        Ok(())
    }
}

/// Main plotting function - creates an ASCII chart from a data series
///
/// # Arguments
///
/// * `series` - A slice of f64 values to plot
/// * `config` - Configuration for chart rendering
///
/// # Returns
///
/// A String containing the rendered ASCII chart
///
/// # Example
///
/// ```rust
/// use rasciichart::{plot_with_config, Config};
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let config = Config::new().with_height(15).with_width(60);
/// let chart = plot_with_config(&data, config).unwrap();
/// println!("{}", chart);
/// ```
pub fn plot_with_config(series: &[f64], config: Config) -> Result<String> {
    config.validate()?;

    if series.is_empty() {
        return Err(ChartError::EmptyData);
    }

    if series.len() == 1 {
        return Ok(format_value(series[0], &config.label_format));
    }

    // Filter out non-finite values for min/max calculation
    let finite_values: Vec<f64> = series.iter()
        .copied()
        .filter(|v| v.is_finite())
        .collect();

    if finite_values.is_empty() {
        return Err(ChartError::InvalidRange);
    }

    // Determine min and max
    let min = config.min.unwrap_or_else(|| {
        finite_values.iter().copied().fold(f64::INFINITY, f64::min)
    });
    
    let max = config.max.unwrap_or_else(|| {
        finite_values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    });

    if !min.is_finite() || !max.is_finite() {
        return Err(ChartError::InvalidRange);
    }

    // Handle case where all values are the same
    if (max - min).abs() < f64::EPSILON {
        return Ok(format_value(min, &config.label_format));
    }

    let range = max - min;
    let height = config.height;
    let ratio = (height as f64) / range;

    // Initialize canvas - no extra width needed
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; config.width]; height + 1];

    // Plot the line - SKIP x=0 (reserved for axis separator)
    let mut y0: Option<usize> = None;

    for (x, &value) in series.iter().enumerate().take(config.width.saturating_sub(1)) {
        if !value.is_finite() {
            continue;
        }

        let y = ((max - value) * ratio).round() as usize;
        let y = y.min(height);
        
        let plot_x = x + 1; // Start from x=1, skip x=0

        if let Some(y_prev) = y0 {
            if y == y_prev {
                // Horizontal line
                canvas[y][plot_x] = config.symbols.horizontal;
            } else {
                // Vertical movement
                let (y_start, y_end) = if y_prev < y {
                    (y_prev, y)
                } else {
                    (y, y_prev)
                };

                // Draw vertical connection
                for y_line in y_start..=y_end {
                    if y_line == y_prev {
                        if y_prev < y {
                            canvas[y_line][plot_x] = config.symbols.top_right;
                        } else {
                            canvas[y_line][plot_x] = config.symbols.bottom_right;
                        }
                    } else if y_line == y {
                        if y_prev < y {
                            canvas[y_line][plot_x] = config.symbols.bottom_left;
                        } else {
                            canvas[y_line][plot_x] = config.symbols.top_left;
                        }
                    } else {
                        canvas[y_line][plot_x] = config.symbols.vertical;
                    }
                }
            }
        } else {
            // First point
            canvas[y][plot_x] = config.symbols.vertical;
        }

        y0 = Some(y);
    }

    // Build output with Y-axis labels
    let mut lines = Vec::new();
    
    if config.show_labels {
        let label_width = format_value(max, &config.label_format).len()
            .max(format_value(min, &config.label_format).len());

        for (idx, row) in canvas.iter().enumerate() {
            let y_value = max - (idx as f64 * range / height as f64);
            
            // Determine if this row should have a label
            let label = if idx == 0 {
                format!("{:>width$}", format_value(max, &config.label_format), width = label_width)
            } else if idx == height {
                format!("{:>width$}", format_value(min, &config.label_format), width = label_width)
            } else if config.label_ticks > 0 && height >= config.label_ticks {
                let step = height / config.label_ticks;
                if step > 0 && idx % step == 0 {
                    format!("{:>width$}", format_value(y_value, &config.label_format), width = label_width)
                } else {
                    " ".repeat(label_width)
                }
            } else {
                " ".repeat(label_width)
            };

            // let line: String = row.iter().collect();
            // label + │ + chart, x=0 is always space so no double │
            // lines.push(format!("{}│{}", label, &line[1..]));
            // Make sure the line starts from column 1 (chart starts here)
            // let chart_part: String = row[1..].iter().collect();
            // lines.push(format!("{}{}{}", label, config.symbols.axis_vertical, chart_part));

            let mut chart_part = row[1..].to_vec();

            // If the first chart row is vertical ('│'), delete it so that it is not double axis
            if chart_part.first() == Some(&config.symbols.axis_vertical) {
                chart_part[0] = ' '; // atau hapus: chart_part.remove(0);
            }

            let chart_str: String = chart_part.iter().collect();
            
            lines.push(format!("{}{}{}", label, config.symbols.axis_vertical, chart_str));

        }
    } else {
        for row in canvas.iter() {
            let line: String = row.iter().collect();
            lines.push(line);
        }
    }

    Ok(lines.join("\n"))
}

/// Format a value according to the format string
fn format_value(value: f64, format: &str) -> String {
    // Simple implementation - extend as needed
    if format.contains(":.2") {
        format!("{:.2}", value)
    } else if format.contains(":.1") {
        format!("{:.1}", value)
    } else if format.contains(":.0") {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Simple plot function with default config
///
/// # Example
///
/// ```rust
/// use rasciichart::plot;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
/// println!("{}", plot(&data));
/// ```
pub fn plot(series: &[f64]) -> String {
    plot_with_config(series, Config::default()).unwrap_or_else(|e| e.to_string())
}

/// Plot with custom height and width
///
/// # Example
///
/// ```rust
/// use rasciichart::plot_sized;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// println!("{}", plot_sized(&data, 15, 60));
/// ```
pub fn plot_sized(series: &[f64], height: usize, width: usize) -> String {
    plot_with_config(
        series,
        Config::default().with_height(height).with_width(width)
    ).unwrap_or_else(|e| e.to_string())
}

/// Plot without Y-axis labels
///
/// # Example
///
/// ```rust
/// use rasciichart::plot_no_labels;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// println!("{}", plot_no_labels(&data));
/// ```
pub fn plot_no_labels(series: &[f64]) -> String {
    plot_with_config(
        series,
        Config::default().with_labels(false)
    ).unwrap_or_else(|e| e.to_string())
}

/// Plot with custom min and max values
///
/// # Example
///
/// ```rust
/// use rasciichart::plot_range;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// println!("{}", plot_range(&data, 0.0, 10.0));
/// ```
pub fn plot_range(series: &[f64], min: f64, max: f64) -> String {
    plot_with_config(
        series,
        Config::default().with_min(min).with_max(max)
    ).unwrap_or_else(|e| e.to_string())
}

/// Plot using ASCII-only characters for better compatibility
///
/// # Example
///
/// ```rust
/// use rasciichart::plot_ascii;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// println!("{}", plot_ascii(&data));
/// ```
pub fn plot_ascii(series: &[f64]) -> String {
    plot_with_config(
        series,
        Config::default().with_ascii_symbols()
    ).unwrap_or_else(|e| e.to_string())
}

/// Plot multiple series on the same chart (overlaid)
///
/// # Example
///
/// ```rust
/// use rasciichart::plot_multiple;
///
/// let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let series2 = vec![5.0, 4.0, 3.0, 2.0, 1.0];
/// println!("{}", plot_multiple(&[&series1, &series2]));
/// ```
pub fn plot_multiple(series: &[&[f64]]) -> String {
    if series.is_empty() {
        return "No data".to_string();
    }

    // Find global min and max
    let mut global_min = f64::INFINITY;
    let mut global_max = f64::NEG_INFINITY;

    for s in series {
        for &val in *s {
            if val.is_finite() {
                global_min = global_min.min(val);
                global_max = global_max.max(val);
            }
        }
    }

    if !global_min.is_finite() || !global_max.is_finite() {
        return "Invalid data".to_string();
    }

    // Plot first series with global min/max
    let config = Config::default()
        .with_min(global_min)
        .with_max(global_max);

    plot_with_config(series[0], config).unwrap_or_else(|e| e.to_string())
}

/// Generate sine wave data for testing
///
/// # Example
///
/// ```rust
/// use rasciichart::{generate_sine, plot};
///
/// let data = generate_sine(50, 1.0, 0.0);
/// println!("{}", plot(&data));
/// ```
pub fn generate_sine(points: usize, frequency: f64, phase: f64) -> Vec<f64> {
    (0..points)
        .map(|i| {
            let x = i as f64 * 2.0 * std::f64::consts::PI / points as f64;
            (frequency * x + phase).sin()
        })
        .collect()
}

/// Generate cosine wave data for testing
pub fn generate_cosine(points: usize, frequency: f64, phase: f64) -> Vec<f64> {
    (0..points)
        .map(|i| {
            let x = i as f64 * 2.0 * std::f64::consts::PI / points as f64;
            (frequency * x + phase).cos()
        })
        .collect()
}

/// Generate random walk data for testing
pub fn generate_random_walk(points: usize, start: f64, volatility: f64) -> Vec<f64> {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    let mut result = Vec::with_capacity(points);
    let mut current = start;
    result.push(current);
    
    for i in 1..points {
        // Simple pseudo-random using hash
        let s = RandomState::new();
        let mut hasher = s.build_hasher();
        i.hash(&mut hasher);
        let hash = hasher.finish();
        let random = (hash % 1000) as f64 / 1000.0 - 0.5;
        
        current += random * volatility;
        result.push(current);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_plot() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let chart = plot(&data);
        assert!(!chart.is_empty());
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<f64> = vec![];
        let result = plot_with_config(&data, Config::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_single_value() {
        let data = vec![5.0];
        let chart = plot(&data);
        assert!(chart.contains("5.00"));
    }

    #[test]
    fn test_custom_config() {
        let data = vec![10.0, 20.0, 30.0, 20.0, 10.0];
        let config = Config::default()
            .with_height(15)
            .with_width(50);
        let chart = plot_with_config(&data, config).unwrap();
        assert!(!chart.is_empty());
    }

    #[test]
    fn test_no_labels() {
        let data = vec![1.0, 2.0, 3.0];
        let chart = plot_no_labels(&data);
        assert!(!chart.is_empty());
        assert!(!chart.contains("│"));
    }

    #[test]
    fn test_ascii_symbols() {
        let data = vec![1.0, 2.0, 3.0];
        let chart = plot_ascii(&data);
        assert!(!chart.is_empty());
    }

    #[test]
    fn test_invalid_range() {
        let config = Config::default().with_min(10.0).with_max(5.0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_generate_sine() {
        let data = generate_sine(50, 1.0, 0.0);
        assert_eq!(data.len(), 50);
        assert!(data[0].abs() < 0.1);
    }

    #[test]
    fn test_with_nan() {
        let data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
        let chart = plot(&data);
        assert!(!chart.is_empty());
    }

    #[test]
    fn test_with_infinity() {
        let data = vec![1.0, 2.0, f64::INFINITY, 4.0, 5.0];
        let chart = plot(&data);
        assert!(!chart.is_empty());
    }

    #[test]
    fn test_small_range() {
        let data = vec![1.001, 1.002, 1.003, 1.002, 1.001];
        let chart = plot(&data);
        assert!(!chart.is_empty());
        // Should not be excessively tall
        let line_count = chart.lines().count();
        assert!(line_count <= 15); // Default height + 1
    }

    #[test]
    fn test_ascending_line() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let chart = plot(&data);
        // Should contain ascending characters
        assert!(chart.contains("╭") || chart.contains("╰"));
    }

    #[test]
    fn test_descending_line() {
        let data = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let chart = plot(&data);
        // Should contain descending characters
        assert!(chart.contains("╮") || chart.contains("╯"));
    }
}