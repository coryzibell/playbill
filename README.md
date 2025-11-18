# playbill

ASCII art title generator with random gradient effects.

This library generates beautiful ASCII art titles with randomly selected fonts and color gradients. Perfect for CLI applications that want a unique, eye-catching splash screen.

## Features

- Random font selection from embedded FIGlet fonts
- Random gradient color schemes generated on-the-fly
- Compressed font storage for minimal binary size
- Fallback to standard font if issues occur
- Zero runtime dependencies on external font files

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
playbill = "0.1.0"
```

Or install from a local path:

```toml
[dependencies]
playbill = { path = "../playbill" }
```

## Usage

```rust
use playbill;

fn main() {
    // Print a title with version
    playbill::print_title("MyApp", Some("1.0.0"));
    
    // Print a title without version
    playbill::print_title("Hello", None);
}
```

Run the included example:

```bash
cargo run --example demo
```

## How It Works

The library embeds FIGlet font files (.flf) at compile time using a build script. Fonts are compressed with zstd to minimize binary size. At runtime, a random font and gradient are selected to create a unique appearance for each run.

Every invocation produces a different visual style - same text, different font and color scheme.

## Adding Custom Fonts

Place additional .flf (FIGlet) font files in the `fonts/` directory before building. The build script will automatically include them.

## Dependencies

- `figlet-rs` - FIGlet font rendering
- `colored` - Terminal color support
- `colorgrad` - Gradient color generation
- `rand` - Random selection
- `zstd` - Font compression
- `anyhow` - Error handling

## License

MIT OR Apache-2.0
