use crate::saturate::saturate;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn get_frequencies_naive(trace: Vec<Vec<u64>>, window_sizes: Vec<usize>) -> (Vec<HashMap<u64, usize>>, Vec<HashMap<(u64, u64), usize>>)
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

pub fn analyze_affinity(img: &image::GrayImage, entry: &str, output_dir: &str)
{
    let mut image: image::GrayImage = image::ImageBuffer::new(img.width() - 2, img.height() - 2);
    let now = Instant::now();
    for i in 0 .. img.width() - 2
    {
        for j in 0 .. img.height() - 2
        {
            let mut square: Vec<Vec<u64>> = Vec::with_capacity(3);
            let mut not_zero = false;
            for r in i .. i + 3
            {
                let mut col = Vec::with_capacity(3);
                for c in j .. j + 3
                {
                    let num = img.get_pixel(r, c) [0];
                    if num != 0
                    {
                        not_zero = true;
                    }
                    col.push(num as u64);
                }
                square.push(col);
            }

            if not_zero
            {
                let frequencies = get_frequencies_naive(square, vec!(2));
                let mut max_diff: u8 = 0;
                let mut max_affinity: f64 = 0.0;
                for pair in frequencies.1 [0].keys()
                {
                    let single_frequecy_a = *frequencies.0 [0].get(&pair.0).unwrap() as f64;
                    let single_frequecy_b = *frequencies.0 [0].get(&pair.1).unwrap() as f64;
                    let mut affinity = *frequencies.1 [0].get(pair).unwrap() as f64;
                    if single_frequecy_b < single_frequecy_a
                    {
                        affinity = affinity / single_frequecy_b;
                    }
                    else
                    {
                        affinity = affinity / single_frequecy_a;
                    }

                    if affinity > max_affinity
                    {
                        max_affinity = affinity;
                        if pair.0 > pair.1
                        {
                            max_diff = (pair.0 - pair.1) as u8;
                        }
                        else
                        {
                            max_diff = (pair.1 - pair.0) as u8;
                        }
                    }
                    else if affinity == max_affinity
                    {
                        if pair.0 > pair.1
                        {
                            let diff = (pair.0 - pair.1) as u8;
                            if diff > max_diff
                            {
                                max_diff = (pair.0 - pair.1) as u8;
                            }
                        }
                        else
                        {
                            let diff = (pair.1 - pair.0) as u8;
                            if diff > max_diff
                            {
                                max_diff = (pair.1 - pair.0) as u8;
                            }
                        }
                    }
                }

                image.get_pixel_mut(i, j) [0] = max_diff;
            }
            else
            {
                image.get_pixel_mut(i, j) [0] = 0;
            }
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
