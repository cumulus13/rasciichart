// tests/integration_test.rs
// Integration tests for rasciichart

use rasciichart::*;

#[test]
fn test_basic_plotting() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
    assert!(chart.contains("│")); // Should have axis
}

#[test]
fn test_custom_size_plotting() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let chart = plot_sized(&data, 20, 80);
    
    assert!(!chart.is_empty());
    let lines: Vec<&str> = chart.lines().collect();
    assert!(lines.len() >= 20); // Should have at least height lines
}

#[test]
fn test_no_labels_plotting() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let chart = plot_no_labels(&data);
    
    assert!(!chart.is_empty());
    // Without labels there is no axis character
    assert!(!chart.contains("│"));
}

#[test]
fn test_range_plotting() {
    let data = vec![5.0, 6.0, 7.0];
    let chart = plot_range(&data, 0.0, 10.0);
    
    assert!(!chart.is_empty());
    // Should contain 0 and 10 in labels
    assert!(chart.contains("0.00") || chart.contains("10.00"));
}

#[test]
fn test_ascii_plotting() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let chart = plot_ascii(&data);
    
    assert!(!chart.is_empty());
    // Should contain ASCII characters instead of Unicode
    assert!(chart.contains("-") || chart.contains("|") || chart.contains("+"));
}

#[test]
fn test_generate_sine() {
    let data = generate_sine(100, 1.0, 0.0);
    
    assert_eq!(data.len(), 100);
    // First value should be close to 0
    assert!(data[0].abs() < 0.1);
    // Values should be between -1 and 1
    for &val in &data {
        assert!(val >= -1.1 && val <= 1.1);
    }
}

#[test]
fn test_generate_cosine() {
    let data = generate_cosine(100, 1.0, 0.0);
    
    assert_eq!(data.len(), 100);
    // First value should be close to 1
    assert!((data[0] - 1.0).abs() < 0.1);
}

#[test]
fn test_generate_random_walk() {
    let data = generate_random_walk(50, 100.0, 1.0);
    
    assert_eq!(data.len(), 50);
    assert_eq!(data[0], 100.0); // Should start at initial value
}

#[test]
fn test_config_validation() {
    // Valid config
    let config = Config::new()
        .with_height(10)
        .with_width(80);
    assert!(config.validate().is_ok());
    
    // Invalid config - zero height
    let config = Config::new()
        .with_height(0)
        .with_width(80);
    assert!(config.validate().is_err());
    
    // Invalid config - zero width
    let config = Config::new()
        .with_height(10)
        .with_width(0);
    assert!(config.validate().is_err());
    
    // Invalid config - bad range
    let config = Config::new()
        .with_min(10.0)
        .with_max(5.0);
    assert!(config.validate().is_err());
}

#[test]
fn test_error_handling() {
    // Empty data
    let empty: Vec<f64> = vec![];
    let result = plot_with_config(&empty, Config::new());
    assert!(result.is_err());
    
    // Invalid dimensions
    let data = vec![1.0, 2.0, 3.0];
    let config = Config::new().with_height(0);
    let result = plot_with_config(&data, config);
    assert!(result.is_err());
}

#[test]
fn test_single_value() {
    let data = vec![42.0];
    let chart = plot(&data);
    
    assert!(chart.contains("42.00"));
}

#[test]
fn test_flat_line() {
    let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
    let chart = plot(&data);
    
    assert!(chart.contains("5.00"));
}

#[test]
fn test_negative_values() {
    let data = vec![-5.0, -2.0, 0.0, 2.0, 5.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
    assert!(chart.contains("-")); // Should have negative values
}

#[test]
fn test_large_values() {
    let data = vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_small_values() {
    let data = vec![0.001, 0.002, 0.003, 0.004, 0.005];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_mixed_positive_negative() {
    let data = vec![-10.0, -5.0, 0.0, 5.0, 10.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_spike_detection() {
    let data = vec![1.0, 1.0, 1.0, 100.0, 1.0, 1.0, 1.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
    // Should have vertical characters for the spike
    assert!(chart.contains("│") || chart.contains("╯") || chart.contains("╰"));
}

#[test]
fn test_gradual_increase() {
    let data: Vec<f64> = (0..20).map(|x| x as f64).collect();
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_step_function() {
    let data = vec![1.0, 1.0, 5.0, 5.0, 2.0, 2.0, 8.0, 8.0];
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_oscillating_data() {
    let data: Vec<f64> = (0..50)
        .map(|x| (x as f64 * 0.2).sin() * 10.0)
        .collect();
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_exponential_growth() {
    let data: Vec<f64> = (0..20)
        .map(|x| 2.0f64.powi(x))
        .collect();
    let chart = plot(&data);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_custom_symbols() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let symbols = Symbols::ascii();
    let config = Config::new().with_symbols(symbols);
    
    let result = plot_with_config(&data, config);
    assert!(result.is_ok());
    
    let chart = result.unwrap();
    // Should contain ASCII characters
    assert!(chart.contains("-") || chart.contains("+"));
}

#[test]
fn test_label_formatting() {
    let data = vec![1.234, 2.567, 3.891];
    let config = Config::new()
        .with_label_format("{:.2}".to_string());
    
    let result = plot_with_config(&data, config);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_series() {
    let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let series2 = vec![5.0, 4.0, 3.0, 2.0, 1.0];
    
    let chart = plot_multiple(&[&series1, &series2]);
    assert!(!chart.is_empty());
}

#[test]
fn test_with_nan_values() {
    let data = vec![1.0, 2.0, f64::NAN, 4.0, 5.0];
    let chart = plot(&data);
    
    // Should handle NaN gracefully
    assert!(!chart.is_empty());
}

#[test]
fn test_with_infinity() {
    let data = vec![1.0, 2.0, f64::INFINITY, 4.0, 5.0];
    let chart = plot(&data);
    
    // Should handle Infinity gracefully
    assert!(!chart.is_empty());
}

#[test]
fn test_very_wide_chart() {
    let data: Vec<f64> = (0..200).map(|x| (x as f64).sin()).collect();
    let chart = plot_sized(&data, 15, 200);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_very_tall_chart() {
    let data = vec![1.0, 5.0, 2.0, 8.0, 3.0];
    let chart = plot_sized(&data, 50, 30);
    
    assert!(!chart.is_empty());
}

#[test]
fn test_label_ticks_configuration() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    // Few ticks
    let config = Config::new().with_label_ticks(3);
    let result = plot_with_config(&data, config);
    assert!(result.is_ok());
    
    // Many ticks
    let config = Config::new().with_label_ticks(10);
    let result = plot_with_config(&data, config);
    assert!(result.is_ok());
}

#[test]
fn test_builder_pattern() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    let config = Config::new()
        .with_height(20)
        .with_width(80)
        .with_min(0.0)
        .with_max(10.0)
        .with_labels(true)
        .with_label_ticks(5)
        .with_label_format("{:.2}".to_string())
        .with_offset(3);
    
    let result = plot_with_config(&data, config);
    assert!(result.is_ok());
}