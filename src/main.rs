use std::fs;
use std::env;
use std::collections::HashMap;

fn get_reuse_times(trace: &Vec<Vec<u64>>) -> HashMap<u64, u64>
{
    let reuse_times: HashMap<u64, u64> = HashMap::new();
    for i in 0 .. trace.len()
    {
        for j in 0 .. trace.get(i).unwrap().len()
        {
            print!("{} ", trace.get(i).unwrap().get(j).unwrap());
        }
        println!();
    }
    reuse_times
}

fn parse_file(filename: String) -> Vec<Vec<u64>>
{
    let contents = fs::read_to_string(&filename).expect(&("File Read Error: ".to_owned() + &filename));

    let estimate = ((contents.len() as f64).sqrt() / 2.0) as usize;
    let mut output = Vec::with_capacity(estimate);
    for token in contents.split('\n')
    {
        let mut inner = Vec::with_capacity(estimate);
        for item in token.split(' ')
        {
            inner.push(item.parse::<u64>().unwrap_or_else(|_| panic!("Invalid Trace Element: {}", item)));
        }
        output.push(inner);
    }

    output
}

fn main()
{
    let v = vec!(vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3));
    get_reuse_times(&v);
}

#[test]
fn test_file_input()
{
    let expect = vec!(vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3));
    let actual = parse_file(String::from("traces/simple_test_trace"));
    for i in 0 .. expect.len()
    {
        for j in 0 .. expect.get(i).unwrap().len()
        {
            assert_eq!(expect.get(i).unwrap().get(j).unwrap(), actual.get(i).unwrap().get(j).unwrap());
        }
    }
}