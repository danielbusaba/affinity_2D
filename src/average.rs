use crate::saturate::saturate;

use std::time::Instant;

pub fn analyze_average(original: &image::GrayImage, analyzed: &image::GrayImage, entry: &str, output_dir: &str)
{
    let mut image: image::GrayImage = image::ImageBuffer::new(analyzed.width(), analyzed.height());
    let now = Instant::now();
    for i in 0 .. analyzed.width()
    {
        for j in 0 .. analyzed.height()
        {
            image.get_pixel_mut(i, j) [0] = ((original.get_pixel(i + 1, j + 1) [0] as u16 + analyzed.get_pixel(i, j) [0] as u16) / 2) as u8;
        }
    }

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
