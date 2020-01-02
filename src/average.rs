use crate::saturate::saturate;

use std::time::Instant;

pub fn analyze_average(original: &image::GrayImage, analyzed: &image::GrayImage, entry: &str, output_dir: &str)
{
    // Setup image to be copied to and start counting time
    let (width, height) = analyzed.dimensions();
    let mut image: image::GrayImage = image::ImageBuffer::new(width, height);
    let now = Instant::now();

    image.enumerate_pixels_mut().for_each(
        | (x, y, pixel) |
        {
            *pixel = image::Luma([ ((original.get_pixel(x, y) [0] as u16 + analyzed.get_pixel(x, y) [0] as u16) / 2) as u8 ]);
        }
    );

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Average Analysis Completed in: {}", sec);
    image.save(output_dir.to_owned() + entry).unwrap();
    let now = Instant::now();
    saturate(&mut image);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Output Saturated in: {}", sec);
    image.save("saturated_".to_owned() + output_dir + entry).unwrap();
}
