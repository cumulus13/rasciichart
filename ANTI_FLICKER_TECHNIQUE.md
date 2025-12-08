# Anti-Flicker Technique for Real-time Updates

## 🎯 The Problem

Terminal flicker occurs when you **clear the screen and then write content separately**.

The key insight from experts: **"Don't send one thing and then overwrite it with something else (within a single 'frame' of drawing)"**

## ❌ WRONG Approach (Causes Flicker)

```rust
// BAD - This WILL flicker!
print!("\x1B[2J\x1B[H");  // Clear screen first
println!("Title");         // Then write content
println!("{}", chart);     // More content
io::stdout().flush().unwrap();

// Problem: Terminal shows blank screen between clear and content!
```

## ✅ CORRECT Approach (Zero Flicker)

Build all output (including clear command) in a single buffer before displaying:

```rust
// GOOD - No flicker!
let output = format!(
    "\x1B[2J\x1B[HTitle\n{}",
    chart
);
print!("{}", output);     // Single write operation!
io::stdout().flush().unwrap();
```

**Why this works**: The terminal receives everything in one write() syscall, so there's no visible blank frame.

## 📊 The Science

Terminal emulators buffer output and update the screen per write operation. Multiple writes = multiple screen updates = flicker.

**Solution**: Use concept of double buffering - get all output before clearing, so minimum time between clearing and redrawing.

## 🔧 Advanced Techniques

### Technique 1: Double Buffering

For even smoother updates:

```rust
struct DoubleBuffer {
    front: String,
    back: String,
}

impl DoubleBuffer {
    fn swap(&mut self) {
        std::mem::swap(&mut self.front, &mut self.back);
    }
    
    fn render(&mut self, content: &str) {
        self.back.clear();
        self.back.push_str(content);
        self.swap();
        print!("{}", self.front);
        io::stdout().flush().unwrap();
    }
}
```

### Technique 2: Selective Updates

Only update what changed:

```rust
fn render_selective(old_screen: &str, new_screen: &str) {
    // Compare line by line
    let old_lines: Vec<_> = old_screen.lines().collect();
    let new_lines: Vec<_> = new_screen.lines().collect();
    
    for (i, (old, new)) in old_lines.iter().zip(&new_lines).enumerate() {
        if old != new {
            // Move cursor to line i and update
            print!("\x1B[{};1H{}", i + 1, new);
        }
    }
    io::stdout().flush().unwrap();
}
```

### Technique 3: Terminal Capabilities

Use `termion` or similar for better control:

```rust
use std::io::Write;

fn render_with_termion(content: &str) {
    let mut stdout = io::stdout();
    
    // Save cursor, clear, write, restore
    write!(stdout, 
        "\x1B[s\x1B[2J\x1B[H{}\x1B[u", 
        content
    ).unwrap();
    
    stdout.flush().unwrap();
}
```

## 🎨 Visual Comparison

### Before (Multiple Writes)
```
Time: 0ms   [Clear screen]
Time: 5ms   [Title appears]    ← Flicker!
Time: 10ms  [Subtitle appears] ← Flicker!
Time: 15ms  [Chart appears]    ← Flicker!
```

### After (Buffer-First)
```
Time: 0ms   [Build buffer...]
Time: 15ms  [Everything appears] ← Smooth!
```

## 📝 Best Practices

### DO ✅
- Pre-allocate buffer with sufficient capacity
- Build entire frame before displaying
- Use single write + flush
- Clear and reset cursor in one escape sequence

### DON'T ❌
- Multiple print statements
- Print-as-you-go
- Forget to flush
- Use aggressive clear commands

## 🧪 Testing Anti-Flicker

### Visual Test
Run this and watch for flicker:

```rust
// Good test - should be smooth
for i in 0..100 {
    let mut output = String::with_capacity(1024);
    output.push_str("\x1B[2J\x1B[H");
    output.push_str(&format!("Frame: {}\n", i));
    output.push_str(&"█".repeat(i));
    print!("{}", output);
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(50));
}
```

### Objective Test
Measure frame consistency:

```rust
let start = Instant::now();
for _ in 0..60 {
    render_screen("Test", "Frame", "Content");
    thread::sleep(Duration::from_millis(16)); // 60fps
}
let elapsed = start.elapsed();
println!("60 frames in {:?}", elapsed);
// Should be close to 960ms (60 * 16)
```

## 🎯 Real-World Example

### Bad (Flickery)
```rust
loop {
    print!("\x1B[2J");           // Clear
    println!("CPU: {}%", get_cpu());
    println!("{}", get_chart());
    thread::sleep(Duration::from_millis(100));
}
```

### Good (Smooth)
```rust
loop {
    let mut output = String::with_capacity(2048);
    output.push_str("\x1B[2J\x1B[H");
    output.push_str(&format!("CPU: {}%\n", get_cpu()));
    output.push_str(&get_chart());
    print!("{}", output);
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(100));
}
```

## 📚 Terminal Escape Codes

Commonly used codes:

| Code | Effect |
|------|--------|
| `\x1B[2J` | Clear entire screen |
| `\x1B[H` | Move cursor to home (0,0) |
| `\x1B[{row};{col}H` | Move cursor to position |
| `\x1B[K` | Clear from cursor to end of line |
| `\x1B[s` | Save cursor position |
| `\x1B[u` | Restore cursor position |

### Optimal Clear + Reset
```rust
"\x1B[2J\x1B[H"  // Clear + Home in one sequence
```

## 🚀 Performance Tips

1. **Pre-allocate**: `String::with_capacity(2048)`
2. **Reuse buffers**: Don't allocate every frame
3. **Minimize allocations**: Use `push_str` not `format!`
4. **Batch updates**: Update once per frame, not per change

## 🎬 Frame Rate Considerations

| FPS | Frame Time | Use Case |
|-----|-----------|----------|
| 60 | 16ms | Smooth animations |
| 30 | 33ms | Standard updates |
| 10 | 100ms | Data monitoring |
| 1 | 1000ms | Slow updates |

**Rule**: Buffer time should be < frame time for smooth results.

## 🔍 Debugging Flicker

If you still see flicker:

1. **Check terminal type**: Some terminals handle updates differently
2. **Measure timing**: Are updates too fast?
3. **Test on different terminals**: PowerShell vs bash vs zsh
4. **Profile buffer allocation**: Is it allocating too much?

## ✨ Final Tips

- **Always flush!** Don't forget `stdout().flush()`
- **Test on target platform** - Windows/Linux/Mac differ
- **Use raw mode** for maximum control
- **Consider async** for non-blocking updates
- **Profile first** - measure before optimizing

## 📖 Further Reading

- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [Terminal Control Sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.html)
- [Rust stdout documentation](https://doc.rust-lang.org/std/io/struct.Stdout.html)

---

**Summary**: Build entire screen in buffer → Single write → Single flush = ZERO FLICKER! ✨

**Version**: 0.2.3  
**Last Updated**: December 8, 2024