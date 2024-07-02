use image::{ImageBuffer, Rgb, RgbImage};
use noise::{Fbm, NoiseFn, Perlin};
use palette::{Hsv, Okhsl, Okhsv, Srgb};
use palette::convert::FromColorUnclamped;

// use palette::{rgb::Rgb, Okhsv};

fn pattern(fbm: &noise::Fbm<Perlin>, p: [f64; 2], q: &mut [f64; 2], r: &mut [f64; 2], time: f64) -> f64 {
    q[0] = fbm.get([p[0] + 0.0, p[1] + 0.0]);
    q[1] = fbm.get([p[0] + 5.2 * time, p[1] + 1.3]);

    q[0] *= 0.7 + 0.2 * (0.05 * time).cos() as f64;
    q[1] *= 0.7 + 0.2 * (0.05 * time).cos() as f64;

    r[0] = fbm.get([p[0] + 4.0 * q[0] + 1.7, p[1] + 4.0 * q[1] + 9.2]);
    r[1] = fbm.get([p[0] + 4.0 * q[0] + 8.3, p[1] + 4.0 * q[1] + 2.8]);

    fbm.get([p[0] + 4.0 * r[0], p[1] + 4.0 * r[1]])
}

fn dot(v: &[f64; 2]) -> f64 {
    (v[0] * v[0] + v[1] * v[1])
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0 - t) + b * t
}

fn mix(c0: Okhsl, c1: Okhsl, t: f64) -> Okhsl {
    Okhsl::new(
        lerp(c0.hue.into_positive_degrees() as f64, c1.hue.into_positive_degrees() as f64, t) as f32,
        lerp(c0.saturation as f64, c1.saturation as f64, t) as f32,
        lerp(c0.lightness as f64, c1.lightness as f64, t) as f32,
    )
}

/*
q *= 0.7 + 0.2*cos(0.05*iTime);
 */

fn map_to_color(q: &[f64; 2], r: &[f64; 2], m: f64) -> Okhsl {
    let mut c = Okhsl::new(261.0 as f32, 58.0 / 100.0 as f32, 40.0 / 100.0 as f32);
    c.saturation = lerp(c.saturation as f64, 1.0, dot(q)) as f32;
    c.lightness = lerp(c.lightness as f64, 1.0, 0.5 * r[0] * r[0]) as f32;
    return c;
}

fn generate_image_from_fbm(t: f64) -> RgbImage {
    let fbm: noise::Fbm<Perlin> = Fbm::new(0);
    let mut img = RgbImage::new(512, 512);
    let mut q_buf = [0.0, 0.0];
    let mut r_buf = [0.0, 0.0];
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let p = [x as f64 / 64.0, y as f64 / 64.0];
        let value: f64 = pattern(&fbm, p, &mut q_buf, &mut r_buf, t);
        let okhsv = map_to_color(&q_buf, &r_buf, value);
        let rgb = Srgb::from_color_unclamped(okhsv);
        let value = (value + 1.0) * 127.5;
        let r = (rgb.red * 255.0).min(255.0) as u8;
        let g = (rgb.green * 255.0).min(255.0) as u8;
        let b = (rgb.blue * 255.0).min(255.0) as u8;
        *pixel = Rgb([r, g, b]);
    }
    img
}

fn main() {
    for i in 0..30 {
        let img = generate_image_from_fbm(i as f64 / 40.0);
        img.save(format!("images/{:0>3}.png", i)).unwrap();
    }
    println!("Hello, world!");
}
