use super::*;
use std::env;

#[test]
/// Should convert images between formats
fn io_image() -> GreenfieldResult<()> {
    let base_path = env::current_dir()?.join("../../../").join("assets");

    // Should load a image on a common format from disk, convert it to a Greenfield image and save
    // it as a png
    let path = PathBuf::clone(&base_path).join("Lenna.png");
    let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
    let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
    let _ = save_image(&img, &path)?;

    // Should load a image on a common format from disk, convert it to a Greenfield image and save
    // it as a Greenfield image
    let path = PathBuf::clone(&base_path).join("Lenna.png");
    let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
    let path = PathBuf::clone(&base_path).join("Lenna.gfd");
    let _ = save_image(&img, &path)?;

    // Should load a Greenfield image from disk and save it as a png
    let path = PathBuf::clone(&base_path).join("Lenna.gfd");
    let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
    let path = PathBuf::clone(&base_path).join("Lenna.gfd.png");
    let _ = save_image(&img, &path)?;

    // Should load a Greenfield image from disk and save it as a Greenfield image
    let path = PathBuf::clone(&base_path).join("Lenna.gfd");
    let img = load_image(&path, UniformQuantization::new(5, 6, 5)?)?;
    let path = PathBuf::clone(&base_path).join("Lenna.gfd");
    let _ = save_image(&img, &path)?;

    // clean up
    std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd.png"))?;
    std::fs::remove_file(PathBuf::clone(&base_path).join("Lenna.gfd"))?;

    Ok(())
}
