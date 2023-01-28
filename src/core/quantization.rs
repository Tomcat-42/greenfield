//! ## Rgb quantization data structures and algorithms for greenfield images.
//!
//! Rgb quantization is the process of reducing the color space size of an image with the
//! objective of reducing the size of the image in disk. This is done by grouping similar colors in
//! the space and referring to the group instead of a particular color.
//!
//! Greenfield images uses a quantization technique know as **Uniform Quantization**. In uniform
//! quantization, we divide each component of the color space in equal intervals, indexing each
//! interval with a number, and then we assign a color to each interval (usually the mean of this interval), in the end,
//! we just store on disk this index, instead of the color. With that, we can reduce the number of
//! bits needed to store each component of a color. Once an image has been loaded from disk, we can
//! find the respective color for each index (the mean of an interval represented by that index)
//! and reconstruct the image.
//!
//! For example, if we have a 24-bit RGB image, we can reduce the number of bits needed to store
//! each component to 5 bits, reducing the number of bits needed to store each pixel from 24 to 15.
//! Reducing each component to 5 bits, we now have 2^5 = 32 possible values for each component.
//! Each distinct value is the mean of the interval in the RGB color space.
//!
//! ## Examples
//!
//! ```rust
//! /// All quantizations fields correctly set
//! #[test]
//! fn quantization_new_ok() -> GreenfieldResult<()> {
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     assert_eq!(
//!         quantization,
//!         UniformQuantization {
//!             bits_r: 1,
//!             bits_g: 1,
//!             bits_b: 1
//!         }
//!     );
//!
//!     Ok(())
//! }
//!
//! /// Fields lower than 1 or higher than 8 are not allowed
//! #[test]
//! fn quantization_new_err() -> GreenfieldResult<()> {
//!     let quantization = UniformQuantization::new(0, 1, 1);
//!     assert!(quantization.is_err());
//!
//!     let quantization = UniformQuantization::new(1, 0, 1);
//!     assert!(quantization.is_err());
//!
//!     let quantization = UniformQuantization::new(1, 1, 0);
//!     assert!(quantization.is_err());
//!
//!     let quantization = UniformQuantization::new(9, 1, 1);
//!     assert!(quantization.is_err());
//!
//!     let quantization = UniformQuantization::new(1, 9, 1);
//!     assert!(quantization.is_err());
//!
//!     let quantization = UniformQuantization::new(1, 1, 9);
//!     assert!(quantization.is_err());
//!
//!     Ok(())
//! }
//!
//! /// The default quantization is 8 bits per channel
//! #[test]
//! fn quantization_default() -> GreenfieldResult<()> {
//!     let quantization = UniformQuantization::default();
//!     assert_eq!(
//!         quantization,
//!         UniformQuantization {
//!             bits_r: 8,
//!             bits_g: 8,
//!             bits_b: 8
//!         }
//!     );
//!
//!     Ok(())
//! }
//!
//! /// A color should be correctly quantized
//! #[test]
//! fn quantization_get_quantized_color() -> GreenfieldResult<()> {
//!     let color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(0, 0, 0);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     let color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(1, 1, 1);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     let color = color::Rgb::new(100, 100, 100);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(0, 0, 0);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     let color = color::Rgb::new(200, 200, 200);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(1, 1, 1);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     let color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(31, 63, 31);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     let color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     let quantized_color = quantization.get_quantized_color(&color);
//!     let expected_color = color::Rgb::new(255, 255, 255);
//!     assert_eq!(quantized_color, expected_color);
//!
//!     Ok(())
//! }
//!
//! /// A color should be correctly quantized in place
//! #[test]
//! fn quantization_quantify_color() -> GreenfieldResult<()> {
//!     let mut color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(0, 0, 0);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(1, 1, 1);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(100, 100, 100);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(0, 0, 0);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(200, 200, 200);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(1, 1, 1);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(31, 63, 31);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     quantization.quantify_color(&mut color);
//!     let expected_color = color::Rgb::new(255, 255, 255);
//!     assert_eq!(color, expected_color);
//!
//!     Ok(())
//! }
//!
//! /// A color should be correctly dequantized
//! #[test]
//! fn quantization_get_dequantized_color() -> GreenfieldResult<()> {
//!     let color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let dequantized_color = quantization.get_dequantized_color(&color);
//!     let expected_color = color::Rgb::new(64, 64, 64);
//!     assert_eq!(dequantized_color, expected_color);
//!
//!     let color = color::Rgb::new(1, 1, 1);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     let dequantized_color = quantization.get_dequantized_color(&color);
//!     let expected_color = color::Rgb::new(192, 192, 192);
//!     assert_eq!(dequantized_color, expected_color);
//!
//!     let color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let dequantized_color = quantization.get_dequantized_color(&color);
//!     let expected_color = color::Rgb::new(4, 2, 4);
//!     assert_eq!(dequantized_color, expected_color);
//!
//!     let color = color::Rgb::new(31, 63, 31);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let dequantized_color = quantization.get_dequantized_color(&color);
//!     let expected_color = color::Rgb::new(252, 254, 252);
//!     assert_eq!(dequantized_color, expected_color);
//!
//!     let color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     let dequantized_color = quantization.get_dequantized_color(&color);
//!     let expected_color = color::Rgb::new(255, 255, 255);
//!     assert_eq!(dequantized_color, expected_color);
//!
//!     Ok(())
//! }
//!
//! /// A color should be correctly dequantized in place
//! #[test]
//! fn quantization_dequantify_color() -> GreenfieldResult<()> {
//!     let mut color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.dequantify_color(&mut color);
//!     let expected_color = color::Rgb::new(64, 64, 64);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(1, 1, 1);
//!     let quantization = UniformQuantization::new(1, 1, 1)?;
//!     quantization.dequantify_color(&mut color);
//!     let expected_color = color::Rgb::new(192, 192, 192);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(0, 0, 0);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     quantization.dequantify_color(&mut color);
//!     let expected_color = color::Rgb::new(4, 2, 4);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(31, 63, 31);
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     quantization.dequantify_color(&mut color);
//!     let expected_color = color::Rgb::new(252, 254, 252);
//!     assert_eq!(color, expected_color);
//!
//!     let mut color = color::Rgb::new(255, 255, 255);
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     quantization.dequantify_color(&mut color);
//!     let expected_color = color::Rgb::new(255, 255, 255);
//!     assert_eq!(color, expected_color);
//!
//!     Ok(())
//! }
//!
//! /// A compressed BitSlice should be correctly decompressed to a Vec of Colors
//! #[test]
//! fn quantization_decompress() -> GreenfieldResult<()> {
//!     let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     let decompressed = quantization.decompress(&compressed);
//!     let expected = vec![color::Rgb::new(1, 1, 1)];
//!     assert_eq!(decompressed, expected);
//!
//!     let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let decompressed = quantization.decompress(&compressed);
//!     let expected = vec![color::Rgb::new(12, 6, 12)];
//!     assert_eq!(decompressed, expected);
//!
//!     let compressed =
//!         bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let decompressed = quantization.decompress(&compressed);
//!     let expected = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
//!     assert_eq!(decompressed, expected);
//!
//!     let compressed = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
//!     let quantization = UniformQuantization::new(2, 2, 2)?;
//!     let decompressed = quantization.decompress(&compressed);
//!     let expected = vec![color::Rgb::new(224, 224, 224)];
//!     assert_eq!(decompressed, expected);
//!
//!     Ok(())
//! }
//!
//! /// A Vec of Colors should be correctly compressed to a BitSlice
//! #[test]
//! fn quantization_compress() -> GreenfieldResult<()> {
//!     let colors = vec![color::Rgb::new(1, 1, 1)];
//!     let quantization = UniformQuantization::new(8, 8, 8)?;
//!     let compressed = quantization.compress(&colors);
//!     let expected = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
//!     assert_eq!(compressed, expected);
//!
//!     let colors = vec![color::Rgb::new(12, 6, 12)];
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let compressed = quantization.compress(&colors);
//!     let expected = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
//!     assert_eq!(compressed, expected);
//!
//!     let colors = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
//!     let quantization = UniformQuantization::new(5, 6, 5)?;
//!     let compressed = quantization.compress(&colors);
//!     let expected =
//!         bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
//!     assert_eq!(compressed, expected);
//!
//!     let colors = vec![color::Rgb::new(224, 224, 224)];
//!     let quantization = UniformQuantization::new(2, 2, 2)?;
//!     let compressed = quantization.compress(&colors);
//!     let expected = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
//!     assert_eq!(compressed, expected);
//!
//!     Ok(())
//! }
//! ```
#[cfg(test)]
mod tests;

use std::fmt::Display;

use crate::error::{GreenfieldError, GreenfieldResult};

use super::color;
use deku::prelude::*;

use bitvec::prelude::*;
use deku::bitvec::{BitSlice, BitVec, Msb0};

/// ## A quantization information structure.
///
/// Each entry in this structure represents the number of bits necessary to represent a color
/// component in the image. So, N bits should give 2^N different colors.
///
/// For example, if we have a UniformQuantization{bits_r: 5, bits_g: 6, bits_b: 5}, we can represent
/// 2^5 = 32 different colors for the red component, 2^6 = 64 different colors for the green
/// component, and 2^5 = 32 different colors for the blue component. So, we can represent 32 * 64 * 32
/// = 65536 different colors in total. In this case, instead of 3 bytes to encode a RGB color, we
/// just use 2 bytes.
///
/// This structure is Deku serializable, with each entry occupying only 4 bits in disk (12 bits in
/// total)
#[derive(Debug, Eq, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: deku::ctx::Endian")]
pub struct UniformQuantization {
    #[deku(bits = "4")]
    pub bits_r: u8,
    #[deku(bits = "4")]
    pub bits_g: u8,
    #[deku(bits = "4")]
    pub bits_b: u8,
}

impl Display for UniformQuantization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.bits_r, self.bits_g, self.bits_b)
    }
}

impl Default for UniformQuantization {
    /// ## Creates a default UniformQuantization structure.
    ///
    /// The default UniformQuantization structure has 8 bits for each component. Meaning a true color
    /// (24 bit) representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use greenfield::quantization::UniformQuantization;
    ///
    /// /// The default quantization is 8 bits per channel
    /// #[test]
    /// fn quantization_default() -> GreenfieldResult<()> {
    ///     let quantization = UniformQuantization::default();
    ///     assert_eq!(
    ///         quantization,
    ///         UniformQuantization {
    ///             bits_r: 8,
    ///             bits_g: 8,
    ///             bits_b: 8
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    fn default() -> Self {
        UniformQuantization {
            bits_r: 8,
            bits_g: 8,
            bits_b: 8,
        }
    }
}

impl UniformQuantization {
    /// ## Creates a new UniformQuantization structure.
    ///
    /// The number of bits for each component must be between 1 and 8.
    ///
    /// ## Examples
    /// ```rust
    /// /// All quantizations fields correctly set
    /// #[test]
    /// fn quantization_new_ok() -> GreenfieldResult<()> {
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     assert_eq!(
    ///         quantization,
    ///         UniformQuantization {
    ///             bits_r: 1,
    ///             bits_g: 1,
    ///             bits_b: 1
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    ///
    /// /// Fields lower than 1 or higher than 8 are not allowed
    /// #[test]
    /// fn quantization_new_err() -> GreenfieldResult<()> {
    ///     let quantization = UniformQuantization::new(0, 1, 1);
    ///     assert!(quantization.is_err());
    ///
    ///     let quantization = UniformQuantization::new(1, 0, 1);
    ///     assert!(quantization.is_err());
    ///
    ///     let quantization = UniformQuantization::new(1, 1, 0);
    ///     assert!(quantization.is_err());
    ///
    ///     let quantization = UniformQuantization::new(9, 1, 1);
    ///     assert!(quantization.is_err());
    ///
    ///     let quantization = UniformQuantization::new(1, 9, 1);
    ///     assert!(quantization.is_err());
    ///
    ///     let quantization = UniformQuantization::new(1, 1, 9);
    ///     assert!(quantization.is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(bits_r: u8, bits_g: u8, bits_b: u8) -> GreenfieldResult<Self> {
        // all quantization levels must be between 1 and 8
        match (bits_r, bits_g, bits_b) {
            (1..=8, 1..=8, 1..=8) => Ok(Self {
                bits_r,
                bits_g,
                bits_b,
            }),
            _ => Err(GreenfieldError::InvalidQuantizationLevel(
                bits_r, bits_g, bits_b,
            )),
        }
    }

    /// ## Return a new color, quantized to the given number of bits. Immutable version of
    /// [`quantify_color`].
    ///
    /// The quantization is done by dividing the color space in equal intervals, and then assigning a
    /// index to each interval, this is accomplished using bitwise magic 🧙 ((x >> (8 - bits_x)) puts a
    /// component in a specific interval).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// /// A color should be correctly quantized
    /// #[test]
    /// fn quantization_get_quantized_color() -> GreenfieldResult<()> {
    ///     let color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(0, 0, 0);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(1, 1, 1);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(100, 100, 100);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(0, 0, 0);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(200, 200, 200);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(1, 1, 1);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(31, 63, 31);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     let quantized_color = quantization.get_quantized_color(&color);
    ///     let expected_color = color::Rgb::new(255, 255, 255);
    ///     assert_eq!(quantized_color, expected_color);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_quantized_color(&self, color: &color::Rgb) -> color::Rgb {
        let color::Rgb { r, g, b } = *color;

        match &self {
            Self {
                bits_r: 8,
                bits_g: 8,
                bits_b: 8,
            } => color::Rgb::new(r, g, b),

            Self {
                bits_r,
                bits_g,
                bits_b,
            } => color::Rgb::new(r >> 8 - bits_r, g >> 8 - bits_g, b >> 8 - bits_b),
        }
    }

    /// ## Quantify a color in place, given a number of bits. Mutable version of [`get_quantized_color`].
    ///
    /// The quantization is done by dividing the color space in equal intervals, and then assigning a
    /// index to each interval, this is accomplished using bitwise magic 🧙 ((x >> (8 - bits_x)) puts a
    /// component in a specific interval).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// /// A color should be correctly quantized in place
    /// #[test]
    /// fn quantization_quantify_color() -> GreenfieldResult<()> {
    ///     let mut color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(0, 0, 0);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(1, 1, 1);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(100, 100, 100);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(0, 0, 0);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(200, 200, 200);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(1, 1, 1);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(31, 63, 31);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     quantization.quantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(255, 255, 255);
    ///     assert_eq!(color, expected_color);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn quantify_color(&self, color: &mut color::Rgb) {
        match &self {
            Self {
                bits_r: 8,
                bits_g: 8,
                bits_b: 8,
            } => (),

            Self {
                bits_r,
                bits_g,
                bits_b,
            } => {
                color.r >>= 8 - bits_r;
                color.g >>= 8 - bits_g;
                color.b >>= 8 - bits_b;
            }
        }
    }

    /// ## Return a new color, dequantized to the given number of bits. Immutable version of `quantization::dequantify_color`.
    ///
    /// The dequantification is done by shifting the color components to the left to "align" a
    /// component to his index interval and then summing the "middle of the interval" to the total
    /// value. This is done using the bitwise wizardry 🧙 ((x << (8 - bits_x)) to align a component to his
    /// interval and then summing the middle of the interval (1 << (7 - bits_x) to the total value.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// /// A color should be correctly dequantized
    /// #[test]
    /// fn quantization_get_dequantized_color() -> GreenfieldResult<()> {
    ///     let color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let dequantized_color = quantization.get_dequantized_color(&color);
    ///     let expected_color = color::Rgb::new(64, 64, 64);
    ///     assert_eq!(dequantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(1, 1, 1);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     let dequantized_color = quantization.get_dequantized_color(&color);
    ///     let expected_color = color::Rgb::new(192, 192, 192);
    ///     assert_eq!(dequantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let dequantized_color = quantization.get_dequantized_color(&color);
    ///     let expected_color = color::Rgb::new(4, 2, 4);
    ///     assert_eq!(dequantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(31, 63, 31);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let dequantized_color = quantization.get_dequantized_color(&color);
    ///     let expected_color = color::Rgb::new(252, 254, 252);
    ///     assert_eq!(dequantized_color, expected_color);
    ///
    ///     let color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     let dequantized_color = quantization.get_dequantized_color(&color);
    ///     let expected_color = color::Rgb::new(255, 255, 255);
    ///     assert_eq!(dequantized_color, expected_color);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_dequantized_color(&self, color: &color::Rgb) -> color::Rgb {
        let color::Rgb { r, g, b } = *color;

        match &self {
            Self {
                bits_r: 8,
                bits_g: 8,
                bits_b: 8,
            } => color::Rgb::new(r, g, b),
            Self {
                bits_r,
                bits_g,
                bits_b,
            } => color::Rgb::new(
                (r << (8 - bits_r)) + (1 << (7 - bits_r)),
                (g << (8 - bits_g)) + (1 << (7 - bits_g)),
                (b << (8 - bits_b)) + (1 << (7 - bits_b)),
            ),
        }
    }
    /// ## Dequantify a color in place, to the given number of bits. Mutable version of `quantization::get_dequantized_color`.
    ///
    /// The dequantification is done by shifting the color components to the left to "align" a
    /// component to his index interval and then summing the "middle of the interval" to the total
    /// value. This is done using the bitwise wizardry 🧙 ((x << (8 - bits_x)) to align a component to his
    /// interval and then summing the middle of the interval (1 << (7 - bits_x) to the total value.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// /// A color should be correctly dequantized in place
    /// #[test]
    /// fn quantization_dequantify_color() -> GreenfieldResult<()> {
    ///     let mut color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.dequantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(64, 64, 64);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(1, 1, 1);
    ///     let quantization = UniformQuantization::new(1, 1, 1)?;
    ///     quantization.dequantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(192, 192, 192);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(0, 0, 0);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     quantization.dequantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(4, 2, 4);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(31, 63, 31);
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     quantization.dequantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(252, 254, 252);
    ///     assert_eq!(color, expected_color);
    ///
    ///     let mut color = color::Rgb::new(255, 255, 255);
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     quantization.dequantify_color(&mut color);
    ///     let expected_color = color::Rgb::new(255, 255, 255);
    ///     assert_eq!(color, expected_color);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn dequantify_color(&self, color: &mut color::Rgb) {
        match &self {
            Self {
                bits_r: 8,
                bits_g: 8,
                bits_b: 8,
            } => (),
            Self {
                bits_r,
                bits_g,
                bits_b,
            } => {
                color.r = (color.r << (8 - bits_r)) + (1 << (7 - bits_r));
                color.g = (color.g << (8 - bits_g)) + (1 << (7 - bits_g));
                color.b = (color.b << (8 - bits_b)) + (1 << (7 - bits_b));
            }
        }
    }

    /// ## Decompress a BitSlice containing color data into a Vec of colors.
    ///
    /// Given a quantization struct, we know that the r component are in the first bits_r bits,
    /// the g component are in the next bits_g bits and the b component are in the last bits_b bits.
    /// So, we iterate over the BitSlice bit chunks(each with size bits_r + bits_g + bits_b) and
    /// (using bitwise wizardry 🧙 again) we extract the r, g and b components from their respective positions
    /// and then we dequantize them to the original color values.
    ///
    /// ## Examples
    /// ```rust
    /// /// A compressed BitSlice should be correctly decompressed to a Vec of Colors
    /// #[test]
    /// fn quantization_decompress() -> GreenfieldResult<()> {
    ///     let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     let decompressed = quantization.decompress(&compressed);
    ///     let expected = vec![color::Rgb::new(1, 1, 1)];
    ///     assert_eq!(decompressed, expected);
    ///
    ///     let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let decompressed = quantization.decompress(&compressed);
    ///     let expected = vec![color::Rgb::new(12, 6, 12)];
    ///     assert_eq!(decompressed, expected);
    ///
    ///     let compressed =
    ///         bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let decompressed = quantization.decompress(&compressed);
    ///     let expected = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
    ///     assert_eq!(decompressed, expected);
    ///
    ///     let compressed = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
    ///     let quantization = UniformQuantization::new(2, 2, 2)?;
    ///     let decompressed = quantization.decompress(&compressed);
    ///     let expected = vec![color::Rgb::new(224, 224, 224)];
    ///     assert_eq!(decompressed, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn decompress<'a>(&'a self, data: &'a BitSlice<u8, Msb0>) -> Vec<color::Rgb> {
        let Self {
            bits_r,
            bits_g,
            bits_b,
        } = &self;
        let data_size = bits_r + bits_g + bits_b;

        data.chunks_exact(data_size as usize)
            .map(|chunk| {
                let r = chunk[0..*bits_r as usize].load_be::<u8>();
                let g = chunk[*bits_r as usize..(*bits_r + *bits_g) as usize].load_be::<u8>();
                let b = chunk[(*bits_r + *bits_g) as usize..].load_be::<u8>();

                self.get_dequantized_color(&color::Rgb::new(r, g, b))
            })
            .collect::<Vec<_>>()
    }

    /// ## Compress a Vec of colors into a BitVec containing the compressed data.
    ///
    /// Given a quantization struct, we know that the r component are in the first bits_r bits,
    /// the g component are in the next bits_g bits and the b component are in the last bits_b bits.
    /// So, we iterate over the Vec of colors and (using bitwise wizardry 🧙 again) we insert the r, g and b
    /// components into their respective positions (using only the respective needed bits ).
    ///
    /// ## Examples
    /// ```rust
    /// /// A Vec of Colors should be correctly compressed to a BitSlice
    /// #[test]
    /// fn quantization_compress() -> GreenfieldResult<()> {
    ///     let colors = vec![color::Rgb::new(1, 1, 1)];
    ///     let quantization = UniformQuantization::new(8, 8, 8)?;
    ///     let compressed = quantization.compress(&colors);
    ///     let expected = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
    ///     assert_eq!(compressed, expected);
    ///
    ///     let colors = vec![color::Rgb::new(12, 6, 12)];
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let compressed = quantization.compress(&colors);
    ///     let expected = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    ///     assert_eq!(compressed, expected);
    ///
    ///     let colors = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
    ///     let quantization = UniformQuantization::new(5, 6, 5)?;
    ///     let compressed = quantization.compress(&colors);
    ///     let expected =
    ///         bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    ///     assert_eq!(compressed, expected);
    ///
    ///     let colors = vec![color::Rgb::new(224, 224, 224)];
    ///     let quantization = UniformQuantization::new(2, 2, 2)?;
    ///     let compressed = quantization.compress(&colors);
    ///     let expected = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
    ///     assert_eq!(compressed, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn compress(&self, colors: &Vec<color::Rgb>) -> BitVec<u8, Msb0> {
        let Self {
            bits_r,
            bits_g,
            bits_b,
        } = &self;
        let data_size = (bits_r + bits_g + bits_b) as usize;
        let mut compressed_data = BitVec::<u8, Msb0>::repeat(false, colors.len() * data_size);

        for (i, color) in colors.iter().enumerate() {
            let color::Rgb { r, g, b } = self.get_quantized_color(color);
            let index = i * data_size;

            compressed_data[index..(index + *bits_r as usize)].store_be(r);
            compressed_data
                [(index + *bits_r as usize)..(index + *bits_r as usize + *bits_g as usize)]
                .store_be(g);
            compressed_data[(index + *bits_r as usize + *bits_g as usize)..(index + data_size)]
                .store_be(b);
        }

        compressed_data
    }
}
