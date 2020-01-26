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
extern crate rand;                  // Used for randomly splitting data
use rand::Rng;                      // Used for randomly splitting data
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

fn create_dir(dir: &str, del: bool, val: bool, categories: &HashSet<String>)
{
    if del
    {
        match fs::remove_dir_all(dir)
        {
            Ok(()) => println!("Deleted directory {}", dir),
            Err(_) => println!("Failed to delete directory {} ", dir),
        }
    }
    match fs::create_dir(dir)
    {
        Ok(()) => println!("Made directory {}", dir),
        Err(_) => println!("Directory {} already exists", dir),
    }

    // Create validation and test folders
    if val
    {
        let sub = dir.to_owned() + &"validation/";
        match fs::create_dir(&sub)
        {
            Ok(()) => println!("Made subdirectory {}", sub),
            Err(_) => println!("Subdirectory {} already exists", sub),
        }
        let sub = dir.to_owned() + &"training/";
        match fs::create_dir(&sub)
        {
            Ok(()) => println!("Made subdirectory {}", sub),
            Err(_) => println!("Subdirectory {} already exists", sub),
        }
    }
    
    // Create subdirectories for each category
    if !categories.is_empty()
    {
        categories.iter().for_each(
            | category |
            {
                if val
                {
                    let sub = dir.to_owned() + &"validation/" + category + &"/";
                    match fs::create_dir(&sub)
                    {
                        Ok(()) => println!("Made subdirectory {}", sub),
                        Err(_) => println!("Subdirectory {} already exists", sub),
                    }
                    let sub = dir.to_owned() + &"training/" + category + &"/";
                    match fs::create_dir(&sub)
                    {
                        Ok(()) => println!("Made subdirectory {}", sub),
                        Err(_) => println!("Subdirectory {} already exists", sub),
                    }
                }
                else
                {
                    let sub = dir.to_owned() + category + &"/";
                    match fs::create_dir(&sub)
                    {
                        Ok(()) => println!("Made subdirectory {}", sub),
                        Err(_) => println!("Subdirectory {} already exists", sub),
                    }
                }
            }
        )
    }
}

fn output_dir(dir: &str, sample: &str, examples: &HashMap<String, String>) -> String
{
    match examples.get(sample)
    {
        Some(subdir) => dir.to_owned() + "/" + subdir + "/",
        None => dir.to_owned() + "/",
    }
}

fn main()
{
    // Read arguments from user
    let mut image_dir = IMAGE_DIR.to_owned() + &"/";
    let mut answers = "".to_owned();
    let mut validation = 0;
    let mut delete = false;
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pre-process images to demonstrate affinity analysis's usefulness in machine learning");
        ap.refer(&mut image_dir)
            .add_option(&["-i", "--images"], Store,
            "Set the directory of input images (set to images/ in executable directory by default)");
        ap.refer(&mut answers)
            .add_option(&["-a", "--answers"], Store,
            "Set the path of a CSV file with answers to classify the provided images");
        ap.refer(&mut validation)
            .add_option(&["-t", "--test"], Store,
            "Set the number of images to be split off into a validation set for training (ignores negative and 0 values and requires answers file to be set)");
        ap.refer(&mut delete)
            .add_option(&["-d", "--delete"], StoreTrue,
            "Delete the existing directories of processed images");
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
        let num_categories = header [1].parse::<usize>().expect("Invalid number of categories in CSV");
        let num_images = header [0].parse::<usize>().expect("Invalid number of images in CSV");
        assert_eq!(num_categories + 2, header.len());
        for i in 2 .. 2 + num_categories
        {
            categories.insert(header [i].to_owned());
        }

        // Handles creating the training and validation sets
        let mut validation_index: HashSet<usize> = HashSet::new();
        let mut rng = rand::thread_rng();
        for _ in 0 .. validation
        {
            // Loop until we find a new value
            let mut sample = rng.gen_range(0, num_images);
            while validation_index.contains(&sample)
            {
                sample = rng.gen_range(0, num_images);
            }
            validation_index.insert(sample);
        }

        // Insert the example answers into a HashMap for sorting later
        reader.records().enumerate().for_each(
            | (index, record) |
            {
                let record = record.unwrap();
                assert_eq!(record.len(), 2);
                if !categories.contains(&record [1]) { panic!("Category {} not defined in provided answers file", &record [1]); }

                // Handle appending validation/training set path to answer path
                let mut set = "".to_owned();
                if validation > 0
                {
                    if validation_index.contains(&index)
                    {
                        set = "validation/".to_owned();
                    }
                    else
                    {
                        set = "training/".to_owned();
                    }
                }

                examples.insert(record [0].to_owned(), set + &record [1]);
            }
        );
        assert_eq!(num_images, examples.len());
    }
    else if validation > 0
    {
        panic!("Validation size set without any provided categorization");
    }

    // Create directories to store images
    for dir in &DIRS
    {
        let val = validation > 0;
        let d = "".to_owned() + dir + "/";
        create_dir(&d, delete, val, &categories);
        let d = "saturated_".to_owned() + dir + "/";
        create_dir(&d, delete, val, &categories);
        let d = "".to_owned() + dir + "_div16/";
        create_dir(&d, delete, val, &categories);
        let d = "saturated_".to_owned() + dir + "_div16/";
        create_dir(&d, delete, val, &categories);
    }
    println!("");

    for entry in fs::read_dir(image_dir).expect("Image directory not found")
    {
        let entry = entry.unwrap();
        let name_in = entry.file_name().into_string().unwrap();
        let bmp = entry.path().with_extension("bmp");
        let name_out = bmp.file_name().unwrap().to_str().unwrap();
        if !examples.is_empty() { assert!(examples.contains_key(&name_in)); }

        let mut original = image::open(entry.path()).unwrap().to_rgb();
        original.save(output_dir(&BASE_DIR, &name_in, &examples) + name_out).unwrap();
        println!("Name: {} | Dimensions: {:?}", name_in, original.dimensions());
        analyze_affinity(&original, name_out, &output_dir(&OUTPUT_DIR, &name_in, &examples));
        analyze_max_diff(&original, name_out, &output_dir(&OUTPUT_MAX_DIFF_DIR, &name_in, &examples));
        analyze_center_diff(&original, name_out, &output_dir(&OUTPUT_CENTER_DIFF_DIR, &name_in, &examples));
        let analyzed = image::open("saturated_".to_owned() + &output_dir(&OUTPUT_DIR, &name_in, &examples) + name_out).unwrap().to_rgb();
        saturate(&mut original);
        original.save("saturated_".to_owned() + &output_dir(&BASE_DIR, &name_in, &examples) + name_out).unwrap();
        analyze_average(&original, &analyzed, name_out, &output_dir(&OUTPUT_AVERAGE_DIR, &name_in, &examples));
        
        println!("\tDividing by 16:");
        let mut original = image::open(entry.path()).unwrap().to_rgb();
        div16(&mut original);
        original.save(output_dir(&(BASE_DIR.to_owned() + "_div16/"), &name_in, &examples) + name_out).unwrap();
        analyze_affinity(&original, name_out, &output_dir(&(OUTPUT_DIR.to_owned() + "_div16/"), &name_in, &examples));
        analyze_max_diff(&original, name_out, &output_dir(&(OUTPUT_MAX_DIFF_DIR.to_owned() + "_div16/"), &name_in, &examples));
        analyze_center_diff(&original, name_out, &output_dir(&(OUTPUT_CENTER_DIFF_DIR.to_owned() + "_div16/"), &name_in, &examples));
        let analyzed = image::open("saturated_".to_owned() + &output_dir(&(OUTPUT_DIR.to_owned() + "_div16/"), &name_in, &examples) + name_out).unwrap().to_rgb();
        saturate(&mut original);
        original.save("saturated_".to_owned() + &output_dir(&(BASE_DIR.to_owned() + "_div16/"), &name_in, &examples) + name_out).unwrap();
        analyze_average(&original, &analyzed, name_out, &output_dir(&(OUTPUT_AVERAGE_DIR.to_owned() + "_div16/"), &name_in, &examples));
        println!("");
    }
}
