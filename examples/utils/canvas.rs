//! A somewhat generic canvas
//
// - a way to accept colorsa also as (f32, f32, f32)?

#![allow(unused_variables)]

#[cfg(any(test, doc))]
use crate::NcRgb;
#[cfg(not(any(test, doc)))]
use libnotcurses_sys::NcRgb;

/// A rectangular array of pixels that can be individually colored.
#[derive(Debug)]
pub struct Canvas {
    /// The height.
    pub h: u32,
    /// The width.
    pub w: u32,
    /// A bytes buffer.
    pub buf: Vec<u8>,
}

impl Canvas {
    /// New white `Canvas`.
    pub fn new(h: u32, w: u32) -> Self {
        let buf: Vec<u8> = vec![255; (h * w) as usize * 3];
        Self { h, w, buf }
    }

    /// New custom colored `Canvas`.
    pub fn new_colored(h: u32, w: u32, color: NcRgb) -> Self {
        let mut buf = Vec::<u8>::with_capacity((h * w * 3) as usize);

        let (r, g, b) = Self::decompose_rgb(color);
        #[allow(unused_parens)]
        for _byte in (0..={ h * w }) {
            buf.push(r);
            buf.push(g);
            buf.push(b);
        }
        Self { h, w, buf }
    }

    /// Returns the canvas size in cartesian coordinates.
    ///
    /// # Returns
    /// Cartesian coordinates, goes from the bottom-left to the top-right corner.
    ///
    /// (beg_y, end_y, beg_x, end_x)
    ///
    /// ```txt
    ///     ↓ beg_x (-)   0.  (+) end_x ↓
    ///                   ↓
    ///     ·---------------------------· ← end_y (+)
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     |             +             | ← 0.
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     ·---------------------------· ← beg_y (-)
    /// ```
    ///
    pub fn size_cartesian(&self) -> (f32, f32, f32, f32) {
        (
            self.s2c_y(self.h),
            self.s2c_y(0),
            self.s2c_x(0),
            self.s2c_x(self.w),
        )
    }

    /// Returns the canvas size in screen coordinates.
    ///
    /// # Returns
    /// Screen coordinates, goes from the top-left to the bottom-right corner.
    ///
    /// (beg_y, end_y, beg_x, end_x)
    ///
    /// ```txt
    ///     ↓ beg_x (0)          end_x ↓
    ///     +---------------------------· ← beg_y (0)
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     |                           |
    ///     ·---------------------------· ← end_y
    /// ```

    pub const fn size_screen(&self) -> (u32, u32, u32, u32) {
        (0, self.h, 0, self.w)
    }

    /// Assigns a color to a pixel, using cartesian coordinates.
    pub fn set_pixel(&mut self, y: f32, x: f32, color: NcRgb) {
        let (r, g, b) = Self::decompose_rgb(color);
        let (y, x) = self.c2s(y, x);

        let idx = (x * 3 + y * 3 * self.w) as usize;
        self.buf[idx] = r;
        self.buf[idx + 1] = g;
        self.buf[idx + 2] = b;
    }

    /// Assigns a color to a pixel, using screen coordinates.
    pub fn set_pixel2(&mut self, y: u32, x: u32, color: NcRgb) {
        let (r, g, b) = Self::decompose_rgb(color);
        let idx = (x * 3 + y * 3 * self.w) as usize;
        self.buf[idx] = r;
        self.buf[idx + 1] = g;
        self.buf[idx + 2] = b;
    }

    /// Performs conversion from screen coordinates to cartesian coordiantes.
    ///
    /// In screen coordinates the origin (0,0) is at the top left:
    /// - `y` increases towards the *bottom*.
    /// - `x` increases towards the right.
    #[inline]
    pub fn s2c(&self, y: u32, x: u32) -> (f32, f32) {
        let cart_y = (y as f32 * -1.) + self.h as f32 / 2.;
        let cart_x = x as f32 - self.w as f32 / 2.;
        // println!("screen_yx=({},{}) → cart_yx=[{},{}]", y, x, cart_y, cart_x);
        (cart_y, cart_x)
    }

    pub fn s2c_y(&self, y: u32) -> f32 {
        (y as f32 * -1.) + self.h as f32 / 2.
    }
    pub fn s2c_x(&self, x: u32) -> f32 {
        x as f32 - self.w as f32 / 2.
    }

    /// Performs conversion from cartesian coordinates to screen coordiantes.
    ///
    /// In cartesian coordiantes the origin (0,0) is at the center:
    /// - `y` increases towards the top and decreases towards the bottom.
    /// - `x` increases towards the right and decreases towards the left.
    #[inline]
    pub fn c2s(&self, y: f32, x: f32) -> (u32, u32) {
        let screen_y = (self.h as f32 / 2. - y) as u32;
        let screen_x = (self.w as f32 / 2. + x) as u32;
        // println!("cart_yx=({},{}) → screen_yx=[{},{}]", y, x, screen_y, screen_x);
        (screen_y, screen_x)
    }

    // decomposes an NcRgb into its constituents.
    fn decompose_rgb(color: NcRgb) -> (u8, u8, u8) {
        let r = ((color.0 & 0xFF0000) >> 16) as u8;
        let g = ((color.0 & 0xFF00) >> 8) as u8;
        let b = (color.0 & 0xFF) as u8;
        (r, g, b)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn cartesian_to_screen() {
        let c = super::Canvas::new(30, 40);
        assert_eq!(c.c2s(0., 0.), (15, 20)); // center of the screen
        assert_eq!(c.c2s(15., 20.), (0, 40)); // top-right corner
        assert_eq!(c.c2s(-15., -20.), (30, 0)); // bottom-left corner
    }
    #[test]
    fn screen_to_cartesian() {
        let c = super::Canvas::new(30, 40);
        assert_eq!(c.s2c(15, 20), (0., 0.)); // center of the screen
        assert_eq!(c.s2c(0, 40), (15., 20.)); // top-right corner
        assert_eq!(c.s2c(30, 0), (-15., -20.)); // bottom-left corner
    }
}
