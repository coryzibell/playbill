use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_fonts.rs");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let compressed_dir = Path::new(&manifest_dir).join("fonts/compressed");
    let mut file = fs::File::create(&dest_path)?;

    println!("cargo:rerun-if-changed=fonts/compressed");
    println!("cargo:rerun-if-changed=fonts");

    writeln!(file, "use rand::seq::IteratorRandom;")?;
    writeln!(file, "pub fn get_embedded_fonts() -> Vec<(&'static str, &'static [u8])> {{")?;
    writeln!(file, "    vec![")?;

    let mut font_files: Vec<_> = fs::read_dir(&compressed_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|e| e == "zst").unwrap_or(false))
        .collect();
    font_files.sort_by_key(|e| e.file_name());

    for entry in font_files {
        let zst_name = entry.file_name().into_string().unwrap();
        // zst_name is "<stem>.zst"; reconstruct the original .flf name for the key
        let stem = zst_name.strip_suffix(".zst").unwrap_or(&zst_name);
        let flf_name = format!("{}.flf", stem);
        writeln!(
            file,
            "        (\"{}\", include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/fonts/compressed/{}\"))),",
            flf_name, zst_name
        )?;
    }

    writeln!(file, "    ]")?;
    writeln!(file, "}}")?;
    writeln!(file, "\npub fn random_font() -> (&'static str, &'static [u8]) {{")?;
    writeln!(file, "    let fonts = get_embedded_fonts();")?;
    writeln!(file, "    let mut rng = rand::rng();")?;
    writeln!(
        file,
        "    fonts.into_iter().choose(&mut rng).unwrap_or((\"Standard.flf\", include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/fonts/compressed/Standard.zst\"))))"
    )?;
    writeln!(file, "}}")?;
    Ok(())
}
