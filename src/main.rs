//Import helper functions from other modules
mod affinity;
use affinity::analyze_affinity;
mod average;
use average::analyze_average;
mod center_diff;
use center_diff::analyze_center_diff;
mod div16;
use div16::div16;
mod max_diff;
use max_diff::analyze_max_diff;
mod saturate;
use saturate::saturate;

extern crate image;                 // Used for image processing
use std::fs;                        // Used for file I/O and directory creation
use std::collections::HashMap;      // Used for storing examples in the answers file
use std::collections::HashSet;      // Used for storing categories from the answers file
use argparse::{ArgumentParser, Store, StoreTrue};   // Used for argument parsing
use csv::ReaderBuilder;             // Used to read answers CSV file


const IMAGE_DIR: &str = "images";               // Stores the default image directory globally
const BASE_DIR: &str = "base";                  // Stores the base output directory globally
const OUTPUT_DIR: &str = "output";              // Stores affinity output directory globally
const OUTPUT_MAX_DIFF_DIR: &str = "output_max_diff";           // Stores max diff output directory globally
const OUTPUT_CENTER_DIFF_DIR: &str = "output_center_diff";     // Stores center diff output directory globally
const OUTPUT_AVERAGE_DIR: &str = "output_average";             // Stores average output directory globally

// Stores a list of output directories for directory creation
const DIRS: [&str; 5] = [BASE_DIR, OUTPUT_DIR, OUTPUT_MAX_DIFF_DIR, OUTPUT_CENTER_DIFF_DIR, OUTPUT_AVERAGE_DIR];

fn create_dir(dir: &str, categories: &HashSet<String>)
{
    match fs::create_dir(dir)
    {
        Ok(()) => println!("Made directory {}", dir),
        Err(_) => println!("Directory {} already exists", dir),
    }
    
    // Create subdirectories for each category
    if !categories.is_empty()
    {
        categories.iter().for_each(
            | category |
            {
                let sub = dir.to_owned() + category + &"/";
                match fs::create_dir(&sub)
                {
                    Ok(()) => println!("Made directory {}", sub),
                    Err(_) => println!("Directory {} already exists", sub),
                }
            }
        )
    }
}

fn main()
{
    // Read arguments from user
    let mut image_dir = IMAGE_DIR.to_owned() + &"/";
    let mut verbose = false;
    let mut answers = "".to_owned();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pre-process images to demonstrate affinity analysis's usefulness in machine learning");
        ap.refer(&mut image_dir)
            .add_option(&["-i", "--images"], Store,
            "Set the directory of input images (set to images/ in executable directory by default)");
        ap.refer(&mut answers)
            .add_option(&["-a", "--answers"], Store,
            "Set the path of a CSV file with answers to classify the provided images");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Print verbose logging messages");
        ap.parse_args_or_exit();
    }

    // Process provided examples, if available
    let mut examples: HashMap<String, String> = HashMap::new();
    let mut categories: HashSet<String> = HashSet::new();
    if answers.len() > 0
    {
        let mut reader = ReaderBuilder::new().flexible(true).from_path(answers).unwrap();

        // Reads the category names from the header
        let header = reader.headers().unwrap();
        let num = header [1].parse::<usize>().unwrap();
        assert_eq!(num + 2, header.len());
        for i in 2 .. 2 + num
        {
            categories.insert(header [i].to_owned());
        }

        // Insert the example answers into a HashMap for sorting later
        reader.records().for_each(
            | record |
            {
                let record = record.unwrap();
                assert_eq!(record.len(), 2);
                if !categories.contains(&record [1]) { panic!("Category {} not defined in provided answers file", &record [1]); }
                examples.insert(record [0].to_owned(), record [1].to_owned());
            }
        );
    }

    // Create directories to store images
    for dir in &DIRS
    {
        let d = "".to_owned() + dir + "/";
        create_dir(&d, &categories);
        let d = "saturated_".to_owned() + dir + "/";
        create_dir(&d, &categories);
        let d = "".to_owned() + dir + "_div16/";
        create_dir(&d, &categories);
        let d = "saturated_".to_owned() + dir + "_div16/";
        create_dir(&d, &categories);
    }
    println!("");

    for entry in fs::read_dir(image_dir).unwrap()
    {
        let entry = entry.unwrap();
        let mut original = image::open(entry.path()).unwrap().to_luma();
        let name = entry.file_name().into_string().unwrap();
        original.save(BASE_DIR.to_owned() + &"/" + &name).unwrap();
        println!("Name: {} | Dimensions: {:?}", name, original.dimensions());
        analyze_affinity(&original, &name, &(OUTPUT_DIR.to_owned() + "/"));
        analyze_max_diff(&original, &name, &(OUTPUT_MAX_DIFF_DIR.to_owned() + "/"));
        analyze_center_diff(&original, &name, &(OUTPUT_CENTER_DIFF_DIR.to_owned() + "/"));
        let analyzed = image::open("saturated_".to_owned() + OUTPUT_DIR + &"/" + &name).unwrap().to_luma();
        saturate(&mut original);
        original.save("saturated_".to_owned() + BASE_DIR + &"/" + &name).unwrap();
        analyze_average(&original, &analyzed, &name, &(OUTPUT_AVERAGE_DIR.to_owned() + "/"));
        
        println!("\tDividing by 16:");
        let mut original = image::open(entry.path()).unwrap().to_luma();
        div16(&mut original);
        original.save(BASE_DIR.to_owned() + &"_div16/" + &name).unwrap();
        analyze_affinity(&original, &name, &(OUTPUT_DIR.to_owned() + "_div16/"));
        analyze_max_diff(&original, &name, &(OUTPUT_MAX_DIFF_DIR.to_owned() + "_div16/"));
        analyze_center_diff(&original, &name, &(OUTPUT_CENTER_DIFF_DIR.to_owned() + "_div16/"));
        let analyzed = image::open("saturated_".to_owned() + OUTPUT_DIR + &"_div16/" + &name).unwrap().to_luma();
        saturate(&mut original);
        original.save("saturated_".to_owned() + BASE_DIR + &"_div16/" + &name).unwrap();
        analyze_average(&original, &analyzed, &name, &(OUTPUT_AVERAGE_DIR.to_owned() + "_div16/"));
        println!("");
    }
}
