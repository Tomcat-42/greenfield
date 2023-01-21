use super::*;
use crate::prelude::*;

#[test]
/// Should create a new pixel
fn pixel_new() -> GreenfieldResult<()> {
    let color = color::Rgb::new(0, 0, 0);
    let pixel = Pixel::new(0, 0, &color);

    let Pixel { x, y, color } = pixel;
    let color::Rgb { r, g, b } = color;

    assert_eq!(x, 0);
    assert_eq!(y, 0);
    assert_eq!(*r, 0);
    assert_eq!(*g, 0);
    assert_eq!(*b, 0);

    Ok(())
}

#[test]
/// Should Display a pixel
fn pixel_display() -> GreenfieldResult<()> {
    let color = color::Rgb::random();
    let pixel = Pixel::new(0, 0, &color);

    println!("{}", pixel);

    Ok(())
}

#[test]
/// Should debug a pixel
fn pixel_debug() -> GreenfieldResult<()> {
    let color = color::Rgb::random();
    let pixel = Pixel::new(0, 0, &color);

    println!("{:?}", pixel);

    Ok(())
}
