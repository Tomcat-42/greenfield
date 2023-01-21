//! ## Pixel type and related functions/methos for pixel manipulation.
//!
//! A Pixel is a struct that contains the color and the position of a pixel in the image.
//!
//! ## Examples
//!
//! ```rust
//! #[test]
//! /// Should create a new pixel
//! fn pixel_new() {
//!     let color = color::Rgb::new(0, 0, 0);
//!     let pixel = Pixel::new(0, 0, &color);
//!
//!     let Pixel { x, y, color } = pixel;
//!     let color::Rgb { r, g, b } = color;
//!
//!     assert_eq!(x, 0);
//!     assert_eq!(y, 0);
//!     assert_eq!(*r, 0);
//!     assert_eq!(*g, 0);
//!     assert_eq!(*b, 0);
//! }
//!
//! #[test]
//! /// Should Display a pixel
//! fn pixel_display() {
//!     let color = color::Rgb::random();
//!     let pixel = Pixel::new(0, 0, &color);
//!
//!     println!("{}", pixel);
//! }
//!
//! #[test]
//! /// Should debug a pixel
//! fn pixel_debug() {
//!     let color = color::Rgb::random();
//!     let pixel = Pixel::new(0, 0, &color);
//!
//!     println!("{:?}", pixel);
//! }
//! ```

#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};

use super::color;
use colored::Colorize;

/// ## Pixel struct
///
/// Contains the color and the position of a pixel in the image.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pixel<'a> {
    pub x: usize,
    pub y: usize,
    pub color: &'a color::Rgb,
}

impl<'a> Pixel<'a> {
    /// ## Creates a new `Pixel` struct.
    ///
    /// ## Examples
    ///
    /// ```
    /// #[test]
    /// /// Should create a new pixel
    /// fn pixel_new() {
    ///     let color = color::Rgb::new(0, 0, 0);
    ///     let pixel = Pixel::new(0, 0, &color);
    ///
    ///     let Pixel { x, y, color } = pixel;
    ///     let color::Rgb { r, g, b } = color;
    ///
    ///     assert_eq!(x, 0);
    ///     assert_eq!(y, 0);
    ///     assert_eq!(*r, 0);
    ///     assert_eq!(*g, 0);
    ///     assert_eq!(*b, 0);
    /// }
    /// ```
    pub fn new(x: usize, y: usize, color: &'a color::Rgb) -> Self {
        Self { x, y, color }
    }
}

impl<'a> Display for Pixel<'a> {
    /// ## Display a pixel
    ///
    /// ## Examples
    ///
    /// ```
    /// #[test]
    /// /// Should Display a pixel
    /// fn pixel_display() {
    ///     let color = color::Rgb::random();
    ///     let pixel = Pixel::new(0, 0, &color);
    ///
    ///     println!("{}", pixel);
    /// }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, color } = self;
        let color::Rgb { r, g, b } = color;
        let pixel = format!("({},{})", x, y);
        write!(f, "{}", pixel.truecolor(*r, *g, *b))
    }
}
