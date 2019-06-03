#![allow(dead_code)]                //Removes annoying unused function warnings
#![allow(unused_imports)]           //Removes annoying unused import warnings

//Imports for code
use std::fs;                        //Used for file I/O
use std::env;                       //Used for command line arguments
use std::collections::HashMap;      //Imports HashMap data structure

//Imports for tests
extern crate rand;                  //Used for testing random cases
use rand::{thread_rng, Rng};        //Used for random number generation
use std::fs::File;                  //Used for writing to files
use std::io::Write;                 //Used for writing to files

const TRACE_DIR: &str = "traces/";    //Stores trace directory globally

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

fn main()
{
    println!("{:?}", env::args());
    let arr = vec!(vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0),
                   vec!(0, 1, 2, 3, 0));
    let sizes = vec!(2, 3);
    let test = get_single_frequencies(arr, sizes.clone());

    let mut i = 0;
    for hash in test
    {
        println!("Window Size {}", sizes[i]);
        i = i + 1;

        for key in hash.keys()
        {
            println!("{:?}: {}", key, hash.get(key).unwrap());
        }
    }
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