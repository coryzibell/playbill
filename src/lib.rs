// In the sprawl of identical terminals, your mark matters. Every run gets a face, 
// random font-face and color bleeding different each time. They can copy the code,
// but they can't copy the signature. That's yours.

mod fonts;

use figlet_rs::FIGfont;
use colored::*;

pub fn print_title(text: &str, version: Option<&str>) {
    let (_font_name, font_bytes) = fonts::random_font();
    if let Ok(decompressed) = fonts::decompress_font(font_bytes) {
        let font_str = String::from_utf8_lossy(&decompressed);
        match FIGfont::from_content(&font_str) {
            Ok(font) => {
                if let Some(figure) = font.convert(text) {
                    print_gradient_text(&figure.to_string());
                    if let Some(v) = version {
                        println!("{} v{}\n", text, v);
                    } else {
                        println!();
                    }
                    return;
                }
            }
            Err(_) => {}
        }
    }
    
    // Fallback to standard font if something goes wrong
    if let Ok(standard_font) = FIGfont::standard() {
        if let Some(figure) = standard_font.convert(text) {
            print_gradient_text(&figure.to_string());
            if let Some(v) = version {
                println!("{} v{}\n", text, v);
            } else {
                println!();
            }
        }
    }
}

fn print_gradient_text(text: &str) {
    use colorgrad::{Color, GradientBuilder, Gradient};
    use rand::Rng;
    
    let mut rng = rand::rng();
    
    // Generate a random gradient with 2-5 colors
    let num_colors = rng.random_range(2..=5);
    let mut colors = Vec::new();
    
    for _ in 0..num_colors {
        // Generate vibrant colors by using HSV color space
        let hue = rng.random_range(0.0..360.0);
        let saturation = rng.random_range(0.6..1.0);  // High saturation for vibrant colors
        let value = rng.random_range(0.7..1.0);       // High value for bright colors
        
        let color = Color::from_hsva(hue, saturation, value, 1.0);
        colors.push(color);
    }
    
    let gradient = GradientBuilder::new()
        .colors(&colors)
        .build::<colorgrad::LinearGradient>()
        .unwrap_or_else(|_| GradientBuilder::new()
            .colors(&[Color::from_html("#ff6b6b").unwrap(), Color::from_html("#4ecdc4").unwrap()])
            .build::<colorgrad::LinearGradient>()
            .unwrap());
    
    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return;
    }
    
    // Find the maximum line width
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    
    if max_width == 0 {
        println!("{}", text);
        return;
    }
    
    // Print each line with gradient applied horizontally
    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            let t = i as f32 / max_width as f32;
            let color = gradient.at(t);
            let rgba = color.to_rgba8();
            print!("{}", ch.to_string().truecolor(rgba[0], rgba[1], rgba[2]));
        }
        println!();
    }
}

