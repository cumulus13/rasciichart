// examples/realtime.rs
// Real-time simulation - PROPERLY fixed with NO flicker!

use rasciichart::*;
use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    println!("=== rasciichart - Real-time Simulation ===\n");
    println!("Press Ctrl+C to exit\n");
    thread::sleep(Duration::from_secs(1));

    // Example 1: Scrolling sine wave
    println!("\n1. Scrolling Sine Wave (15 frames):");
    thread::sleep(Duration::from_millis(500));
    
    let mut phase = 0.0;
    for frame in 0..15 {
        let data = generate_sine(60, 2.0, phase);
        let chart = plot_sized(&data, 12, 60);
        
        // IMPORTANT: Build EVERYTHING including clear in ONE string!
        let output = format!(
            "\x1B[2J\x1B[H=== Scrolling Sine Wave ===\nFrame: {} | Phase: {:.2}\n\n{}",
            frame + 1, phase, chart
        );
        
        // Single write!
        print!("{}", output);
        io::stdout().flush().unwrap();
        
        phase += 0.2;
        thread::sleep(Duration::from_millis(150));
    }

    thread::sleep(Duration::from_secs(1));

    // Example 2: Growing data
    println!("\n2. Growing Dataset (20 steps):");
    thread::sleep(Duration::from_millis(500));
    
    let mut data: Vec<f64> = vec![];
    for i in 0..20 {
        data.push((i as f64 * 0.3).sin() * 5.0 + 10.0);
        let chart = plot_sized(&data, 12, 60);
        
        let output = format!(
            "\x1B[2J\x1B[H=== Growing Dataset ===\nData points: {}\n\n{}",
            data.len(), chart
        );
        
        print!("{}", output);
        io::stdout().flush().unwrap();
        
        thread::sleep(Duration::from_millis(200));
    }

    thread::sleep(Duration::from_secs(1));

    // Example 3: Random walk
    println!("\n3. Random Walk (20 steps):");
    thread::sleep(Duration::from_millis(500));
    
    let mut value = 50.0;
    let mut history = vec![value];
    
    for step in 0..20 {
        let change = ((step * 7) % 13) as f64 - 6.0;
        value += change * 0.5;
        history.push(value);
        
        if history.len() > 50 {
            history.remove(0);
        }
        
        let chart = plot_sized(&history, 15, 60);
        
        let output = format!(
            "\x1B[2J\x1B[H=== Random Walk ===\nStep: {} | Current: {:.2}\n\n{}",
            step + 1, value, chart
        );
        
        print!("{}", output);
        io::stdout().flush().unwrap();
        
        thread::sleep(Duration::from_millis(200));
    }

    thread::sleep(Duration::from_secs(1));

    // Example 4: Oscillating values
    println!("\n4. Damped Oscillation (15 frames):");
    thread::sleep(Duration::from_millis(500));
    
    let mut t = 0.0;
    for frame in 0..15 {
        let data: Vec<f64> = (0..50)
            .map(|x| {
                let phase = (x as f64 * 0.2) + t;
                phase.sin() * 3.0 * (-t * 0.1).exp() + 5.0
            })
            .collect();
        
        let chart = plot_sized(&data, 12, 50);
        
        let output = format!(
            "\x1B[2J\x1B[H=== Damped Oscillation ===\nFrame: {} | Time: {:.2}\n\n{}",
            frame + 1, t, chart
        );
        
        print!("{}", output);
        io::stdout().flush().unwrap();
        
        t += 0.5;
        thread::sleep(Duration::from_millis(250));
    }

    thread::sleep(Duration::from_secs(1));

    // Example 5: CPU usage simulation
    println!("\n5. Simulated CPU Usage Monitor (25 samples):");
    thread::sleep(Duration::from_millis(500));
    
    let mut cpu_history: Vec<f64> = vec![];
    
    for sample in 0..25 {
        let base = 30.0;
        let variation = ((sample * 13) % 40) as f64;
        let spike = if sample % 7 == 0 { 20.0 } else { 0.0 };
        let usage = (base + variation + spike).min(100.0);
        
        cpu_history.push(usage);
        if cpu_history.len() > 50 {
            cpu_history.remove(0);
        }
        
        let config = Config::new()
            .with_height(12)
            .with_width(60)
            .with_min(0.0)
            .with_max(100.0);
        
        let chart = plot_with_config(&cpu_history, config).unwrap_or_default();
        
        let status = if usage > 80.0 {
            "\x1B[31m[HIGH]\x1B[0m"
        } else if usage > 60.0 {
            "\x1B[33m[MEDIUM]\x1B[0m"
        } else {
            "\x1B[32m[NORMAL]\x1B[0m"
        };
        
        let output = format!(
            "\x1B[2J\x1B[H=== CPU Usage Monitor ===\nSample: {} | Current: {:.1}% | Status: {}\n\n{}",
            sample + 1, usage, status, chart
        );
        
        print!("{}", output);
        io::stdout().flush().unwrap();
        
        thread::sleep(Duration::from_millis(150));
    }

    // Final message
    print!("\x1B[2J\x1B[H");
    println!("\n=== Simulation Complete! ===\n");
    println!("All examples finished successfully.");
    println!("Technique: Build entire output (including clear) in single string, then single write!");
    io::stdout().flush().unwrap();
}