// examples/simple.rs
// Basic usage examples

use rasciichart::*;

fn main() {
    println!("=== rasciichart - Simple Examples ===\n");

    // Example 1: Basic plot
    println!("1. Basic Plot:");
    let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    println!("{}\n", plot(&data1));

    // Example 2: Larger dataset
    println!("2. Larger Dataset:");
    let data2 = vec![
        1.0, 3.0, 2.0, 4.0, 3.0, 5.0, 4.0, 6.0, 5.0, 7.0,
        6.0, 8.0, 7.0, 9.0, 8.0, 10.0, 9.0, 8.0, 7.0, 6.0
    ];
    println!("{}\n", plot(&data2));

    // Example 3: Custom size
    println!("3. Custom Size (height=15, width=60):");
    println!("{}\n", plot_sized(&data1, 15, 60));

    // Example 4: Without labels
    println!("4. Without Y-axis Labels:");
    println!("{}\n", plot_no_labels(&data1));

    // Example 5: ASCII-only
    println!("5. ASCII-only Characters:");
    println!("{}\n", plot_ascii(&data1));

    // Example 6: Custom range
    println!("6. Custom Range (0 to 10):");
    let data3 = vec![3.0, 4.0, 5.0, 6.0, 7.0];
    println!("{}\n", plot_range(&data3, 0.0, 10.0));

    // Example 7: Flat line
    println!("7. Flat Line:");
    let data4 = vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0];
    println!("{}\n", plot(&data4));

    // Example 8: Single spike
    println!("8. Single Spike:");
    let data5 = vec![1.0, 1.0, 1.0, 10.0, 1.0, 1.0, 1.0];
    println!("{}\n", plot(&data5));

    // Example 9: Gradual increase
    println!("9. Gradual Increase:");
    let data6: Vec<f64> = (0..20).map(|x| x as f64 * 0.5).collect();
    println!("{}\n", plot(&data6));

    // Example 10: Step function
    println!("10. Step Function:");
    let data7 = vec![
        1.0, 1.0, 1.0, 1.0, 
        5.0, 5.0, 5.0, 5.0,
        2.0, 2.0, 2.0, 2.0,
        8.0, 8.0, 8.0, 8.0
    ];
    println!("{}\n", plot(&data7));
}