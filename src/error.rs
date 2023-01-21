//! Error handling and custom Result types for the greenfield crate.
//!
//! This module contains the error types for the greenfield crate. It also contains the custom
//! [`Result`] type for the crate, which is a [`std::result::Result`] with the error type set to
//! [`GreenfieldError`].
use deku::DekuError;
use image::ImageError;
use thiserror::Error;

/// Error type for the greenfield crate
///
/// This error type is used for all errors that can occur when working with the greenfield crate.
#[derive(Error, Debug)]
pub enum GreenfieldError {
    #[error("Invalid quantization levels: {0} {1} {2}. Levels must be between 1 and 8.")]
    InvalidQuantizationLevel(u8, u8, u8),

    #[error("Each color should be represented by {0} bits, but {1} bits has been found instead.")]
    InvalidDataSize(usize, usize),

    #[error("Invalid image dimensions: {0} Pixels found (expected {1})")]
    InvalidImageDimension(usize, usize),

    #[error("Error while io: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Deku error: {0}")]
    DekuError(#[from] DekuError),

    #[error("Image error: {0}")]
    ImageError(#[from] ImageError),
}

// Not sure why this is needed, but it is
impl From<GreenfieldError> for DekuError {
    fn from(e: GreenfieldError) -> Self {
        DekuError::Unexpected(e.to_string())
    }
}

/// A wrapper around [`std::result::Result`] with the error type set to [`GreenfieldError`].
pub type GreenfieldResult<T> = Result<T, GreenfieldError>;
