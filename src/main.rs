use std::io::Cursor;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return Ok(());
    }

    let img = image::open(&args[1])?;
    let img_rgb = img.to_rgb8();

    let mut buf: Vec<u8> = Vec::new();
    img_rgb.write_to(&mut Cursor::new(&mut buf), image::ImageFormat::Png)?;

    let result = blackbar_remover::remove_black_bar(buf);

    let result_image = image::load_from_memory(&result).unwrap();
    result_image.save("output.png").unwrap();

    Ok(())
}
