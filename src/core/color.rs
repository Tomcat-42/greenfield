//! ## Color formats, implementations and conversions.
//!
//! Provides colors formats like Rgb and some utilities methods/functions over it.
//!
//! ## Implementations
//!
//! - [RGB](https://pt.wikipedia.org/wiki/RGB): Red, Green and Blue.
//!
//! ## Examples
//!
//! ```
//! use greenfield::prelude::*;
//!
//! #[test]
//! /// Should create a new RGB color
//! fn color_rgb_new() {
//!     let color = Rgb::new(200, 150, 10);
//!     let Rgb { r, g, b } = color;
//!
//!     assert_eq!(r, 200);
//!     assert_eq!(g, 150);
//!     assert_eq!(b, 10);
//! }
//!
//! #[test]
//! /// Should create a new default RGB color
//! fn color_rgb_default() {
//!     let color = Rgb::default();
//!     let Rgb { r, g, b } = color;
//!
//!     assert_eq!(r, 0);
//!     assert_eq!(g, 0);
//!     assert_eq!(b, 0);
//! }
//!
//! #[test]
//! /// Should create a new random RGB color
//! fn color_rgb_random() {
//!     let _color = Rgb::random();
//! }
//!
//! #[test]
//! /// Should Display a RGB color
//! fn color_rgb_display() {
//!     let color = Rgb::random();
//!     println!("{}", color);
//! }
//!
//! #[test]
//! /// Should return a RGB color as a tuple of bytes
//! fn color_rgb_bytes() {
//!     let color = Rgb::new(200, 150, 10);
//!     let [r, g, b] = color.bytes();
//!
//!     assert_eq!(r, 200);
//!     assert_eq!(g, 150);
//!     assert_eq!(b, 10);
//! }
//! ```

#[cfg(test)]
mod tests;

use colored::Colorize;
use std::fmt::{Display, Formatter};

use deku::prelude::*;
use rand::Rng;

/// ## RGB color struct
///
/// Contains the red, green and blue components of a color. Can be
/// created directly for practicality, but it is recommended to use
/// the `new` method.
///
/// Note that the color derive from `DekuRead` and `DekuWrite`, so
/// it can be used directly with the `deku` for serialization and
/// deserialization, occupying 3 bytes each on disk (in big endian).
#[derive(Debug, Clone, Eq, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// ## Creates a new `Rgb` struct.
    ///
    /// The arguments are the color components.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use greenfield::prelude::*;
    ///
    /// #[test]
    /// /// Should create a new RGB color
    /// fn color_rgb_new() {
    ///     let color = Rgb::new(200, 150, 10);
    ///     let Rgb { r, g, b } = color;
    ///
    ///     assert_eq!(r, 200);
    ///     assert_eq!(g, 150);
    ///     assert_eq!(b, 10);
    /// }
    /// ````
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// ## Creates a new random `Rgb` struct.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// #[test]
    /// /// Should create a new random RGB color
    /// fn color_rgb_random() {
    ///     let _color = Rgb::random();
    /// }
    ///  ```
    pub fn random() -> Self {
        let (r, g, b) = rand::thread_rng().gen::<(u8, u8, u8)>();
        Self { r, g, b }
    }

    /// ## Returns the color components as bytes.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use greenfield::prelude::*;
    /// #[test]
    /// /// Should return a RGB color as a tuple of bytes
    /// fn color_rgb_bytes() {
    ///     let color = Rgb::new(200, 150, 10);
    ///     let [r, g, b] = color.bytes();
    ///
    ///     assert_eq!(r, 200);
    ///     assert_eq!(g, 150);
    ///     assert_eq!(b, 10);
    /// }
    /// ````
    pub fn bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Display for Rgb {
    /// ## Formats the color as a string.
    ///
    /// Return a hex string with the color components.
    ///
    /// ## Examples
    /// ```rust
    ///
    /// #[test]
    /// /// Should Display a RGB color
    /// fn color_rgb_display() {
    ///     let color = Rgb::random();
    ///     println!("{}", color);
    /// }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { r, g, b } = *self;
        let hex_color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        write!(f, "{}", hex_color.truecolor(r, g, b))
    }
}

impl Default for Rgb {
    /// ## Creates a new default `Rgb` struct.
    ///
    /// The default color components are 0, 0, 0 (black).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// #[test]
    /// /// Should create a new default RGB color
    /// fn color_rgb_default() {
    ///     let color = Rgb::default();
    ///     let Rgb { r, g, b } = color;
    ///
    ///     assert_eq!(r, 0);
    ///     assert_eq!(g, 0);
    ///     assert_eq!(b, 0);
    /// }
    /// ```
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}
