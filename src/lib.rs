// In the sprawl of identical terminals, your mark matters. Every run gets a face,
// random font-face and color bleeding different each time. They can copy the code,
// but they can't copy the signature. That's yours.

mod fonts;

use colored::*;
use figlet_rs::FIGfont;

/// A gradient theme used to color rendered ASCII-art titles.
///
/// [`Gradient::Random`] preserves the original behavior: a fresh palette of
/// 2–5 vibrant HSV colors picked anew on every render. The named presets use
/// fixed, hand-tuned color stops, and [`Gradient::Custom`] lets you supply your
/// own RGB stops.
///
/// # Examples
///
/// ```
/// use playbill::Gradient;
///
/// let title = playbill::generate_title_with_gradient("PLAYBILL", None, Gradient::Cyberpunk);
/// assert!(!title.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    /// The original behavior: 2–5 random vibrant HSV colors, fresh each render.
    Random,
    /// Pink → Cyan → Purple.
    Cyberpunk,
    /// Soft pink → Light blue → Lavender.
    Pastel,
    /// Bright green → Hot pink → Electric blue.
    Neon,
    /// Orange → Magenta → Yellow.
    Retro,
    /// Deep blue → Blue → Turquoise → Aqua.
    Ocean,
    /// Orange → Pink → Purple.
    Sunset,
    /// Dark green → Light green → Mint.
    Forest,
    /// Red → Orange → Yellow.
    Fire,
    /// Aqua → Light blue → Pale cyan.
    Ice,
    /// User-supplied RGB color stops.
    Custom(Vec<(u8, u8, u8)>),
}

pub fn print_title(text: &str, version: Option<&str>) {
    print_title_with_gradient(text, version, Gradient::Random);
}

pub fn generate_title(text: &str, version: Option<&str>) -> String {
    generate_title_with_gradient(text, version, Gradient::Random)
}

/// Print a rendered title using the given [`Gradient`].
///
/// # Examples
///
/// ```no_run
/// use playbill::Gradient;
///
/// playbill::print_title_with_gradient("MyApp", Some("1.0.0"), Gradient::Cyberpunk);
/// ```
pub fn print_title_with_gradient(text: &str, version: Option<&str>, gradient: Gradient) {
    let title = generate_title_with_gradient(text, version, gradient);
    print!("{}", title);
}

/// Render a title to a `String` using the given [`Gradient`].
///
/// Falls back from the chosen random font to the standard figlet font, and
/// finally to plain text, if font rendering fails.
///
/// # Examples
///
/// ```
/// use playbill::Gradient;
///
/// let title = playbill::generate_title_with_gradient("PLAYBILL", None, Gradient::Ocean);
/// assert!(!title.is_empty());
/// ```
pub fn generate_title_with_gradient(
    text: &str,
    version: Option<&str>,
    gradient: Gradient,
) -> String {
    let (_font_name, font_bytes) = fonts::random_font();
    if let Ok(decompressed) = fonts::decompress_font(font_bytes) {
        let font_str = String::from_utf8_lossy(&decompressed);
        if let Ok(font) = FIGfont::from_content(&font_str) {
            if let Some(figure) = font.convert(text) {
                let mut output = generate_gradient_text(&figure.to_string(), &gradient);
                if let Some(v) = version {
                    output.push_str(&format!("{} v{}\n", text, v));
                } else {
                    output.push('\n');
                }
                return output;
            }
        }
    }

    // Fallback to standard font if something goes wrong
    if let Ok(standard_font) = FIGfont::standard() {
        if let Some(figure) = standard_font.convert(text) {
            let mut output = generate_gradient_text(&figure.to_string(), &gradient);
            if let Some(v) = version {
                output.push_str(&format!("{} v{}\n", text, v));
            } else {
                output.push('\n');
            }
            return output;
        }
    }

    // Ultimate fallback
    format!("{}\n", text)
}

fn preset_stops(gradient: &Gradient) -> &'static [&'static str] {
    match gradient {
        Gradient::Cyberpunk => &["#ff006e", "#00f0ff", "#8338ec"],
        Gradient::Pastel => &["#ffc8dd", "#a2d2ff", "#cdb4db"],
        Gradient::Neon => &["#39ff14", "#ff10f0", "#04d9ff"],
        Gradient::Retro => &["#ff8500", "#ff006e", "#ffd60a"],
        Gradient::Ocean => &["#03045e", "#0077b6", "#00b4d8", "#90e0ef"],
        Gradient::Sunset => &["#ff7b00", "#ff5e78", "#9d4edd"],
        Gradient::Forest => &["#1b4332", "#40916c", "#95d5b2"],
        Gradient::Fire => &["#d00000", "#ff6000", "#ffba08"],
        Gradient::Ice => &["#00b4d8", "#90e0ef", "#caf0f8"],
        // Random and Custom build their colors elsewhere.
        Gradient::Random | Gradient::Custom(_) => &[],
    }
}

fn build_colors(gradient: &Gradient) -> Vec<colorgrad::Color> {
    use colorgrad::Color;
    use rand::Rng;

    match gradient {
        Gradient::Random => {
            let mut rng = rand::rng();
            let num_colors = rng.random_range(2..=5);
            let mut colors = Vec::new();
            for _ in 0..num_colors {
                // Generate vibrant colors by using HSV color space
                let hue = rng.random_range(0.0..360.0);
                let saturation = rng.random_range(0.6..1.0); // High saturation for vibrant colors
                let value = rng.random_range(0.7..1.0); // High value for bright colors
                colors.push(Color::from_hsva(hue, saturation, value, 1.0));
            }
            colors
        }
        Gradient::Custom(stops) => stops
            .iter()
            .map(|&(r, g, b)| Color::from_rgba8(r, g, b, 255))
            .collect(),
        named => preset_stops(named)
            .iter()
            .filter_map(|hex| Color::from_html(hex).ok())
            .collect(),
    }
}

fn generate_gradient_text(text: &str, gradient_kind: &Gradient) -> String {
    use colorgrad::{Color, Gradient, GradientBuilder};

    let colors = build_colors(gradient_kind);

    let gradient = GradientBuilder::new()
        .colors(&colors)
        .build::<colorgrad::LinearGradient>()
        .unwrap_or_else(|_| {
            GradientBuilder::new()
                .colors(&[
                    Color::from_html("#ff6b6b").unwrap(),
                    Color::from_html("#4ecdc4").unwrap(),
                ])
                .build::<colorgrad::LinearGradient>()
                .unwrap()
        });

    let lines: Vec<&str> = text.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Find the maximum line width
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    if max_width == 0 {
        return text.to_string();
    }

    let mut output = String::new();

    // Generate each line with gradient applied horizontally
    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            let t = i as f32 / max_width as f32;
            let color = gradient.at(t);
            let rgba = color.to_rgba8();
            output.push_str(
                &ch.to_string()
                    .truecolor(rgba[0], rgba[1], rgba[2])
                    .to_string(),
            );
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRUECOLOR_ESCAPE: &str = "\x1b[38;2;";

    // Under the test harness stdout isn't a TTY (so `colored` suppresses codes)
    // and `COLORTERM` is usually unset (so `colored` downgrades truecolor to the
    // nearest 16-color code). Force both on so the escape assertions hold
    // regardless of where the suite runs.
    fn force_color() {
        std::env::set_var("COLORTERM", "truecolor");
        colored::control::set_override(true);
    }

    fn assert_renders_with_color(title: &str) {
        assert!(!title.is_empty(), "rendered title should not be empty");
        assert!(
            title.contains(TRUECOLOR_ESCAPE),
            "rendered title should contain ANSI truecolor escapes"
        );
    }

    #[test]
    fn presets_render_with_truecolor() {
        force_color();
        for gradient in [
            Gradient::Cyberpunk,
            Gradient::Pastel,
            Gradient::Neon,
            Gradient::Retro,
            Gradient::Ocean,
            Gradient::Sunset,
            Gradient::Forest,
            Gradient::Fire,
            Gradient::Ice,
        ] {
            let title = generate_title_with_gradient("PLAYBILL", None, gradient);
            assert_renders_with_color(&title);
        }
    }

    #[test]
    fn random_renders_with_truecolor() {
        force_color();
        let title = generate_title_with_gradient("PLAYBILL", Some("1.0.0"), Gradient::Random);
        assert_renders_with_color(&title);
    }

    #[test]
    fn custom_renders_with_truecolor() {
        force_color();
        let stops = vec![(255, 0, 110), (0, 240, 255), (131, 56, 236)];
        let title = generate_title_with_gradient("PLAYBILL", None, Gradient::Custom(stops));
        assert_renders_with_color(&title);
    }

    #[test]
    fn default_helpers_still_work() {
        force_color();
        let title = generate_title("PLAYBILL", Some("0.1.0"));
        assert_renders_with_color(&title);
    }
}
