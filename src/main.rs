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

extern crate image;                 //Used for image processing
use std::fs;                        //Used for file I/O
use argparse::{ArgumentParser, Store};   //StoreTrue

const IMAGE_DIR: &str = "images";    //Stores trace directory globally
const BASE_DIR: &str = "base";                 //Stores the base storage directory globally
const SATURATED_DIR: &str = "saturated";      //Stores saturated output directory globally
const OUTPUT_DIR: &str = "output";           //Stores output directory globally
const OUTPUT_MAX_DIFF_DIR: &str = "output_max_diff";           //Stores output directory globally
const OUTPUT_CENTER_DIFF_DIR: &str = "output_center_diff";           //Stores output directory globally
const OUTPUT_AVERAGE_DIR: &str = "output_average";           //Stores output directory globally

const DIRS: [&str; 6] = [BASE_DIR, SATURATED_DIR, OUTPUT_DIR, OUTPUT_MAX_DIFF_DIR, OUTPUT_CENTER_DIFF_DIR, OUTPUT_AVERAGE_DIR]; //Stores a list of output directories for later use

fn main() -> std::io::Result<()>
{
    let mut image_dir = IMAGE_DIR.to_string() + &"/";
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Pre-process images to demonstrate affinity's usefulness in machine learning");
        ap.refer(&mut image_dir)
            .add_option(&["-i", "--images"], Store,
            "Set the directory of input images (set to images/ in executable directory by default)");
        ap.parse_args_or_exit();
    }

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

    for entry in fs::read_dir(image_dir)?
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
        
        println!("\tDividing by 16:");
        let mut original = image::open(entry.path()).unwrap().to_luma();
        div16(&mut original);
        original.save(BASE_DIR.to_owned() + &"_div16/" + &name).unwrap();
        analyze_affinity(&original, &name, &(OUTPUT_DIR.to_owned() + "_div16/"));
        analyze_max_diff(&original, &name, &(OUTPUT_MAX_DIFF_DIR.to_owned() + "_div16/"));
        analyze_center_diff(&original, &name, &(OUTPUT_CENTER_DIFF_DIR.to_owned() + "_div16/"));
        let analyzed = image::open("saturated_".to_owned() + OUTPUT_DIR + &"_div16/" + &name).unwrap().to_luma();
        saturate(&mut original);
        original.save(SATURATED_DIR.to_owned() + &"_div16/" + &name).unwrap();
        analyze_average(&original, &analyzed, &name, &(OUTPUT_AVERAGE_DIR.to_owned() + "_div16/"));
        println!("");
    }

    Ok(())
}
