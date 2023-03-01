// Graphics module for the particle simulation.
// Based on Pixels example project "Invaders"

use std::rc::Rc;

use line_drawing::Bresenham;

use super::{geometry::Point, loader::Assets};

pub type CachedSprite = (usize, usize, Rc<[u8]>);

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Frame {
    Particle,
}

/// Sprites can be drawn and procedurally generated.
///
/// A `Sprite` owns its pixel data, and cannot be animated. Use a `SpriteRef` if you need
/// animations.
#[derive(Debug)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

/// Drawables can be blitted to the pixel buffer and animated.
pub(crate) trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl Sprite {
    pub(crate) fn new(assets: &Assets, frame: Frame) -> Sprite {
        let (width, height, pixels) = assets.sprites().get(&frame).unwrap();

        Sprite {
            width: *width,
            height: *height,
            pixels: pixels.to_vec(),
        }
    }
}

impl Drawable for Sprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

/// Blit a drawable to the pixel buffer.
pub(crate) fn blit<S>(screen: &mut [u8], dest: &Point, sprite: &S, colour: [u8; 4])
where
    S: Drawable,
{
    if (dest.x + sprite.width() > crate::WIDTH as usize)
        || (dest.y + sprite.height() > crate::HEIGHT as usize)
    {
        return;
    }

    let _pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.x * 4 + dest.y * crate::WIDTH as usize * 4 + y * crate::WIDTH as usize * 4;

        // Bit of a hacky way to get force the sprite to be drawn with the specified colour - doesn't support transparency only solid colours.
        let new_pixels = [
            colour[0], colour[1], colour[2], colour[3], colour[0], colour[1], colour[2], colour[3],
            colour[0], colour[1], colour[2], colour[3], colour[0], colour[1], colour[2], colour[3],
        ];

        // Merge pixels from sprite into screen
        let zipped = screen[i..i + width].iter_mut().zip(&new_pixels[0..width]);

        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
        }

        s += width;
    }
}

/// Draw a line to the pixel buffer using Bresenham's algorithm.
pub(crate) fn line(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
    let p1 = (p1.x as i64, p1.y as i64);
    let p2 = (p2.x as i64, p2.y as i64);

    for (x, y) in Bresenham::new(p1, p2) {
        let x = std::cmp::min(x as usize, crate::WIDTH as usize - 1);
        let y = std::cmp::min(y as usize, crate::HEIGHT as usize - 1);
        let i = x * 4 + y * crate::WIDTH as usize * 4;

        screen[i..i + 4].copy_from_slice(&color);
    }
}

/// Draw a rectangle to the pixel buffer using two points in opposite corners.
pub(crate) fn rect(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
    let p2 = Point::new(p2.x - 1, p2.y - 1);
    let p3 = Point::new(p1.x, p2.y);
    let p4 = Point::new(p2.x, p1.y);

    line(screen, p1, &p3, color);
    line(screen, &p3, &p2, color);
    line(screen, &p2, &p4, color);
    line(screen, &p4, p1, color);
}
