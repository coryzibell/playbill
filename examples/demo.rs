use playbill::Gradient;

fn main() {
    // Default behavior: a fresh random HSV gradient each run.
    playbill::print_title("playbill", Some("0.1.0"));

    println!("\nGradient presets:\n");

    for (name, gradient) in [
        ("Cyberpunk", Gradient::Cyberpunk),
        ("Pastel", Gradient::Pastel),
        ("Neon", Gradient::Neon),
        ("Retro", Gradient::Retro),
        ("Ocean", Gradient::Ocean),
        ("Sunset", Gradient::Sunset),
        ("Forest", Gradient::Forest),
        ("Fire", Gradient::Fire),
        ("Ice", Gradient::Ice),
    ] {
        println!("== {name} ==");
        playbill::print_title_with_gradient("PLAYBILL", None, gradient);
        println!();
    }

    println!("== Custom ==");
    let custom = Gradient::Custom(vec![(255, 0, 110), (0, 240, 255), (131, 56, 236)]);
    playbill::print_title_with_gradient("PLAYBILL", None, custom);
}
