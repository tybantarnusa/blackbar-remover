use std::io::Cursor;
use wasm_bindgen::prelude::*;

const THRESHOLD: u8 = 25;

#[wasm_bindgen]
pub fn remove_black_bar(input: Vec<u8>) -> Vec<u8> {
    let img = image::load_from_memory(&input)
        .unwrap_or_else(|err| panic!("Failed to load image: {err:?}"));
    let mut img_rgb = img.to_rgb8();

    let (width, height) = img_rgb.dimensions();

    let half_width = width / 2;
    let half_height = height / 2;

    let mut top = 0;
    for y in 0..height {
        let pixel = img_rgb.get_pixel(half_width, y);
        if pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD {
            top = y;
        } else {
            break;
        }
    }

    let mut left = 0;
    for x in 0..width {
        let pixel = img_rgb.get_pixel(x, half_height);
        if pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD {
            left = x;
        } else {
            break;
        }
    }

    let mut right = width-1;
    for x in (0..width).rev() {
        let pixel = img_rgb.get_pixel(x, half_height);
        if pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD {
            right = x;
        } else {
            break;
        }
    }

    let mut bottom = height-1;
    for y in (0..height).rev() {
        let pixel = img_rgb.get_pixel(half_width, y);
        if pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD {
            bottom = y;
        } else {
            break;
        }
    }

    let new_img = image::imageops::crop(&mut img_rgb, left, top, right - left, bottom - top).to_image();

    let mut output: Vec<u8> = Vec::new();
    new_img
        .write_to(&mut Cursor::new(&mut output), image::ImageFormat::Png)
        .unwrap_or_else(|err| panic!("Failed to write image: {err:?}"));

    output
}
