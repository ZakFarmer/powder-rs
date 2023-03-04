use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;

use super::graphics::{CachedSprite, Frame};

/// A list of assets loaded into memory.
#[derive(Debug)]
pub(crate) struct Assets {
    // sounds: TODO
    sprites: HashMap<Frame, CachedSprite>,
}

impl Assets {
    pub(crate) fn sprites(&self) -> &HashMap<Frame, CachedSprite> {
        &self.sprites
    }
}

/// Load all static assets into an `Assets` structure
pub(crate) fn load_assets() -> Assets {
    use Frame::*;

    let mut sprites = HashMap::new();

    sprites.insert(
        Particle,
        load_pcx(include_bytes!("../../assets/particle.pcx")),
    );

    Assets { sprites }
}

/// Convert PCX data to raw pixels
fn load_pcx(pcx: &[u8]) -> CachedSprite {
    let mut reader = pcx::Reader::new(Cursor::new(pcx)).unwrap();
    let width = reader.width() as usize;
    let height = reader.height() as usize;
    let mut result = Vec::new();

    if reader.is_paletted() {
        // Read the raw pixel data
        let mut buffer = Vec::new();
        buffer.resize_with(width * height, Default::default);
        for y in 0..height {
            let a = y as usize * width;
            let b = a + width;
            reader.next_row_paletted(&mut buffer[a..b]).unwrap();
        }

        // Read the pallete
        let mut palette = Vec::new();
        let palette_length = reader.palette_length().unwrap() as usize;
        palette.resize_with(palette_length * 3, Default::default);
        reader.read_palette(&mut palette).unwrap();

        // Copy to result with an alpha component
        let pixels = buffer
            .into_iter()
            .flat_map(|pal| {
                let i = pal as usize * 3;
                &palette[i..i + 3]
            })
            .cloned()
            .collect::<Vec<u8>>();
        result.extend_from_slice(&pixels);
    } else {
        for _ in 0..height {
            // Read the raw pixel data
            let mut buffer = Vec::new();
            buffer.resize_with(width * 3, Default::default);
            reader.next_row_rgb(&mut buffer[..]).unwrap();

            // Copy to result with an alpha component
            let pixels = buffer
                .chunks(3)
                .flat_map(|rgb| {
                    let mut rgb = rgb.to_vec();
                    rgb.push(255);
                    rgb
                })
                .collect::<Vec<u8>>();
            result.extend_from_slice(&pixels);
        }
    }

    (width, height, Rc::from(result.as_ref()))
}
