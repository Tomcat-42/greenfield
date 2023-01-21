use super::*;

#[test]
/// Should create a new RGB color
fn color_rgb_new() -> GreenfieldResult<()> {
    let color = Rgb::new(200, 150, 10);
    let Rgb { r, g, b } = color;

    assert_eq!(r, 200);
    assert_eq!(g, 150);
    assert_eq!(b, 10);

    Ok(())
}

#[test]
/// Should create a new default RGB color
fn color_rgb_default() -> GreenfieldResult<()> {
    let color = Rgb::default();
    let Rgb { r, g, b } = color;

    assert_eq!(r, 0);
    assert_eq!(g, 0);
    assert_eq!(b, 0);

    Ok(())
}

#[test]
/// Should create a new random RGB color
fn color_rgb_random() -> GreenfieldResult<()> {
    let _color = Rgb::random();
    Ok(())
}

#[test]
/// Should Display a RGB color
fn color_rgb_display() -> GreenfieldResult<()> {
    let color = Rgb::random();
    println!("{}", color);

    Ok(())
}

#[test]
/// Should return a RGB color as a tuple of bytes
fn color_rgb_bytes() -> GreenfieldResult<()> {
    let color = Rgb::new(200, 150, 10);
    let [r, g, b] = color.bytes();

    assert_eq!(r, 200);
    assert_eq!(g, 150);
    assert_eq!(b, 10);

    Ok(())
}
