// This module embeds a selection of .flf font files for ASCII art rendering.
// Add or remove fonts as desired for binary size and style variety.

include!(concat!(env!("OUT_DIR"), "/generated_fonts.rs"));

pub fn decompress_font(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    use std::io::Read;
    let mut decoder =
        ruzstd::decoding::StreamingDecoder::new(data).map_err(|e| anyhow::anyhow!("{}", e))?;
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}
