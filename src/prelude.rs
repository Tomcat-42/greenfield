//! Create prelude
//!
//! [What is a prelude?](std::prelude)
pub use crate::{
    color::Rgb,
    error::{GreenfieldError, GreenfieldResult},
    image::Image,
    io::{load_image, save_image},
    pixel::Pixel,
    quantization::UniformQuantization,
};
