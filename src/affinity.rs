use crate::saturate::saturate;

use std::collections::HashMap;
use std::time::Instant;
use crate::image::GenericImageView;

/*

-------------   ---------   -------------   ---------
| 1 | 2 | 3 |   | 1 | 2 |   | 1 | 2 | 3 |   | 1 | 2 |
-------------   ---------   -------------   ---------
| 4 | 5 | 6 |   | 3 | 4 |   | 4 | 5 | 6 |   | 3 | 4 |
-------------   ---------   -------------   ---------
| 7 | 8 | 9 |   | 5 | 6 |
-------------   ---------

*/

// Insert the joint frequency with the key beinng sorted
fn joint_insert(map: &mut HashMap<(u8, u8), usize>, a: u8, b: u8, num: usize)
{
    if a > b
    {
        *map.entry((a, b)).or_insert(num) += num;
    }
    else if b < a   // Allow pixels to have affinity with themselves?
    {
        *map.entry((b, a)).or_insert(num) += num;
    }
}

// Gets the single and joint frequencies of pixels in each subimage
fn get_frequencies(subimage: &image::SubImage<&image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>>) -> (Vec<HashMap<u8, usize>>, Vec<HashMap<(u8, u8), usize>>)
{
    let mut single_frequencies: Vec<HashMap<u8, usize>> = Vec::with_capacity(3);
    let mut joint_frequencies: Vec<HashMap<(u8, u8), usize>> = Vec::with_capacity(3);

    for _ in 0 .. 3
    {
        single_frequencies.push(HashMap::new());
        joint_frequencies.push(HashMap::new());
    }

    if subimage.dimensions() == (3, 3)
    {
        for i in 0 .. 3
        {
            // Acquire each pixel in subimage according to diagram above
            let one = subimage.get_pixel(0, 0) [i];
            let two = subimage.get_pixel(0, 1) [i];
            let three = subimage.get_pixel(0, 2) [i];
            let four = subimage.get_pixel(1, 0) [i];
            let five = subimage.get_pixel(1, 1) [i];
            let six = subimage.get_pixel(1, 2) [i];
            let seven = subimage.get_pixel(2, 0) [i];
            let eight = subimage.get_pixel(2, 1) [i];
            let nine = subimage.get_pixel(2, 2) [i];

            // Corner pixels occur once
            single_frequencies [i].insert(one, 1);
            *single_frequencies [i].entry(three).or_insert(1) += 1;
            *single_frequencies [i].entry(seven).or_insert(1) += 1;
            *single_frequencies [i].entry(nine).or_insert(1) += 1;

            // Off-center pixels occur twice
            *single_frequencies [i].entry(two).or_insert(2) += 2;
            *single_frequencies [i].entry(four).or_insert(2) += 2;
            *single_frequencies [i].entry(six).or_insert(2) += 2;
            *single_frequencies [i].entry(eight).or_insert(2) += 2;

            // Center pixel occurs four times
            *single_frequencies [i].entry(five).or_insert(4) += 4;

            // Corner pixels occur once with their neighbors
            joint_insert(&mut joint_frequencies [i], one, two, 1);
            joint_insert(&mut joint_frequencies [i], one, four, 1);
            joint_insert(&mut joint_frequencies [i], one, five, 1);
            joint_insert(&mut joint_frequencies [i], three, two, 1);
            joint_insert(&mut joint_frequencies [i], three, six, 1);
            joint_insert(&mut joint_frequencies [i], three, five, 1);
            joint_insert(&mut joint_frequencies [i], seven, four, 1);
            joint_insert(&mut joint_frequencies [i], seven, eight, 1);
            joint_insert(&mut joint_frequencies [i], seven, five, 1);
            joint_insert(&mut joint_frequencies [i], nine, six, 1);
            joint_insert(&mut joint_frequencies [i], nine, eight, 1);
            joint_insert(&mut joint_frequencies [i], nine, five, 1);

            // Off-center pixels occur once with each other
            joint_insert(&mut joint_frequencies [i], four, two, 1);
            joint_insert(&mut joint_frequencies [i], four, eight, 1);
            joint_insert(&mut joint_frequencies [i], six, two, 1);
            joint_insert(&mut joint_frequencies [i], six, eight, 1);

            // Off-center pixels occur twice with the center
            joint_insert(&mut joint_frequencies [i], two, five, 2);
            joint_insert(&mut joint_frequencies [i], four, five, 2);
            joint_insert(&mut joint_frequencies [i], six, five, 2);
            joint_insert(&mut joint_frequencies [i], eight, five, 2);
        }
    }
    else if subimage.dimensions() == (2, 3)
    {
        for i in 0 .. 3
        {
            // Acquire each pixel in subimage according to diagram above
            let one = subimage.get_pixel(0, 0) [i];
            let two = subimage.get_pixel(0, 1) [i];
            let three = subimage.get_pixel(0, 2) [i];
            let four = subimage.get_pixel(1, 0) [i];
            let five = subimage.get_pixel(1, 1) [i];
            let six = subimage.get_pixel(1, 2) [i];

            // Corner pixels occur once
            single_frequencies [i].insert(one, 1);
            *single_frequencies [i].entry(three).or_insert(1) += 1;
            *single_frequencies [i].entry(four).or_insert(1) += 1;
            *single_frequencies [i].entry(six).or_insert(1) += 1;

            // Off-center pixels occur twice
            *single_frequencies [i].entry(two).or_insert(2) += 2;
            *single_frequencies [i].entry(five).or_insert(2) += 2;

            // Corner pixels occur once with their neighbors
            joint_insert(&mut joint_frequencies [i], one, two, 1);
            joint_insert(&mut joint_frequencies [i], one, four, 1);
            joint_insert(&mut joint_frequencies [i], one, five, 1);
            joint_insert(&mut joint_frequencies [i], four, two, 1);
            joint_insert(&mut joint_frequencies [i], four, five, 1);
            joint_insert(&mut joint_frequencies [i], three, two, 1);
            joint_insert(&mut joint_frequencies [i], three, five, 1);
            joint_insert(&mut joint_frequencies [i], three, six, 1);
            joint_insert(&mut joint_frequencies [i], six, two, 1);
            joint_insert(&mut joint_frequencies [i], six, five, 1);

            // Off-center pixels occur twice with each other
            joint_insert(&mut joint_frequencies [i], two, five, 2);
        }
    }
    else if subimage.dimensions() == (3, 2)
    {
        for i in 0 .. 3
        {
            // Acquire each pixel in subimage according to diagram above
            let one = subimage.get_pixel(0, 0) [i];
            let two = subimage.get_pixel(0, 1) [i];
            let three = subimage.get_pixel(1, 0) [i];
            let four = subimage.get_pixel(1, 1) [i];
            let five = subimage.get_pixel(2, 0) [i];
            let six = subimage.get_pixel(2, 1) [i];

            // Corner pixels occur once
            single_frequencies [i].insert(one, 1);
            *single_frequencies [i].entry(two).or_insert(1) += 1;
            *single_frequencies [i].entry(five).or_insert(1) += 1;
            *single_frequencies [i].entry(six).or_insert(1) += 1;

            // Off-center pixels occur twice
            *single_frequencies [i].entry(three).or_insert(2) += 2;
            *single_frequencies [i].entry(four).or_insert(2) += 2;

            // Corner pixels occur once with their neighbors
            joint_insert(&mut joint_frequencies [i], one, two, 1);
            joint_insert(&mut joint_frequencies [i], one, three, 1);
            joint_insert(&mut joint_frequencies [i], one, four, 1);
            joint_insert(&mut joint_frequencies [i], two, three, 1);
            joint_insert(&mut joint_frequencies [i], two, four, 1);
            joint_insert(&mut joint_frequencies [i], five, three, 1);
            joint_insert(&mut joint_frequencies [i], five, four, 1);
            joint_insert(&mut joint_frequencies [i], five, six, 1);
            joint_insert(&mut joint_frequencies [i], six, three, 1);
            joint_insert(&mut joint_frequencies [i], six, four, 1);

            // Off-center pixels occur twice with each other
            joint_insert(&mut joint_frequencies [i], three, four, 2);
        }
    }
    else
    {
        panic!("Invalid subimage dimension: {:?}", subimage.dimensions());
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

            // Handles corners with only one window
            if subimage.dimensions() == (2, 2)
            {
                let mut out = [0; 3];
                for i in 0 .. 3
                {
                    let mut min = 255;
                    let mut max = 0;
                    subimage.pixels().for_each(
                        | (_, _, px) |
                        {
                            let px = px [i];
                            if px < min
                            {
                                min = px;
                            }
                            if px > max
                            {
                                max = px;
                            }
                        }
                    );
                    out [i] = max - min;
                }
                *pixel = image::Rgb(out);
            }
            else
            {
                // Find the strongest affinity with the largest pixel difference
                let (single, joint) = get_frequencies(&subimage);
                
                let mut out = [0; 3];
                for i in 0 .. 3
                {
                    let mut max_affinity: f64 = 0.0;
                    for (l, r) in joint [i].keys()
                    {
                        let a = *single [i].get(l).unwrap();
                        let b = *single [i].get(r).unwrap();
                        let affinity = *joint [i].get(&(*l, *r)).unwrap() as f64 / (std::cmp::max(a, b) as f64);

                        if affinity > max_affinity
                        {
                            max_affinity = affinity;
                            out [i] = l - r;
                        }
                        else if affinity == max_affinity
                        {
                            let diff = l - r;
                            if diff > out [i]
                            {
                                out [i] = l - r;
                            }
                        }
                    }
                }

                *pixel = image::Rgb(out);
            }
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
