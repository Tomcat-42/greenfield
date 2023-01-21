//! ## greenfield image format
//!
//! A create for reading and writing the greenfield image format. For a description of the format,
//! see [`image`].
//!
//! Also, it includes some utilities for manipulating images in the greenfield format, like
//! quantization, dithering and conversion to other formats.

mod core;
mod error;

pub use crate::core::color;
pub use crate::core::image;
pub use crate::core::pixel;
pub use crate::core::quantization;
pub mod io;
pub use crate::error::{GreenfieldError, GreenfieldResult};

pub mod prelude;
