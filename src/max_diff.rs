use crate::saturate::saturate;

use std::time::Instant;

pub fn analyze_max_diff(img: &image::GrayImage, entry: &str, output_dir: &str)
{
    let (width, height) = img.dimensions();
    let mut image: image::GrayImage = image::ImageBuffer::new(width, height);
    let now = Instant::now();
    
    image.enumerate_pixels_mut().for_each(
        | (x, y, pixel) |
        {
            //let subimage = img.view(x, y, 3, 3);
            // let mut min = 255;
            // let mut max = 0;
            // subimage.to_image().iter().for_each(
            //     | sp |
            //     {
            //         let subpixel = *sp;
            //         if subpixel < min
            //         {
            //             min = subpixel;
            //         }
            //         if subpixel > max
            //         {
            //             max = subpixel;
            //         }
            //     }
            // );
            let mut min = 255;
            let mut max = 0;

            let rl = if x > 0 { x - 1 } else { x };
            let rr = if x < img.width() - 1 { x + 1 } else { x };
            let cl = if y > 0 { y - 1 } else { y };
            let cr = if y < img.height() - 1 { y + 1 } else { y };

            for r in rl .. rr
            {
                for c in cl .. cr
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
            *pixel = image::Luma([max - min]);
        }
    );

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
