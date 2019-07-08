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
use image::ImageBuffer;
use image::Rgb;

//Imports for tests
extern crate rand;                  //Used for testing random cases
use rand::{thread_rng, Rng};        //Used for random number generation
use std::io::Write;                 //Used for writing to files

const TRACE_DIR: &str = "traces/";            //Stores trace directory globally
const MAMMOGRAM_DIR: &str = "mammograms/";    //Stores trace directory globally
const OUTPUT_DIR: &str = "output/";           //Stores output directory globally

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

fn main() -> std::io::Result<()>
{
    println!("{:?}", env::args());
    let arr = vec!(vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0));
    let sizes = vec!(2, 3);

    let naive = get_frequencies_naive(arr, sizes.clone());
    let mut i = 0;
    for hash in naive.0
    {
        println!("Window Size {}", sizes[i]);
        i = i + 1;

        for key in hash.keys()
        {
            println!("{:?}: {}", key, hash.get(key).unwrap());
        }
    }

    i = 0;
    for hash in naive.1
    {
        println!("Window Size {}", sizes[i]);
        i = i + 1;

        for key in hash.keys()
        {
            println!("{:?}: {}", key, hash.get(key).unwrap());
        }
    }

    for entry in fs::read_dir(MAMMOGRAM_DIR)?
    {
        let entry = entry?;
        let img = image::open(entry.path()).unwrap().to_luma();
        println!("Dimensions: {:?}", img.dimensions());
        let mut image: image::GrayImage = image::ImageBuffer::new(img.width() - 2, img.height() - 2);
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

                    image.get_pixel_mut(i, j).data = [max_diff];
                }
                else
                {
                    image.get_pixel_mut(i, j).data = [0];
                }
            }
        }
        image.save(OUTPUT_DIR.to_owned() + &entry.file_name().into_string().unwrap()).unwrap();
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

        fs::remove_file(TRACE_DIR.to_owned() + test_name).expect("File I/O Error"); //Deletes the test file
    }
}

#[test]
fn test_single_frequencies()
{

}