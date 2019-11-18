use crate::saturate::saturate;

use std::time::Instant;

pub fn analyze_max_diff(img: &image::GrayImage, entry: &str, output_dir: &str)
{
    let mut image: image::GrayImage = image::ImageBuffer::new(img.width() - 2, img.height() - 2);
    let now = Instant::now();
    for i in 0 .. img.width() - 2
    {
        for j in 0 .. img.height() - 2
        {
            let mut min = 255;
            let mut max = 0;
            for r in i .. i + 3
            {
                for c in j .. j + 3
                {
                    let num = img.get_pixel(r, c) [0];
                    if num < min
                    {
                        min = num;
                    }
                    if num > max
                    {
                        max = num;
                    }
                }
            }

            image.get_pixel_mut(i, j) [0] = max - min;
        }
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Max Diff Analysis Completed in: {}", sec);
    image.save(output_dir.to_owned() + entry).unwrap();
    let now = Instant::now();
    saturate(&mut image);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Output Saturated in: {}", sec);
    image.save("saturated_".to_owned() + output_dir + entry).unwrap();
}
