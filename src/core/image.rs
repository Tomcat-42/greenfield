//! ## A greenfield image representation
//!
//! The greenfield image format is a simple **2D array of colors**, prefixed with
//! the **width** and **height** of the image, and a **quantization information**. All of the field
//! are stored in **big endian** format.
//!
//! - The first 64 bits(a full u64) are the greenfield magic value, used to identify the file as a greenfield image (`b"grnfld42"`).
//! - The next 16 bits (a full u64) bits are the width of the image.
//! - The next 16 bits (a full u64) bits are the height of the image.
//! - The next 12 bits are the quantization information tuple (see [`quantization`]). A qunatization
//!     tuple is in the form: `(bits_r, bits_g, bits_b)`, where each value is the number of bits used to
//!     store the respective color component.
//! - The remaining bits are the image color data, in row-major order. Each color has (bits_r + bits_g + bits_b) bits. So,
//!     for example, if the quantization tuple is `(5, 6, 5)`, then each color is 16 bits. To get
//!     all the colors, you must read (width * height) * (bits_r + bits_g + bits_b) bits.
//!
//! ## Format on Disk
//!
//! ```text
//! ╔════════════════════════════╤══════════════════════════════════════════════════════════╗
//! ║            Bits            │                      Description                         ║
//! ╠════════════════════════════╪══════════════════════════════════════════════════════════╣
//! ║             64             │      b"grnfld42": Magic value (0x47524E464C443432)       ║
//! ╟────────────────────────────┼──────────────────────────────────────────────────────────╢
//! ║             16             │                   u32: Image width                       ║
//! ╟────────────────────────────┼──────────────────────────────────────────────────────────╢
//! ║             16             │                   u32: Image height                      ║
//! ╟────────────────────────────┼──────────────────────────────────────────────────────────╢
//! ║             12             │      (bits_r, bits_g, bits_b): Quantization tuple        ║
//! ╟────────────────────────────┼──────────────────────────────────────────────────────────╢
//! ║      width * height *      │       [RGB]: (bits_r + bits_g + bits_r) per pixel        ║
//! ║ (bits_r + bits_g + bits_b) │                       row-major                          ║
//! ╚════════════════════════════╧══════════════════════════════════════════════════════════╝
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use std::env;
//!
//! #[test]
//! /// Should create a new image
//! fn image_new() -> GreenfieldResult<()> {
//!     // Ok
//!     let image = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(1, 1, 1)?,
//!         vec![color::Rgb::new(0, 0, 0)?],
//!     )?;
//!     assert_eq!(image.width, 1);
//!     assert_eq!(image.height, 1);
//!     assert_eq!(
//!         image.uniform_quantization,
//!         quantization::UniformQuantization::new(1, 1, 1)?
//!     );
//!     assert_eq!(image.data, vec![color::Rgb::new(0, 0, 0)?]);
//!
//!     // You can specify a width and height of 0, as long as the data is empty.
//!     let image = Image::new(
//!         0,
//!         0,
//!         quantization::UniformQuantization::new(1, 1, 1)?,
//!         vec![],
//!     )?;
//!     assert_eq!(image.width, 0);
//!     assert_eq!(image.height, 0);
//!     assert_eq!(
//!         image.uniform_quantization,
//!         quantization::UniformQuantization::new(1, 1, 1)?
//!     );
//!
//!     // Invalid quantization
//!     let quantization = quantization::UniformQuantization::new(0, 1, 1);
//!     assert!(quantization.is_err());
//!
//!     // Invalid data
//!     let image = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(1, 1, 1)?,
//!         vec![color::Rgb::new(0, 0, 0)?, color::Rgb::new(0, 0, 0)?],
//!     );
//!     assert!(image.is_err());
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Shoud serialize an image
//! fn image_serialize() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::new(0, 0, 0)?],
//!     )?;
//!     let serialized = image.serialize()?;
//!     let expected = vec![
//!         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
//!     ];
//!     assert_eq!(serialized, expected);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should deserialize an image
//! fn image_deserialize() -> GreenfieldResult<()> {
//!     // Ok
//!     let serialized = vec![
//!         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
//!     ];
//!     let image = Image::deserialize(&serialized)?;
//!     let expected = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::new(0, 0, 0)?],
//!     )?;
//!     assert_eq!(image, expected);
//!
//!     // Invalid data: lower colors than expected
//!     let serialized = vec![
//!         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128,
//!     ];
//!     let image = Image::deserialize(&serialized);
//!     assert!(image.is_err());
//!
//!     // Ok: additional data will be ignored
//!     let serialized = vec![
//!         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0, 0,
//!     ];
//!     let image = Image::deserialize(&serialized)?;
//!     assert_eq!(image, expected);
//!
//!     // Invalid data: invalid magic number
//!     let serialized = vec![
//!         103, 114, 110, 102, 108, 100, 52, 51, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
//!     ];
//!     let image = Image::deserialize(&serialized);
//!     assert!(image.is_err());
//!
//!     Ok(())
//! }
//!
//! #[test]
//! // Should serialize and deserialize an image
//! fn image_serialize_deserialize() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::new(0, 0, 0)?],
//!     )?;
//!     let serialized = image.clone().serialize()?;
//!     let deserialized = Image::deserialize(&serialized)?;
//!     assert_eq!(image, deserialized);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly IO an image
//! fn image_io() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         1,
//!         1,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::new(0, 0, 0)?],
//!     )?;
//!
//!     let base_path = env::current_dir()?.join("src").join("core").join("image");
//!
//!     image
//!         .clone()
//!         .to_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
//!     let read_image = Image::from_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
//!
//!     assert_eq!(image, read_image);
//!     std::fs::remove_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should display an image
//! fn image_display() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         2,
//!         2,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!         ],
//!     )?;
//!     println!("{}", image);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should debug an image
//! fn image_debug() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         2,
//!         2,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!             color::Rgb::random(),
//!         ],
//!     )?;
//!     println!("{:?}", image);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly iterate over the image as colors
//! fn image_colors() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         10,
//!         10,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::default(); 100],
//!     )?;
//!
//!     let colors = image.colors().collect::<Vec<&color::Rgb>>();
//!     assert_eq!(colors.len(), 100);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly into iterate over the image as pixels
//! fn image_pixels() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         10,
//!         10,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::default(); 100],
//!     )?;
//!
//!     // 🤷
//!     let iter = image.pixels()?.collect::<Vec<pixel::Pixel>>();
//!     assert_eq!(iter.len(), 100);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly into iterate over the image as bytes
//! fn image_bytes() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         10,
//!         10,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::default(); 100],
//!     )?;
//!
//!     let iter = image.bytes().collect::<Vec<u8>>();
//!     assert_eq!(iter.len(), 300);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly get the image dimensions
//! fn image_dimensions() -> GreenfieldResult<()> {
//!     let image = Image::new(
//!         10,
//!         10,
//!         quantization::UniformQuantization::new(8, 8, 8)?,
//!         vec![color::Rgb::default(); 100],
//!     )?;
//!     let (width, height) = image.dimensions();
//!
//!     assert_eq!(width, 10);
//!     assert_eq!(height, 10);
//!
//!     Ok(())
//! }
//!
//! #[test]
//! /// Should correctly get the image quantization
//! fn image_quantization() -> GreenfieldResult<()> {
//!     let quantization = quantization::UniformQuantization::new(8, 8, 8)?;
//!     let image = Image::new(
//!         10,
//!         10,
//!         quantization.clone(),
//!         vec![color::Rgb::default(); 100],
//!     )?;
//!     let image_quantization = image.quantization();
//!
//!     assert_eq!(*image_quantization, quantization);
//!
//!     Ok(())
//! }
//! ```
#[cfg(test)]
mod tests;
use std::fmt::Display;
use std::path::PathBuf;

use super::{color, quantization};
use crate::error::{GreenfieldError, GreenfieldResult};
use crate::pixel;
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;

/// ## Image structure
///
/// The image structure is the main structure of the file. It contains the width, the height, the
/// quantization and the colors.
///
/// The colors are stored in a linear array, from the top left corner to the bottom right corner.
/// Each color is stored in the quantization format.
#[derive(Debug, Eq, Clone, PartialEq, DekuRead, DekuWrite)]
#[deku(magic = b"grnfld42", endian = "big")]
pub struct Image {
    #[deku(bits = "32")]
    width: usize,
    #[deku(bits = "32")]
    height: usize,
    uniform_quantization: quantization::UniformQuantization,

    #[deku(
        count = "self.width * self.height",
        reader = "Self::data_read(deku::rest, &uniform_quantization, &width, &height)",
        writer = "Self::data_write(deku::output, &data, &uniform_quantization, &width, &height)"
    )]
    data: Vec<color::Rgb>,
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}x{}] {} [{}]",
            self.width,
            self.height,
            self.uniform_quantization,
            self.data
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Image {
    /// ## Makes a new image from the given width, height, quantization tuple, and color data.
    ///
    /// Note that you must provide the colors in the full RGB format, even if the quantization tuple
    /// is not `(8, 8, 8)`. After the image is created, it will be quantized to the given tuple.
    ///
    /// ## Errors
    /// - If the quantization tuple is invalid.
    /// - If the color data is not the same length as the width * height.
    ///
    /// ## Examples
    /// ```rust
    /// use greenfield::prelude::*;
    ///
    /// #[test]
    /// /// Should create a new image
    /// fn image_new() -> GreenfieldResult<()> {
    ///     // Ok
    ///     let image = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(1, 1, 1)?,
    ///         vec![color::Rgb::new(0, 0, 0)?],
    ///     )?;
    ///     assert_eq!(image.width, 1);
    ///     assert_eq!(image.height, 1);
    ///     assert_eq!(
    ///         image.uniform_quantization,
    ///         quantization::UniformQuantization::new(1, 1, 1)?
    ///     );
    ///     assert_eq!(image.data, vec![color::Rgb::new(0, 0, 0)?]);
    ///
    ///     // You can specify a width and height of 0, as long as the data is empty.
    ///     let image = Image::new(
    ///         0,
    ///         0,
    ///         quantization::UniformQuantization::new(1, 1, 1)?,
    ///         vec![],
    ///     )?;
    ///     assert_eq!(image.width, 0);
    ///     assert_eq!(image.height, 0);
    ///     assert_eq!(
    ///         image.uniform_quantization,
    ///         quantization::UniformQuantization::new(1, 1, 1)?
    ///     );
    ///
    ///     // Invalid quantization
    ///     let quantization = quantization::UniformQuantization::new(0, 1, 1);
    ///     assert!(quantization.is_err());
    ///
    ///     // Invalid data
    ///     let image = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(1, 1, 1)?,
    ///         vec![color::Rgb::new(0, 0, 0)?, color::Rgb::new(0, 0, 0)?],
    ///     );
    ///     assert!(image.is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        uniform_quantization: quantization::UniformQuantization,
        data: Vec<color::Rgb>,
    ) -> GreenfieldResult<Self> {
        let size = width * height;
        let data_len = data.len();

        match size == data_len {
            true => {
                // Need to quantify the data first
                let data = data
                    .into_iter()
                    .map(|c| uniform_quantization.get_quantized_color(&c))
                    .collect::<Vec<color::Rgb>>();

                Ok(Self {
                    width,
                    height,
                    uniform_quantization,
                    data,
                })
            }
            false => Err(GreenfieldError::InvalidImageDimension(data_len, size)),
        }
    }

    /// ## Transforms the image into a raw byte vector.
    ///
    /// ## Examples
    ///
    /// Should serialize a image.
    ///
    /// ```rust
    /// use greenfield::prelude::*;
    ///
    /// #[test]
    /// /// Shoud serialize an image
    /// fn image_serialize() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::new(0, 0, 0)?],
    ///     )?;
    ///     let serialized = image.serialize()?;
    ///     let expected = vec![
    ///         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ///     ];
    ///     assert_eq!(serialized, expected);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn serialize(self) -> GreenfieldResult<Vec<u8>> {
        Ok(self.try_into()?)
    }

    /// ## Reads the image from a raw byte vector.
    ///
    /// ## Errors
    ///
    /// - If the byte vector is not a valid greenfield image.
    ///
    /// ## Examples
    ///
    /// Should deserialize an image
    ///
    /// ```rust
    /// use greenfield::prelude::*;
    ///
    /// #[test]
    /// /// Should deserialize an image
    /// fn image_deserialize() -> GreenfieldResult<()> {
    ///     // Ok
    ///     let serialized = vec![
    ///         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ///     ];
    ///     let image = Image::deserialize(&serialized)?;
    ///     let expected = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::new(0, 0, 0)?],
    ///     )?;
    ///     assert_eq!(image, expected);
    ///
    ///     // Invalid data: lower colors than expected
    ///     let serialized = vec![
    ///         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128,
    ///     ];
    ///     let image = Image::deserialize(&serialized);
    ///     assert!(image.is_err());
    ///
    ///     // Ok: additional data will be ignored
    ///     let serialized = vec![
    ///         103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0, 0,
    ///     ];
    ///     let image = Image::deserialize(&serialized)?;
    ///     assert_eq!(image, expected);
    ///
    ///     // Invalid data: invalid magic number
    ///     let serialized = vec![
    ///         103, 114, 110, 102, 108, 100, 52, 51, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ///     ];
    ///     let image = Image::deserialize(&serialized);
    ///     assert!(image.is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn deserialize(bytes: &[u8]) -> GreenfieldResult<Self> {
        Ok(Self::try_from(bytes)?)
    }

    /// Reads the image data from a bit slice.
    ///
    /// Just a wrapper around self::serialize.
    ///
    /// # Errors
    /// - If the bit slice is not a valid greenfield image.
    ///
    /// # Examples
    ///
    /// Should correctly IO an image
    ///
    /// ```rust
    /// #[test]
    /// /// Should correctly IO an image
    /// fn image_io() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::new(0, 0, 0)?],
    ///     )?;
    ///
    ///     let base_path = env::current_dir()?.join("src").join("core").join("image");
    ///
    ///     image
    ///         .clone()
    ///         .to_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///     let read_image = Image::from_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///
    ///     assert_eq!(image, read_image);
    ///     std::fs::remove_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn to_file(self, path: &PathBuf) -> GreenfieldResult<()> {
        std::fs::write(path, self.serialize()?)?;
        Ok(())
    }

    /// Reads the image data from a file.
    ///
    /// Just a wrapper around self::deserialize.
    ///
    /// # Errors
    ///
    /// # Examples
    ///
    /// Should correctly IO an image
    ///
    /// ```rust
    /// #[test]
    /// /// Should correctly IO an image
    /// fn image_io() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         1,
    ///         1,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::new(0, 0, 0)?],
    ///     )?;
    ///
    ///     let base_path = env::current_dir()?.join("src").join("core").join("image");
    ///
    ///     image
    ///         .clone()
    ///         .to_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///     let read_image = Image::from_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///
    ///     assert_eq!(image, read_image);
    ///     std::fs::remove_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_file(path: &PathBuf) -> GreenfieldResult<Image> {
        let serialized = std::fs::read(path)?;
        let image = Image::try_from(serialized.as_slice())?;
        Ok(image)
    }

    /// ## Returns the width and height of the image.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// #[test]
    /// /// Should correctly get the image dimensions
    /// fn image_dimensions() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         10,
    ///         10,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::default(); 100],
    ///     )?;
    ///     let (width, height) = image.dimensions();
    ///
    ///     assert_eq!(width, 10);
    ///     assert_eq!(height, 10);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// ## Returns the quantization of the image.
    ///
    /// ## Examples
    ///
    /// ```rust
    ///
    /// #[test]
    /// /// Should correctly get the image quantization
    /// fn image_quantization() -> GreenfieldResult<()> {
    ///     let quantization = quantization::UniformQuantization::new(8, 8, 8)?;
    ///     let image = Image::new(
    ///         10,
    ///         10,
    ///         quantization.clone(),
    ///         vec![color::Rgb::default(); 100],
    ///     )?;
    ///     let image_quantization = image.quantization();
    ///
    ///     assert_eq!(*image_quantization, quantization);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn quantization(&self) -> &quantization::UniformQuantization {
        &self.uniform_quantization
    }

    /// ## Iterates over the colors of the image.
    ///
    /// ## Examples
    ///
    /// ```rust
    ///
    /// #[test]
    /// /// Should correctly iterate over the image as colors
    /// fn image_colors() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         10,
    ///         10,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::default(); 100],
    ///     )?;
    ///
    ///     let colors = image.colors().collect::<Vec<&color::Rgb>>();
    ///     assert_eq!(colors.len(), 100);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn colors(&self) -> impl Iterator<Item = &color::Rgb> {
        self.data.iter()
    }

    /// ## Iterates over the pixels of the image.
    ///
    /// ## Examples
    /// ```rust
    ///
    /// #[test]
    /// /// Should correctly into iterate over the image as pixels
    /// fn image_pixels() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         10,
    ///         10,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::default(); 100],
    ///     )?;
    ///
    ///     // 🤷
    ///     let iter = image.pixels()?.collect::<Vec<pixel::Pixel>>();
    ///     assert_eq!(iter.len(), 100);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn pixels(&self) -> impl Iterator<Item = pixel::Pixel> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, color)| pixel::Pixel::new(i / self.width, i % self.height, &color))
    }

    /// ## Iterates over the pixels of the image as bytes.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// #[test]
    /// /// Should correctly into iterate over the image as bytes
    /// fn image_bytes() -> GreenfieldResult<()> {
    ///     let image = Image::new(
    ///         10,
    ///         10,
    ///         quantization::UniformQuantization::new(8, 8, 8)?,
    ///         vec![color::Rgb::default(); 100],
    ///     )?;
    ///
    ///     let iter = image.bytes().collect::<Vec<u8>>();
    ///     assert_eq!(iter.len(), 300);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
        self.data.iter().map(|color| color.bytes()).flatten()
    }

    /// ## Custom writer for the data field.
    ///
    /// Writes the data field of the image considering the quantization.
    fn data_read<'a>(
        rest: &'a BitSlice<u8, Msb0>,
        uniform_quantization: &quantization::UniformQuantization,
        width: &usize,
        height: &usize,
    ) -> GreenfieldResult<(&'a BitSlice<u8, Msb0>, Vec<color::Rgb>)> {
        let quantization::UniformQuantization {
            bits_r,
            bits_g,
            bits_b,
        } = uniform_quantization;
        let bits = (bits_r + bits_g + bits_b) as usize; // Number of bits per color
        let count: usize = (*width as usize) * (*height as usize); // Expected number of colors
        let data_len = rest.len() / bits as usize; // Actual number of colors

        match count == data_len {
            true => {
                let colors = uniform_quantization.decompress(rest);
                let rest = BitSlice::<u8, Msb0>::empty();
                Ok((rest, colors))
            }
            false => Err(GreenfieldError::InvalidImageDimension(data_len, count)),
        }
    }

    /// ## Custom writer for the data field.
    ///
    /// Writes the data field of the image considering the quantization.
    fn data_write(
        output: &mut BitVec<u8, Msb0>,
        data: &Vec<color::Rgb>,
        uniform_quantization: &quantization::UniformQuantization,
        width: &usize,
        height: &usize,
    ) -> GreenfieldResult<()> {
        let data_len = data.len(); // Actual number of colors
        let count: usize = (*width as usize) * (*height as usize); // Expected number of colors

        match count == data_len {
            true => {
                let compressed = uniform_quantization.compress(data);
                output.extend(compressed);
                Ok(())
            }
            false => Err(GreenfieldError::InvalidImageDimension(data_len, count)),
        }
    }
}
