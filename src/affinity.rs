use crate::saturate::saturate;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use crate::image::GenericImageView;

// Gets the single and joint frequencies of pixels in each subimage
fn get_frequencies(subimage: &image::SubImage<&image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>>) -> (Vec<HashMap<u8, usize>>, Vec<HashMap<(u8, u8), usize>>)
{
    // HashMaps for each color channel to represent co-occurrences and single occurrences
    let mut single_frequencies: Vec<HashMap<u8, usize>> = Vec::with_capacity(3);
    let mut joint_frequencies: Vec<HashMap<(u8, u8), usize>> = Vec::with_capacity(3);

    // Initialize single and joint frequency vectors
    for _ in 0 .. 3
    {
        single_frequencies.push(HashMap::new());
        joint_frequencies.push(HashMap::new());
    }

    // Iterate through the subimage
    let (width, height) = subimage.dimensions();
    for i in 0 .. width - 1
    {
        for j in 0 .. height - 1
        {
            // Remember what pairs and singles we have seen in the sliding window
            let mut singles: Vec<HashSet<u8>> = Vec::with_capacity(3);
            let mut doubles: Vec<HashSet<(u8, u8)>> = Vec::with_capacity(3);

            // Initialize seen pixel HashSets
            for _ in 0 .. 3
            {
                singles.push(HashSet::new());
                doubles.push(HashSet::new());
            }

            // Iterate through window
            for r in i .. i + 2
            {
                for c in j .. j + 2
                {
                    // Iterate through each color channel
                    for p in 0 .. 3
                    {
                        let pixel = subimage.get_pixel(r, c) [p];
                        for seen_pixel in &singles [p]
                        {
                            let seen_pixel = *seen_pixel;
                            // Handle pair if affinity is not with self
                            if true //seen_pixel != pixel
                            {
                                let tuple = if seen_pixel < pixel
                                            {
                                                (pixel, seen_pixel)
                                            }
                                            else
                                            {
                                                (seen_pixel, pixel)
                                            };
                                
                                // Remember seen pair and increment joint frequency
                                if !doubles [p].contains(&tuple)
                                {
                                    *joint_frequencies [p].entry(tuple).or_insert(1) += 1;
                                    doubles [p].insert(tuple);
                                }
                            }
                        }

                        // Remember individual and increment single frequency
                        if !singles [p].contains(&pixel)
                        {
                            *single_frequencies [p].entry(pixel).or_insert(1) += 1;
                            singles [p].insert(pixel);
                        }
                    }
                }
            }
        }
    }

    (single_frequencies, joint_frequencies)
}

// Uses affinity analysis to set each pixel to the highest affinity in a 3x3 square around it
pub fn analyze_affinity(img: &image::RgbImage, entry: &str, output_dir: &str)
{
    // Setup image to be copied to and start counting time
    let (width, height) = img.dimensions();
    let mut image: image::RgbImage = image::ImageBuffer::new(width, height);
    let now = Instant::now();
    let mut subimage = img.view(0, 0, 2, 2);

    image.enumerate_pixels_mut().for_each(
        | (x, y, pixel) |
        {
            // Acquire subimage based on bounds
            let xb = if x > 0 { x - 1 } else { x };
            let yb = if y > 0 { y - 1 } else { y };
            let wb = if x > 0 && x < width - 1 { 3 } else { 2 };
            let hb = if y > 0 && y < height - 1 { 3 } else { 2 };
            subimage.change_bounds(xb, yb, wb, hb);

            // Find the strongest affinity with the largest pixel difference
            let (_, joint) = get_frequencies(&subimage);
            
            let mut out = [0; 3];
            for i in 0 .. 3
            {
                // let mut max_affinity: f64 = 0.0;
                let mut max_cooccurance = 0;
                for (l, r) in joint [i].keys()
                {
                    // // Divide the joint frequency by the minimum of the single frequencies to get the largest conditional probability of the pair
                    // let a = *single [i].get(l).unwrap();
                    // let b = *single [i].get(r).unwrap();
                    // let affinity = *joint [i].get(&(*l, *r)).unwrap() as f64 / (std::cmp::min(a, b) as f64);

                    // // Keep widest affinity with the largest pixel difference
                    // if affinity > max_affinity
                    // {
                    //     max_affinity = affinity;
                    //     out [i] = l - r;
                    // }
                    // else if affinity == max_affinity
                    // {
                    //     let diff = l - r;
                    //     if diff > out [i]
                    //     {
                    //         out [i] = l - r;
                    //     }
                    // }
                    
                    let cooccurance = *joint [i].get(&(*l, *r)).unwrap();
                    if cooccurance > max_cooccurance
                    {
                        max_cooccurance = cooccurance;
                        out [i] = l - r;
                    }
                    else if cooccurance == max_cooccurance
                    {
                        let diff = l - r;
                        if diff > out [i]
                        {
                            out [i] = l - r;
                        }
                    }
                }
            }

            // Assign center pixel to be the difference between the farthest two pixels with the highest affinity
            *pixel = image::Rgb(out);
        }
    );

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Affinity Analysis Completed in: {}", sec);
    image.save(output_dir.to_owned() + entry).unwrap();
    let now = Instant::now();
    saturate(&mut image);
    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Output Saturated in: {}", sec);
    image.save("saturated_".to_owned() + output_dir + entry).unwrap();
}
