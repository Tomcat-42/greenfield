//! ## IO output of the Greenfield image format
//!
//! This module contains the functions for converting images between the Greenfield image format and
//! other formats (like png, jpg, etc). You can load images from disk in common formats and convert
//! them to Greenfield images or other formats. You can also save Greenfield images to disk in
//! common formats or in the Greenfield image format itself.
//!
//! ## Examples
//!
//! ```rust
//! use greenfield::prelude::*;
//! use std::env;
//! use std::path::PathBuf;
//!
//! /// Should convert images between formats
//! fn io_image() -> GreenfieldResult<()> {
//!     let base_path = env::current_dir()?.join("src").join("io").join("assets");
//!
//!     // Should load a image on a common format from disk, convert it to a Greenfield image and save
//!     // it as a png
//!     let path = PathBuf::clone(&base_path).join("Lenna.png");
//!     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
//!     let _ = save_image(&img, &path)?;
//!
//!     // Should load a image on a common format from disk, convert it to a Greenfield image and save
//!     // it as a Greenfield image
//!     let path = PathBuf::clone(&base_path).join("Lenna.png");
//!     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
//!     let _ = save_image(&img, &path)?;
//!
//!     // Should load a Greenfield image from disk and save it as a png
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
//!     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
//!     let _ = save_image(&img, &path)?;
//!
//!     // Should load a Greenfield image from disk and save it as a Greenfield image
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
//!     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
//!     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
//!     let _ = save_image(&img, &path)?;
//!
//!     // clean up
//!     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd.png"))?;
//!     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd"))?;
//!
//!     Ok(())
//! }
//! ````
use crate::prelude::*;
use image::GenericImageView;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

/// ## Convert a image to a Greenfield image and returns it
///
/// Uses the `image` crate to read an image from a file and convert it to a Greenfield image.
///
/// ## Arguments
/// * `path` - The path to the image file
///
/// ## Returns
/// A Greenfield image
///
/// ## Errors
/// * If the image cannot be read
/// * If the image cannot be converted to a Greenfield image
///
/// ## Examples
///
/// ```rust
/// use greenfield::prelude::*;
/// use std::env;
/// use std::path::PathBuf;
///
/// /// Should convert images between formats
/// fn io_image() -> GreenfieldResult<()> {
///     let base_path = env::current_dir()?.join("src").join("io").join("assets");
///
///     // Should load a image on a common format from disk, convert it to a Greenfield image and save
///     // it as a png
///     let path = PathBuf::clone(&base_path).join("Lenna.png");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a image on a common format from disk, convert it to a Greenfield image and save
///     // it as a Greenfield image
///     let path = PathBuf::clone(&base_path).join("Lenna.png");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a Greenfield image from disk and save it as a png
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a Greenfield image from disk and save it as a Greenfield image
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let _ = save_image(&img, &path)?;
///
///     // clean up
///     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd.png"))?;
///     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd"))?;
///
///     Ok(())
/// }
/// ````
pub fn load_image(
    path: &PathBuf,
    uniform_quantization: UniformQuantization,
) -> GreenfieldResult<Image> {
    let input_image = image::open(path);

    // HACK: Lil' hack to load a greenfield image from disk,
    // for the image crate doesn't support it
    match input_image {
        // It's a image on a common format
        Ok(image) => {
            let (width, height) = image.dimensions();
            let data = match image.as_rgb8() {
                Some(data) => Ok(data),
                None => Err(GreenfieldError::InvalidImageDimension(
                    width as usize * height as usize * 3,
                    0,
                )),
            }?;

            Image::new(
                width as usize,
                height as usize,
                uniform_quantization,
                data.chunks(3)
                    .map(|c| Rgb::new(c[0] as u8, c[1] as u8, c[2] as u8))
                    .collect::<Vec<Rgb>>(),
            )
        }
        // It's a greenfield image
        Err(image::ImageError::Unsupported(_)) => Ok(Image::from_file(path)?),
        Err(e) => Err(GreenfieldError::ImageError(e)),
    }
}

/// ## Convert a Greenfield image to a image and save it to a file
///
/// Uses the `image` crate to convert a Greenfield image to a image and save it to a file.
///
/// ## Arguments
/// * `image` - The Greenfield image
/// * `path` - The path to the image file
///
/// ## Returns
/// Nothing
///
/// ## Errors
/// * If the image cannot be converted to a image
/// * If the image cannot be saved to a file
///
/// ## Examples
///
/// ```rust
/// use greenfield::prelude::*;
/// use std::env;
/// use std::path::PathBuf;
///
/// /// Should convert images between formats
/// fn io_image() -> GreenfieldResult<()> {
///     let base_path = env::current_dir()?.join("src").join("io").join("assets");
///
///     // Should load a image on a common format from disk, convert it to a Greenfield image and save
///     // it as a png
///     let path = PathBuf::clone(&base_path).join("Lenna.png");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a image on a common format from disk, convert it to a Greenfield image and save
///     // it as a Greenfield image
///     let path = PathBuf::clone(&base_path).join("Lenna.png");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a Greenfield image from disk and save it as a png
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
///     let _ = save_image(&img, &path)?;
///
///     // Should load a Greenfield image from disk and save it as a Greenfield image
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
///     let path = PathBuf::clone(&base_path).join("Lenna.gfd");
///     let _ = save_image(&img, &path)?;
///
///     // clean up
///     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd.png"))?;
///     std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd"))?;
///
///     Ok(())
/// }
/// ````
pub fn save_image(image: &Image, path: &PathBuf) -> GreenfieldResult<()> {
    let (width, height) = image.dimensions();

    let data = image.bytes().collect::<Vec<u8>>();

    // HACK: This is a bit of a hack, for saving the image as a greenfield image
    // we need to change the extension to .gfd but the image crate doesn't
    // support this, so we have to do it manually
    let res = image::save_buffer(
        path,
        &data,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    );
    match res {
        Err(image::ImageError::Unsupported(_)) => Ok(image.clone().to_file(path)?),
        _ => Ok(()),
    }
}
