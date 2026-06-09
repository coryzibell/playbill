# playbill

ASCII art title generator with random gradient effects.

In the sprawl of identical terminals, your mark matters. Every run gets a face, random font-face and color bleeding different each time. They can copy the code, but they can't copy the signature. That's yours.

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
playbill = "0.1.3"
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

## Examples

Every run produces a different font and color gradient. Here are some examples:

```
::::::::::.  :::       :::.     .-:.     ::-.:::::::.  ::: :::      :::     
 `;;;```.;;; ;;;       ;;`;;     ';;.   ;;;;' ;;;'';;' ;;; ;;;      ;;;     
  `]]nnn]]'  [[[      ,[[ '[[,     '[[,[[['   [[[__[[\.[[[ [[[      [[[     
   $$$""     $$'     c$$$cc$$$c      c$$"     $$""""Y$$$$$ $$'      $$'     
   888o     o88oo,.__ 888   888,   ,8P"`     _88o,,od8P888o88oo,.__o88oo,.__
   YMMMb    """"YUMMM YMM   ""`   mM"        ""YUMMMP" MMM""""YUMMM""""YUMMM
```

```
 ,--.-,,-,--,       ,----.                              _,.---._     
/==/  /|=|  |    ,-.--` , \    _.-.        _.-.       ,-.' , -  `.   
|==|_ ||=|, |   |==|-  _.-`  .-,.'|      .-,.'|      /==/_,  ,  - \  
|==| ,|/=| _|   |==|   `.-. |==|, |     |==|, |     |==|   .=.     | 
|==|- `-' _ |  /==/_ ,    / |==|- |     |==|- |     |==|_ : ;=:  - | 
|==|  _     |  |==|    .-'  |==|, |     |==|, |     |==| , '='     | 
|==|   .-. ,\  |==|_  ,`-._ |==|- `-._  |==|- `-._   \==\ -    ,_ /  
/==/, //=/  |  /==/ ,     / /==/ - , ,/ /==/ - , ,/   '.='. -   .'   
`--`-' `-`--`  `--`-----``  `--`-----'  `--`-----'      `--`--''     
```

```
 _  (`-')            (`-')  _             <-.(`-')    _                          
 \-.(OO )    <-.     (OO ).-/       .->    __( OO)   (_)        <-.       <-.    
 _.'    \  ,--. )    / ,---.    ,--.'  ,-.'-'---.\   ,-(`-')  ,--. )    ,--. )   
(_...--''  |  (`-')  | \ /`.\  (`-')'.'  /| .-. (/   | ( OO)  |  (`-')  |  (`-') 
|  |_.' |  |  |OO )  '-'|_.' | (OO \    / | '-' `.)  |  |  )  |  |OO )  |  |OO ) 
|  .___.' (|  '__ | (|  .-.  |  |  /   /) | /`'.  | (|  |_/  (|  '__ | (|  '__ | 
|  |       |     |'  |  | |  |  `-/   /`  | '--'  /  |  |'->  |     |'  |     |' 
`--'       `-----'   `--' `--'    `--'    `------'   `--'     `-----'   `-----'  
```

```
       __                           ___   ___         _                               
  .'|=|  |    .'|        .'|=|`.   |   | |   |   .'|=| `.   .'|   .'|        .'|      
.'  | |  |  .'  |      .'  | |  `. `.  |_|  .' .'  | | .' .'  | .'  |      .'  |      
|   |=|.'   |   |      |   |=|   |   `.   .'   |   |=|'.  |   | |   |      |   |      
|   |       |   |  ___ |   | |   |    |   |    |   | |  | |   | |   |  ___ |   |  ___ 
|___|       |___|=|_.' |___| |___|    |___|    |___|=|_.' |___| |___|=|_.' |___|=|_.' 
```

## How It Works

The library embeds pre-compressed FIGlet font files (.flf) at compile time using a build script. Fonts are stored as committed zstd blobs in `fonts/compressed/` and decoded at runtime using the pure-Rust `ruzstd` decoder — no C toolchain required. A random font and gradient are selected on each run to create a unique appearance.

Every invocation produces a different visual style - same text, different font and color scheme.

## Adding Custom Fonts / Contributing

Drop new `.flf` (FIGlet) font files in `fonts/`, then regenerate the compressed blobs:

```bash
scripts/compress-fonts.sh
```

This requires the [`zstd` CLI](https://github.com/facebook/zstd) installed locally — a dev-time
tool only, not a build or consumer dependency. Commit both the `.flf` and the generated `.zst`
file from `fonts/compressed/`.

The library builds with zero C dependencies; consumers can cross-compile to targets like
`aarch64-unknown-linux-musl` without a C cross-toolchain.

## Dependencies

- `figlet-rs` - FIGlet font rendering
- `colored` - Terminal color support
- `colorgrad` - Gradient color generation
- `rand` - Random selection
- `ruzstd` - Pure-Rust zstd decompression (no C toolchain required)
- `anyhow` - Error handling

## License

MIT OR Apache-2.0
