use std::fs;                    //Used for file I/O
use std::env;                   //Used for command line arguments
//use std::collections::HashMap;  //Imports HashMap data structure

fn parse_file(filename: String, x: usize, y: usize) -> Vec<Vec<u64>>    //Converts a file of numbers seperated by spaces and new lines into a 2D array of those numbers
{
    let contents = fs::read_to_string(&filename).expect(&("File Read Error: ".to_owned() + &filename)); //Reads the file into a String

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
fn test_file_input()
{
    let expect = vec!(vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3));
    let actual = parse_file(String::from("traces/simple_test_trace"), 4, 4);
    for i in 0 .. expect.len()
    {
        for j in 0 .. expect.get(i).unwrap().len()
        {
            assert_eq!(expect.get(i).unwrap().get(j).unwrap(), actual.get(i).unwrap().get(j).unwrap());
        }
    }
}