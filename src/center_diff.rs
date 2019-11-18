use crate::saturate::saturate;

use std::time::Instant;

pub fn analyze_center_diff(img: &image::GrayImage, entry: &str, output_dir: &str)
{
    let mut image: image::GrayImage = image::ImageBuffer::new(img.width() - 2, img.height() - 2);
    let now = Instant::now();
    for i in 0 .. img.width() - 2
    {
        for j in 0 .. img.height() - 2
        {
            let mut max = 0;
            for r in i .. i + 3
            {
                for c in j .. j + 3
                {
                    let num = img.get_pixel(r, c) [0];
                    let center = img.get_pixel(i + 1, j + 1) [0];
                    if num > center
                    {
                        let diff = num - center;
                        if diff > max
                        {
                            max = diff;
                        }
                    }
                    else
                    {
                        let diff = center - num;
                        if diff > max
                        {
                            max = diff;
                        }
                    }
                }
            }

            image.get_pixel_mut(i, j) [0] = max;
        }
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Center Diff Analysis Completed in: {}", sec);
    image.save(output_dir.to_owned() + entry).unwrap();
    let now = Instant::now();
    saturate(&mut image);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Output Saturated in: {}", sec);
    image.save("saturated_".to_owned() + output_dir + entry).unwrap();
}
