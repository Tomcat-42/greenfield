use super::*;

#[test]
/// Should create a new RGB color
fn color_rgb_new() {
    let color = Rgb::new(200, 150, 10);
    let Rgb { r, g, b } = color;

    assert_eq!(r, 200);
    assert_eq!(g, 150);
    assert_eq!(b, 10);
}

#[test]
/// Should create a new default RGB color
fn color_rgb_default() {
    let color = Rgb::default();
    let Rgb { r, g, b } = color;

    assert_eq!(r, 0);
    assert_eq!(g, 0);
    assert_eq!(b, 0);
}

#[test]
/// Should create a new random RGB color
fn color_rgb_random() {
    let _color = Rgb::random();
}

#[test]
/// Should Display a RGB color
fn color_rgb_display() {
    let color = Rgb::random();
    println!("{}", color);
}

#[test]
/// Should return a RGB color as a tuple of bytes
fn color_rgb_bytes() {
    let color = Rgb::new(200, 150, 10);
    let [r, g, b] = color.bytes();

    assert_eq!(r, 200);
    assert_eq!(g, 150);
    assert_eq!(b, 10);
}
