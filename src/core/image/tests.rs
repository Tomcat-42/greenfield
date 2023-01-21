use super::*;
use std::env;

#[test]
/// Should create a new image
fn image_new() -> GreenfieldResult<()> {
    // Ok
    let image = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(1, 1, 1)?,
        vec![color::Rgb::new(0, 0, 0)],
    )?;
    assert_eq!(image.width, 1);
    assert_eq!(image.height, 1);
    assert_eq!(
        image.uniform_quantization,
        quantization::UniformQuantization::new(1, 1, 1)?
    );
    assert_eq!(image.data, vec![color::Rgb::new(0, 0, 0)]);

    // You can specify a width and height of 0, as long as the data is empty.
    let image = Image::new(
        0,
        0,
        quantization::UniformQuantization::new(1, 1, 1)?,
        vec![],
    )?;
    assert_eq!(image.width, 0);
    assert_eq!(image.height, 0);
    assert_eq!(
        image.uniform_quantization,
        quantization::UniformQuantization::new(1, 1, 1)?
    );

    // Invalid quantization
    let quantization = quantization::UniformQuantization::new(0, 1, 1);
    assert!(quantization.is_err());

    // Invalid data
    let image = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(1, 1, 1)?,
        vec![color::Rgb::new(0, 0, 0), color::Rgb::new(0, 0, 0)],
    );
    assert!(image.is_err());

    Ok(())
}

#[test]
/// Shoud serialize an image
fn image_serialize() -> GreenfieldResult<()> {
    let image = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::new(0, 0, 0)],
    )?;
    let serialized = image.serialize()?;
    let expected = vec![
        103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ];
    assert_eq!(serialized, expected);

    Ok(())
}

#[test]
/// Should deserialize an image
fn image_deserialize() -> GreenfieldResult<()> {
    // Ok
    let serialized = vec![
        103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ];
    let image = Image::deserialize(&serialized)?;
    let expected = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::new(0, 0, 0)],
    )?;
    assert_eq!(image, expected);

    // Invalid data: lower colors than expected
    let serialized = vec![
        103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128,
    ];
    let image = Image::deserialize(&serialized);
    assert!(image.is_err());

    // Ok: additional data will be ignored
    let serialized = vec![
        103, 114, 110, 102, 108, 100, 52, 50, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0, 0,
    ];
    let image = Image::deserialize(&serialized)?;
    assert_eq!(image, expected);

    // Invalid data: invalid magic number
    let serialized = vec![
        103, 114, 110, 102, 108, 100, 52, 51, 0, 0, 0, 1, 0, 0, 0, 1, 136, 128, 0, 0, 0,
    ];
    let image = Image::deserialize(&serialized);
    assert!(image.is_err());

    Ok(())
}

#[test]
// Should serialize and deserialize an image
fn image_serialize_deserialize() -> GreenfieldResult<()> {
    let image = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::new(0, 0, 0)],
    )?;
    let serialized = image.clone().serialize()?;
    let deserialized = Image::deserialize(&serialized)?;
    assert_eq!(image, deserialized);

    Ok(())
}

#[test]
/// Should correctly IO an image
fn image_io() -> GreenfieldResult<()> {
    let image = Image::new(
        1,
        1,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::new(0, 0, 0)],
    )?;

    let base_path = env::current_dir()?.join("src").join("core").join("image");

    image
        .clone()
        .to_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;
    let read_image = Image::from_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;

    assert_eq!(image, read_image);
    std::fs::remove_file(&PathBuf::clone(&base_path).join("image_io.gfd"))?;

    Ok(())
}

#[test]
/// Should display an image
fn image_display() -> GreenfieldResult<()> {
    let image = Image::new(
        2,
        2,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![
            color::Rgb::random(),
            color::Rgb::random(),
            color::Rgb::random(),
            color::Rgb::random(),
        ],
    )?;
    println!("{}", image);

    Ok(())
}

#[test]
/// Should debug an image
fn image_debug() -> GreenfieldResult<()> {
    let image = Image::new(
        2,
        2,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![
            color::Rgb::random(),
            color::Rgb::random(),
            color::Rgb::random(),
            color::Rgb::random(),
        ],
    )?;
    println!("{:?}", image);

    Ok(())
}

#[test]
/// Should correctly iterate over the image as colors
fn image_colors() -> GreenfieldResult<()> {
    let image = Image::new(
        10,
        10,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::default(); 100],
    )?;

    let colors = image.colors().collect::<Vec<&color::Rgb>>();
    assert_eq!(colors.len(), 100);

    Ok(())
}

#[test]
/// Should correctly into iterate over the image as pixels
fn image_pixels() -> GreenfieldResult<()> {
    let image = Image::new(
        10,
        10,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::default(); 100],
    )?;

    // 🤷
    let iter = image.pixels().collect::<Vec<pixel::Pixel>>();
    assert_eq!(iter.len(), 100);

    Ok(())
}

#[test]
/// Should correctly into iterate over the image as bytes
fn image_bytes() -> GreenfieldResult<()> {
    let image = Image::new(
        10,
        10,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::default(); 100],
    )?;

    let iter = image.bytes().collect::<Vec<u8>>();
    assert_eq!(iter.len(), 300);

    Ok(())
}

#[test]
/// Should correctly get the image dimensions
fn image_dimensions() -> GreenfieldResult<()> {
    let image = Image::new(
        10,
        10,
        quantization::UniformQuantization::new(8, 8, 8)?,
        vec![color::Rgb::default(); 100],
    )?;
    let (width, height) = image.dimensions();

    assert_eq!(width, 10);
    assert_eq!(height, 10);

    Ok(())
}

#[test]
/// Should correctly get the image quantization
fn image_quantization() -> GreenfieldResult<()> {
    let quantization = quantization::UniformQuantization::new(8, 8, 8)?;
    let image = Image::new(
        10,
        10,
        quantization.clone(),
        vec![color::Rgb::default(); 100],
    )?;
    let image_quantization = image.quantization();

    assert_eq!(*image_quantization, quantization);

    Ok(())
}
