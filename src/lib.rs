use std::io::Cursor;

use wasm_bindgen::prelude::*;

const THRESHOLD: u8 = 30;
const ANCHOR_NUMS: u32 = 5;

#[wasm_bindgen]
pub fn remove_black_bar(input: Vec<u8>) -> Result<Vec<u8>, JsError> {
    let img = match image::load_from_memory(&input) {
        Ok(img) => img,
        Err(_) => return Err(JsError::new("Failed to load image")),
    };
    let mut img_rgb = img.to_rgb8();

    let (width, height) = img_rgb.dimensions();

    if width < ANCHOR_NUMS || height < ANCHOR_NUMS {
        return Err(JsError::new("Image too small"));
    }

    let mut top = height;
    let mut left = width;
    let mut right = 0;
    let mut bottom = 0;

    for i in 0..ANCHOR_NUMS {
        let anchor_width = (width / ANCHOR_NUMS * i) + (width / (ANCHOR_NUMS * 2));
        let anchor_height = (height / ANCHOR_NUMS * i) + (height / (ANCHOR_NUMS * 2));

        let mut selected_top;
        for y in 0..height {
            let pixel = img_rgb.get_pixel(anchor_width, y);
            selected_top = y;

            if !check_pixel_threshold(&pixel) {
                if selected_top < top {
                    top = selected_top;
                }
                break;
            }
        }

        let mut selected_left;
        for x in 0..width {
            let pixel = img_rgb.get_pixel(x, anchor_height);
            selected_left = x;

            if !check_pixel_threshold(&pixel) {
                if selected_left < left {
                    left = selected_left;
                }
                break;
            }
        }

        let mut selected_right = width;
        for x in (0..width).rev() {
            let pixel = img_rgb.get_pixel(x, anchor_height);
            if check_pixel_threshold(&pixel) {
                selected_right = x;
            } else {
                if selected_right > right {
                    right = selected_right;
                }
                break;
            }
        }

        let mut selected_bottom = height;
        for y in (0..height).rev() {
            let pixel = img_rgb.get_pixel(anchor_width, y);
            if check_pixel_threshold(&pixel) {
                selected_bottom = y;
            } else {
                if selected_bottom > bottom {
                    bottom = selected_bottom;
                }
                break;
            }
        }
    }

    if right <= left || bottom <= top {
        return Err(JsError::new("Accidentally removed the whole image"));
    }

    let new_img =
        image::imageops::crop(&mut img_rgb, left, top, right - left, bottom - top).to_image();

    let mut output: Vec<u8> = Vec::new();
    if let Err(_) = new_img.write_to(&mut Cursor::new(&mut output), image::ImageFormat::Png) {
        return Err(JsError::new("Failed to save image"));
    };

    Ok(output)
}

fn check_pixel_threshold(pixel: &image::Rgb<u8>) -> bool {
    pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD
}
