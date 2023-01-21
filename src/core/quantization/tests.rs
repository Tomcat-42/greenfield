#![allow(unused_imports)]
use super::{color, BitVec, GreenfieldResult, Msb0, UniformQuantization};

/// All quantizations fields correctly set
#[test]
fn quantization_new_ok() -> GreenfieldResult<()> {
    let quantization = UniformQuantization::new(1, 1, 1)?;
    assert_eq!(
        quantization,
        UniformQuantization {
            bits_r: 1,
            bits_g: 1,
            bits_b: 1
        }
    );

    Ok(())
}

/// Fields lower than 1 or higher than 8 are not allowed
#[test]
fn quantization_new_err() -> GreenfieldResult<()> {
    let quantization = UniformQuantization::new(0, 1, 1);
    assert!(quantization.is_err());

    let quantization = UniformQuantization::new(1, 0, 1);
    assert!(quantization.is_err());

    let quantization = UniformQuantization::new(1, 1, 0);
    assert!(quantization.is_err());

    let quantization = UniformQuantization::new(9, 1, 1);
    assert!(quantization.is_err());

    let quantization = UniformQuantization::new(1, 9, 1);
    assert!(quantization.is_err());

    let quantization = UniformQuantization::new(1, 1, 9);
    assert!(quantization.is_err());

    Ok(())
}

/// The default quantization is 8 bits per channel
#[test]
fn quantization_default() -> GreenfieldResult<()> {
    let quantization = UniformQuantization::default();
    assert_eq!(
        quantization,
        UniformQuantization {
            bits_r: 8,
            bits_g: 8,
            bits_b: 8
        }
    );

    Ok(())
}

/// A color should be correctly quantized
#[test]
fn quantization_get_quantized_color() -> GreenfieldResult<()> {
    let color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(0, 0, 0);
    assert_eq!(quantized_color, expected_color);

    let color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(1, 1, 1);
    assert_eq!(quantized_color, expected_color);

    let color = color::Rgb::new(100, 100, 100);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(0, 0, 0);
    assert_eq!(quantized_color, expected_color);

    let color = color::Rgb::new(200, 200, 200);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(1, 1, 1);
    assert_eq!(quantized_color, expected_color);

    let color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(31, 63, 31);
    assert_eq!(quantized_color, expected_color);

    let color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(8, 8, 8)?;
    let quantized_color = quantization.get_quantized_color(&color);
    let expected_color = color::Rgb::new(255, 255, 255);
    assert_eq!(quantized_color, expected_color);

    Ok(())
}

/// A color should be correctly quantized in place
#[test]
fn quantization_quantify_color() -> GreenfieldResult<()> {
    let mut color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(0, 0, 0);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(1, 1, 1);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(100, 100, 100);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(0, 0, 0);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(200, 200, 200);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(1, 1, 1);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(31, 63, 31);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(8, 8, 8)?;
    quantization.quantify_color(&mut color);
    let expected_color = color::Rgb::new(255, 255, 255);
    assert_eq!(color, expected_color);

    Ok(())
}

/// A color should be correctly dequantized
#[test]
fn quantization_get_dequantized_color() -> GreenfieldResult<()> {
    let color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let dequantized_color = quantization.get_dequantized_color(&color);
    let expected_color = color::Rgb::new(64, 64, 64);
    assert_eq!(dequantized_color, expected_color);

    let color = color::Rgb::new(1, 1, 1);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    let dequantized_color = quantization.get_dequantized_color(&color);
    let expected_color = color::Rgb::new(192, 192, 192);
    assert_eq!(dequantized_color, expected_color);

    let color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let dequantized_color = quantization.get_dequantized_color(&color);
    let expected_color = color::Rgb::new(4, 2, 4);
    assert_eq!(dequantized_color, expected_color);

    let color = color::Rgb::new(31, 63, 31);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let dequantized_color = quantization.get_dequantized_color(&color);
    let expected_color = color::Rgb::new(252, 254, 252);
    assert_eq!(dequantized_color, expected_color);

    let color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(8, 8, 8)?;
    let dequantized_color = quantization.get_dequantized_color(&color);
    let expected_color = color::Rgb::new(255, 255, 255);
    assert_eq!(dequantized_color, expected_color);

    Ok(())
}

/// A color should be correctly dequantized in place
#[test]
fn quantization_dequantify_color() -> GreenfieldResult<()> {
    let mut color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.dequantify_color(&mut color);
    let expected_color = color::Rgb::new(64, 64, 64);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(1, 1, 1);
    let quantization = UniformQuantization::new(1, 1, 1)?;
    quantization.dequantify_color(&mut color);
    let expected_color = color::Rgb::new(192, 192, 192);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(0, 0, 0);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    quantization.dequantify_color(&mut color);
    let expected_color = color::Rgb::new(4, 2, 4);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(31, 63, 31);
    let quantization = UniformQuantization::new(5, 6, 5)?;
    quantization.dequantify_color(&mut color);
    let expected_color = color::Rgb::new(252, 254, 252);
    assert_eq!(color, expected_color);

    let mut color = color::Rgb::new(255, 255, 255);
    let quantization = UniformQuantization::new(8, 8, 8)?;
    quantization.dequantify_color(&mut color);
    let expected_color = color::Rgb::new(255, 255, 255);
    assert_eq!(color, expected_color);

    Ok(())
}

/// A compressed BitSlice should be correctly decompressed to a Vec of Colors
#[test]
fn quantization_decompress() -> GreenfieldResult<()> {
    let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
    let quantization = UniformQuantization::new(8, 8, 8)?;
    let decompressed = quantization.decompress(&compressed);
    let expected = vec![color::Rgb::new(1, 1, 1)];
    assert_eq!(decompressed, expected);

    let compressed = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let decompressed = quantization.decompress(&compressed);
    let expected = vec![color::Rgb::new(12, 6, 12)];
    assert_eq!(decompressed, expected);

    let compressed =
        bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let decompressed = quantization.decompress(&compressed);
    let expected = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
    assert_eq!(decompressed, expected);

    let compressed = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
    let quantization = UniformQuantization::new(2, 2, 2)?;
    let decompressed = quantization.decompress(&compressed);
    let expected = vec![color::Rgb::new(224, 224, 224)];
    assert_eq!(decompressed, expected);

    // let the world know that we are done
    // let mut compressed = Box::new(BitVec::<u8, Msb0>::new());
    // compressed.resize(1920 * 1080 * 8 * 3, true);
    // let quantization = UniformQuantization::new(8, 8, 8)?;
    // let decompressed = quantization.decompress(&compressed)?;
    // let expected = vec![color::Rgb::new(255, 255, 255)?; 1920 * 1080];
    // assert_eq!(decompressed, expected);

    Ok(())
}

/// A Vec of Colors should be correctly compressed to a BitSlice
#[test]
fn quantization_compress() -> GreenfieldResult<()> {
    let colors = vec![color::Rgb::new(1, 1, 1)];
    let quantization = UniformQuantization::new(8, 8, 8)?;
    let compressed = quantization.compress(&colors);
    let expected = bitvec::bits![u8, Msb0; 0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1];
    assert_eq!(compressed, expected);

    let colors = vec![color::Rgb::new(12, 6, 12)];
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let compressed = quantization.compress(&colors);
    let expected = bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    assert_eq!(compressed, expected);

    let colors = vec![color::Rgb::new(12, 6, 12), color::Rgb::new(12, 6, 12)];
    let quantization = UniformQuantization::new(5, 6, 5)?;
    let compressed = quantization.compress(&colors);
    let expected =
        bitvec::bits![u8, Msb0; 0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1];
    assert_eq!(compressed, expected);

    let colors = vec![color::Rgb::new(224, 224, 224)];
    let quantization = UniformQuantization::new(2, 2, 2)?;
    let compressed = quantization.compress(&colors);
    let expected = bitvec::bits![u8, Msb0; 1,1,1,1,1,1];
    assert_eq!(compressed, expected);

    // let the world know that we are done
    // let colors = vec![color::Rgb::new(255, 255, 255)?; 1920 * 1080];
    // let quantization = UniformQuantization::new(8, 8, 8)?;
    // let compressed = quantization.compress(&colors)?;
    // let mut expected = Box::new(BitVec::<u8, Msb0>::new());
    // expected.resize(1920 * 1080 * 8 * 3, true);
    // assert_eq!(compressed, *expected);

    Ok(())
}
