extern crate rand;                  //Used for testing random cases
use rand::{thread_rng, Rng};        //Used for random number generation
use std::fs;                        //Used for file I/O
use std::fs::File;                  //Used for writing to files
use std::io::Write;                 //Used for writing to files
use std::env;                       //Used for command line arguments
use std::collections::HashMap;      //Imports HashMap data structure

const trace_dir: &str = "traces/";    //Stores trace directory globally

fn get_trace_file(filename: String, x: usize, y: usize) -> Vec<Vec<u64>>    //Converts a file of numbers seperated by spaces and new lines into a 2D array of those numbers
{
    let contents = fs::read_to_string(trace_dir.to_owned() + &filename).expect(&("File Read Error: ".to_owned() + &filename)); //Reads the file into a String

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

fn main()
{
    println!("{:?}", env::args());
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
        let mut file = File::create(trace_dir.to_owned() + test_name).unwrap();   //Creates the test trace file
        
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

        fs::remove_file(trace_dir.to_owned() + test_name).expect("File I/O Error"); //Deletes the test file
    }
}