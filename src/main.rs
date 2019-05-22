use std::collections::HashMap;

fn get_reuse_times(trace: &Vec<Vec<u64>>) -> HashMap<u64, u64>
{
    let reuse_times = HashMap::new();
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

fn main()
{
    let v = vec!(vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3), vec!(0, 1, 2, 3));
    get_reuse_times(&v);
}

#[test]
fn test_reuse_times()
{
    assert_eq!(0, 0);
}