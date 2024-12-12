use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    const THRESHOLD: u8 = 50;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let img = image::open(&args[1])?;
    let img_rgb = img.to_rgb8();

    let mut start_point = (0, 0);
    let mut end_point = (0, 0);
    for (x, y, pixel) in img_rgb.enumerate_pixels() {
        if (pixel[0] > THRESHOLD || pixel[1] > THRESHOLD || pixel[2] > THRESHOLD) && start_point == (0, 0) {
            start_point = (x, y);
        }

        if start_point != (0, 0) {
            if pixel[0] <= THRESHOLD && pixel[1] <= THRESHOLD && pixel[2] <= THRESHOLD {
                if end_point == (0, 0) {
                    end_point = (x, y);
                }
            } else {
                end_point = (0, 0);
            }
        }

        if x == img_rgb.dimensions().0-1 {
            break;
        }
    }

    let mut new_img = image::RgbImage::new(
        end_point.0 - start_point.0,
        img_rgb.dimensions().1,
    );

    for x in start_point.0..end_point.0 {
        for y in 0..img_rgb.dimensions().1 {
            let pixel = img_rgb.get_pixel(x, y);
            new_img.put_pixel(x - start_point.0, y, *pixel);
        }
    }

    new_img.save("output.png")?;
    println!("Saved to output.png");

    Ok(())
}
