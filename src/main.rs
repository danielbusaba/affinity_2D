#![allow(dead_code)]                //Removes annoying unused function warnings
#![allow(unused_imports)]           //Removes annoying unused import warnings

//Imports for code
extern crate image;                 //Used for image processing
use std::fs;                        //Used for file I/O
use std::fs::File;                  //Used for writing to files
use std::path::Path;                //Used for accessing files
use std::env;                       //Used for command line arguments
use std::collections::HashMap;      //Imports HashMap data structure
use std::collections::HashSet;      //Imports HashMap data structure
use image::ImageDecoder;            //Decodes images
use image::GenericImageView;        //Gets image meta data
use image::ImageBuffer;             //Raw image data from disk
use std::time::Instant;             //Allows for timing computations

//Imports for tests
extern crate rand;                  //Used for testing random cases
use rand::{thread_rng, Rng};        //Used for random number generation
use std::io::Write;                 //Used for writing to files

const TRACE_DIR: &str = "traces";            //Stores trace directory globally
const MAMMOGRAM_DIR: &str = "../../mammograms";    //Stores trace directory globally
const BASE_DIR: &str = "base";                 //Stores the base storage directory globally
const SATURATED_DIR: &str = "saturated";      //Stores saturated output directory globally
const OUTPUT_DIR: &str = "output";           //Stores output directory globally
const OUTPUT_MAX_DIFF_DIR: &str = "output_max_diff";           //Stores output directory globally
const OUTPUT_CENTER_DIFF_DIR: &str = "output_center_diff";           //Stores output directory globally
const OUTPUT_AVERAGE_DIR: &str = "output_average";           //Stores output directory globally

const DIRS: [&str; 6] = [BASE_DIR, SATURATED_DIR, OUTPUT_DIR, OUTPUT_MAX_DIFF_DIR, OUTPUT_CENTER_DIFF_DIR, OUTPUT_AVERAGE_DIR];

fn get_trace_file(filename: String, x: usize, y: usize) -> Vec<Vec<u64>>    //Converts a file of numbers seperated by spaces and new lines into a 2D array of those numbers
{
    let contents = fs::read_to_string(TRACE_DIR.to_owned() + &filename).expect(&("File Read Error: ".to_owned() + &filename)); //Reads the file into a String

    let mut output = Vec::with_capacity(y); //Allocates the output row array
    let mut rows = 0;   //Counts the number of rows
    for token in contents.split('\n')   //Iterates over each line of the file
    {
        let mut inner = Vec::with_capacity(x);  //Allocates the column array
        let mut cols = 0; //Counts the number of columns
        for number in token.split(' ')    //Iterates over each number in the file
        {
            inner.push(number.parse::<u64>().unwrap_or_else(|_| panic!("Invalid Trace Element: {}", number)));  //Pushes casted number into the inner array
            cols = cols + 1;
        }

        if cols != x    //Checks to see if there were the correct number of columns
        {
            panic!("Trace is Not Rectangular or is of Wrong Dimension: {} != {}", cols, x);
        }

        output.push(inner); //Pushes the inner array into the outer array
        rows = rows + 1;
    }

    if rows != y    //Checks to see if there were the correct number of rows
    {
        panic!("Trace is Not Rectangular or is of Wrong Dimension: {} != {}", rows, y);
    }

    output
}

fn get_single_frequencies(trace: Vec<Vec<u64>>, window_sizes: Vec<usize>) -> Vec<HashMap<(u64, usize), usize>>
{
    let mut single_frequencies: Vec<HashMap<(u64, usize), usize>> = Vec::with_capacity(window_sizes.len());

    for size in window_sizes
    {
        let mut last_seen_row: HashMap<(u64, usize), usize> = HashMap::new();
        let mut last_seen_col: HashMap<(u64, usize), usize> = HashMap::new();
        let mut last_hit: HashMap<u64, usize> = HashMap::new();
        let mut frequencies: HashMap<(u64, usize), usize> = HashMap::new();

        let x = trace.get(0).unwrap().len();
        let y = trace.len();
        for j in 0 .. x
        {
            for i in 0 .. y
            {
                let current = *trace.get(i).unwrap().get(j).unwrap();
                let mut rt = j + 1;
                let tuple = (current, i + 1);
                if last_seen_row.contains_key(&tuple)
                {
                    rt = rt - last_seen_row.get(&tuple).unwrap();
                }

                if rt > size
                {
                    if frequencies.contains_key(&tuple)
                    {
                        frequencies.insert(tuple, *frequencies.get(&tuple).unwrap() + rt - size);
                    }
                    else
                    {
                        frequencies.insert(tuple, rt - size);
                    }
                }
                else
                {
                    last_hit.insert(current, i + 1);
                }

                last_seen_row.insert(tuple, j + 1);
                last_seen_col.insert(tuple, i + 1);
            }
        }

        for tuple in last_seen_row.keys()
        {
            let rt = x + 1 - last_seen_row.get(tuple).unwrap();
            if rt > size
            {
                if frequencies.contains_key(tuple)
                {
                    frequencies.insert(*tuple, *frequencies.get(tuple).unwrap() + rt - size);
                }
                else
                {
                    frequencies.insert(*tuple, rt - size);
                }
            }
        }

        single_frequencies.push(frequencies);
    }

    single_frequencies
}

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

fn saturate(image: &mut image::GrayImage)
{
    let mut min = 255;
    let mut max = 0;
    for i in 0 .. image.width()
    {
        for j in 0 .. image.height()
        {
            let pixel = image.get_pixel(i, j) [0];
            if pixel > max
            {
                max = pixel;
            }
            if pixel < min
            {
                min = pixel;
            }
        }
    }

    let scale: f64 = (max - min) as f64 / 256.0;
    for i in 0 .. image.width()
    {
        for j in 0 .. image.height()
        {
            let pixel = image.get_pixel(i, j) [0];
            (*image).get_pixel_mut(i, j) [0] = ((pixel - min) as f64 * scale) as u8;
        }
    }
}

fn analyze_affinity(img: &image::GrayImage, entry: &str, output_dir: &str)
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

fn analyze_max_diff(img: &image::GrayImage, entry: &str, output_dir: &str)
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

fn analyze_center_diff(img: &image::GrayImage, entry: &str, output_dir: &str)
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

fn analyze_average(original: &image::GrayImage, analyzed: &image::GrayImage, entry: &str, output_dir: &str)
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

fn main() -> std::io::Result<()>
{
    for dir in &DIRS
    {
        let d = "".to_owned() + dir + "/";
        match fs::create_dir(&d)
        {
            Ok(()) => println!("Made directory {}", d),
            Err(_) => println!("Directory {} already exists", d),
        }
        let d = "saturated_".to_owned() + dir + "/";
        match fs::create_dir(&d)
        {
            Ok(()) => println!("Made directory {}", d),
            Err(_) => println!("Directory {} already exists", d),
        }
        let d = "".to_owned() + dir + "_div16/";
        match fs::create_dir(&d)
        {
            Ok(()) => println!("Made directory {}", d),
            Err(_) => println!("Directory {} already exists", d),
        }
        let d = "saturated_".to_owned() + dir + "_div16/";
        match fs::create_dir(&d)
        {
            Ok(()) => println!("Made directory {}", d),
            Err(_) => println!("Directory {} already exists", d),
        }
    }
    println!("");

    for entry in fs::read_dir(MAMMOGRAM_DIR.to_owned() + &"/")?
    {
        let entry = entry?;
        let mut original = image::open(entry.path()).unwrap().to_luma();
        let name = entry.file_name().into_string().unwrap();
        original.save(BASE_DIR.to_owned() + &"/" + &name).unwrap();
        println!("Name: {} | Dimensions: {:?}", name, original.dimensions());
        analyze_affinity(&original, &name, &(OUTPUT_DIR.to_owned() + "/"));
        analyze_max_diff(&original, &name, &(OUTPUT_MAX_DIFF_DIR.to_owned() + "/"));
        analyze_center_diff(&original, &name, &(OUTPUT_CENTER_DIFF_DIR.to_owned() + "/"));
        let analyzed = image::open("saturated_".to_owned() + OUTPUT_DIR + &"/" + &name).unwrap().to_luma();
        saturate(&mut original);
        original.save(SATURATED_DIR.to_owned() + &"/" + &name).unwrap();
        analyze_average(&original, &analyzed, &name, &(OUTPUT_AVERAGE_DIR.to_owned() + "/"));
        println!("");
    }

    Ok(())
}

#[test]
fn test_5_random_file_inputs() //Tests file input on 5 randomly generated trace files
{
    for _ in 0 .. 5 //Loops through test five times
    {
        let mut rng = thread_rng(); //Randomly generates dimensions of the trace
        let x = rng.gen_range(50, 100);
        let y = rng.gen_range(50, 100);

        let test_name = "temp_test";    //Stores the name of the test file
        let mut file = File::create(TRACE_DIR.to_owned() + test_name).unwrap();   //Creates the test trace file

        let mut expect = Vec::with_capacity(y); //Stores the expected reulting 2D array
        for i in 0 .. y //Iterates over the columns
        {
            let mut inner = Vec::with_capacity(x);  //Stores the rows of the expected resulting 2D array
            for j in 0 .. x //Iterates over the rows
            {
                let num = rng.gen_range(0, 100);    //Generates a random number to be inserted
                if j != x - 1   //Prevents extra space from being written to line
                {
                    file.write_all((num.to_string() + " ").as_bytes()).expect("File I/O Error");
                }
                else
                {
                    file.write_all((num.to_string()).as_bytes()).expect("File I/O Error");
                }
                inner.push(num);
            }
            if i != y - 1   //Prevents an extra new line at the end of the file
            {
                file.write_all("\n".as_bytes()).expect("File I/O Error");
            }
            expect.push(inner);
        }

        let actual = get_trace_file(String::from(test_name), x, y);   //Reads in the randomly generated file
        for i in 0 .. expect.len()  //Compares the two 2D arrays to ensure they are identical
        {
            for j in 0 .. expect.get(i).unwrap().len()
            {
                assert_eq!(expect.get(i).unwrap().get(j).unwrap(), actual.get(i).unwrap().get(j).unwrap());
            }
        }

        fs::remove_file(TRACE_DIR.to_owned() + &"/" + test_name).expect("File I/O Error"); //Deletes the test file
    }
}
