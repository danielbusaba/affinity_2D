use crate::saturate::saturate;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
use crate::image::GenericImageView;

fn _get_frequencies_abstract(trace: Vec<Vec<u64>>, window_sizes: Vec<usize>) -> (Vec<HashMap<u64, usize>>, Vec<HashMap<(u64, u64), usize>>)
{
    let mut single_frequencies_list: Vec<HashMap<u64, usize>> = Vec::with_capacity(window_sizes.len());
    let mut joint_frequencies_list: Vec<HashMap<(u64, u64), usize>> = Vec::with_capacity(window_sizes.len());
    for size in window_sizes
    {
        let mut single_frequencies: HashMap<u64, usize> = HashMap::new();
        let mut joint_frequencies: HashMap<(u64, u64), usize> = HashMap::new();
        for i in 0 .. (trace.len() - size + 1)
        {
            for j in 0 .. (trace.get(i).unwrap().len() - size + 1)
            {
                let mut singles: HashSet<u64> = HashSet::new();
                let mut doubles: HashSet<(u64, u64)> = HashSet::new();
                for r in i .. i + size
                {
                    for c in j .. j + size
                    {
                        let num = *trace.get(r).unwrap().get(c).unwrap();

                        for sub_num in &singles
                        {
                            if *sub_num != num
                            {
                                if *sub_num < num
                                {
                                    doubles.insert((num, *sub_num));
                                }
                                else
                                {
                                    doubles.insert((*sub_num, num));
                                }
                            }
                        }
                        singles.insert(num);
                    }
                }

                for single in singles
                {
                    if single_frequencies.contains_key(&single)
                    {
                        let current = *single_frequencies.get_mut(&single).unwrap();
                        single_frequencies.insert(single, current + 1);
                    }
                    else
                    {
                        single_frequencies.insert(single, 1);
                    }
                }

                for double in doubles
                {
                    if joint_frequencies.contains_key(&double)
                    {
                        let current = *joint_frequencies.get_mut(&double).unwrap();
                        joint_frequencies.insert(double, current + 1);
                    }
                    else
                    {
                        joint_frequencies.insert(double, 1);
                    }
                }
            }
        }

        single_frequencies_list.push(single_frequencies);
        joint_frequencies_list.push(joint_frequencies);
    }

    (single_frequencies_list, joint_frequencies_list)
}

/*

-------------
| 1 | 2 | 3 |
-------------
| 4 | 5 | 6 |
-------------
| 7 | 8 | 9 |
-------------

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
fn get_frequencies(subimage: &image::SubImage<&image::ImageBuffer<image::Luma<u8>, std::vec::Vec<u8>>>) -> (HashMap<u8, usize>, HashMap<(u8, u8), usize>)
{
    let mut single_frequencies: HashMap<u8, usize> = HashMap::new();
    let mut joint_frequencies: HashMap<(u8, u8), usize> = HashMap::new();

    if subimage.dimensions() == (3, 3)
    {
        // Acquire each pixel in subimage according to diagram above
        let one = subimage.get_pixel(0, 0) [0];
        let two = subimage.get_pixel(0, 1) [0];
        let three = subimage.get_pixel(0, 2) [0];
        let four = subimage.get_pixel(1, 0) [0];
        let five = subimage.get_pixel(1, 1) [0];
        let six = subimage.get_pixel(1, 2) [0];
        let seven = subimage.get_pixel(2, 0) [0];
        let eight = subimage.get_pixel(2, 1) [0];
        let nine = subimage.get_pixel(2, 2) [0];

        // Corner pixels occur once
        single_frequencies.insert(one, 1);
        *single_frequencies.entry(three).or_insert(1) += 1;
        *single_frequencies.entry(seven).or_insert(1) += 1;
        *single_frequencies.entry(nine).or_insert(1) += 1;

        // Off-center pixels occur twice
        *single_frequencies.entry(two).or_insert(2) += 2;
        *single_frequencies.entry(four).or_insert(2) += 2;
        *single_frequencies.entry(six).or_insert(2) += 2;
        *single_frequencies.entry(eight).or_insert(2) += 2;

        // Center pixel occurs four times
        *single_frequencies.entry(five).or_insert(4) += 4;

        // Corner pixels occur once with their neighbors
        joint_insert(&mut joint_frequencies, one, two, 1);
        joint_insert(&mut joint_frequencies, one, four, 1);
        joint_insert(&mut joint_frequencies, one, five, 1);
        joint_insert(&mut joint_frequencies, three, two, 1);
        joint_insert(&mut joint_frequencies, three, six, 1);
        joint_insert(&mut joint_frequencies, three, five, 1);
        joint_insert(&mut joint_frequencies, seven, four, 1);
        joint_insert(&mut joint_frequencies, seven, eight, 1);
        joint_insert(&mut joint_frequencies, seven, five, 1);
        joint_insert(&mut joint_frequencies, nine, six, 1);
        joint_insert(&mut joint_frequencies, nine, eight, 1);
        joint_insert(&mut joint_frequencies, nine, five, 1);

        // Off-center pixels occur twice with the center
        joint_insert(&mut joint_frequencies, two, five, 2);
        joint_insert(&mut joint_frequencies, four, five, 2);
        joint_insert(&mut joint_frequencies, six, five, 2);
        joint_insert(&mut joint_frequencies, eight, five, 2);
    }

    (single_frequencies, joint_frequencies)
}

pub fn analyze_affinity(img: &image::GrayImage, entry: &str, output_dir: &str)
{
    let mut image: image::GrayImage = image::ImageBuffer::new(img.width() - 2, img.height() - 2);
    let now = Instant::now();
    let mut subimage = img.view(0, 0, 3, 3);

    for i in 0 .. img.width() - 2
    {
        for j in 0 .. img.height() - 2
        {
            subimage.change_bounds(i, j, 3, 3);
            let (single, joint) = get_frequencies(&subimage);
            let mut max_diff: u8 = 0;
            let mut max_affinity: f64 = 0.0;
            for (l, r) in joint.keys()
            {
                let a = *single.get(l).unwrap();
                let b = *single.get(r).unwrap();
                let affinity = *joint.get(&(*l, *r)).unwrap() as f64 / (std::cmp::max(a, b) as f64);

                if affinity > max_affinity
                {
                    max_affinity = affinity;
                    max_diff = l - r;
                }
                else if affinity == max_affinity
                {
                    let diff = l - r;
                    if diff > max_diff
                    {
                        max_diff = l - r;
                    }
                }
            }

            image.get_pixel_mut(i, j) [0] = max_diff;
        }
    }
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
