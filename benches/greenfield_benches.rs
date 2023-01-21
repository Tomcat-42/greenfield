use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;

use greenfield::prelude::*;
use std::{env, path::PathBuf};

lazy_static! {
    static ref ASSETS_DIR: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("benches/assets/");
        path
    };
}

// Small images benches
/// load a small png, convert to Greenfield M N O, save to disk as png
fn small_png_gfd_png(c: &mut Criterion) {
    c.bench_function("small_png_gfd565_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_565.png"),
            )
        })
    });

    c.bench_function("small_png_gfd222_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_222.png"),
            )
        })
    });

    c.bench_function("small_png_gfd888_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_888.png"),
            )
        })
    });
}

/// load a small png, convert to Greenfield M N O, save to disk as greenfield
fn small_png_gfd_gfd(c: &mut Criterion) {
    c.bench_function("small_png_gfd565_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_565.gfd"),
            )
        })
    });

    c.bench_function("small_png_gfd222_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_222.gfd"),
            )
        })
    });

    c.bench_function("small_png_gfd888_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna.png"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_888.gfd"),
            )
        })
    });
}

/// load a small gfd, convert to Greenfield M N O, save to disk as png
fn small_gfd_gfd_png(c: &mut Criterion) {
    c.bench_function("small_gfd_gfd565_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna_gfd_565.gfd"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_565.png"),
            )
        })
    });

    c.bench_function("small_gfd_gfd222_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna_gfd_222.gfd"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_222.png"),
            )
        })
    });

    c.bench_function("small_gfd_gfd888_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("lenna_gfd_888.gfd"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap())
                    .join("benches/assets/lenna_gfd_888.png"),
            )
        })
    });
}

// Big images benches
/// load a big png(4k), convert to Greenfield M N O, save to disk as png
fn big_png_gfd_png(c: &mut Criterion) {
    c.bench_function("big_png_gfd565_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_565.png"),
            )
        })
    });

    c.bench_function("big_png_gfd222_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_222.png"),
            )
        })
    });

    c.bench_function("big_png_gfd888_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_888.png"),
            )
        })
    });
}

/// load a big png(4k), convert to Greenfield M N O, save to disk as greenfield
fn big_png_gfd_gfd(c: &mut Criterion) {
    c.bench_function("big_png_gfd565_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_565.gfd"),
            )
        })
    });

    c.bench_function("big_png_gfd222_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_222.gfd"),
            )
        })
    });

    c.bench_function("big_png_gfd888_gfd", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k.png"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_888.gfd"),
            )
        })
    });
}

/// load a big gfd(4k), convert to Greenfield M N O, save to disk as png
fn big_gfd_gfd_png(c: &mut Criterion) {
    c.bench_function("big_gfd_gfd565_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k_gfd_565.gfd"),
                UniformQuantization::new(5, 6, 5).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_565.png"),
            )
        })
    });

    c.bench_function("big_gfd_gfd222_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k_gfd_222.gfd"),
                UniformQuantization::new(2, 2, 2).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_222.png"),
            )
        })
    });

    c.bench_function("big_gfd_gfd888_png", |b| {
        b.iter(|| {
            let gfd_image = load_image(
                &PathBuf::clone(&ASSETS_DIR).join("4k_gfd_888.gfd"),
                UniformQuantization::new(8, 8, 8).unwrap(),
            )
            .unwrap();

            save_image(
                &gfd_image,
                &PathBuf::from(env::current_dir().unwrap()).join("benches/assets/4k_gfd_888.png"),
            )
        })
    });
}

criterion_group!(
    benches,
    small_png_gfd_png,
    small_png_gfd_gfd,
    small_gfd_gfd_png,
    big_png_gfd_png,
    big_png_gfd_gfd,
    big_gfd_gfd_png
);
criterion_main!(benches);
