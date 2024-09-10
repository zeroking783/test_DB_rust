use csv::Writer;
use dotenv::from_path;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::collections::HashSet;
use std::{env, fs};
use std::fmt::format;
use std::fs::File;
use std::path::Path;
use std::io::Error;



fn main() {

    // Read .env file
    let path_env = Path::new("./.env");
    from_path(path_env).expect("Failed to open .env file");
    let count_data = env::var("COUNT_DATA").expect("Fail read COUNT_DATA in .env");
    let count_file = env::var("COUNT_FILE").expect("Fail read COUNT_FILE in .env");
    let main_file_name = env::var("MAIN_FILE_NAME").expect("Fail read MAIN_FILE_NAME in .env");

    let count_data = count_data.parse::<u32>().expect("COUNT_DATA is mast be u32");
    let count_file = count_file.parse::<u32>().expect("COUNT_FILE is mast be u32");

    let file_volume_path = String::from("var/lib/DB_test/");

    fs::create_dir(file_volume_path).expect("Failed to create root directory");

    match delete_extra_csv(&file_volume_path) {
        Ok(()) => println!("All used .csv files are deleted"),
        Err(e) => eprintln!("Error: {e}"),
    }

    match check_divisibility_data_file(&count_data, &count_file) {
        Ok(()) => println!("COUNT_DATA and COUNT_FILE are successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Processing create general.csv file
    match create_file_csv(&file_volume_path, count_file, &main_file_name) {
        Ok(()) => println!("File {} created successfully", main_file_name),
        Err(e) => eprintln!("Error: {}", e),
    }

    // I use HashSet because it checks an element for occurrence O(1)
    let mut all_word = HashSet::new();
    let mut all_num = HashSet::new();

    // I use Vec because it save order elements
    let mut words = Vec::new();
    let mut nums = Vec::new();

    // Create HashSet and Vec with random word and random num
    for i in 0..count_data {
        loop {
            let (word, num) = create_data();

            if !all_word.contains(&word) && !all_num.contains(&num) {
                all_word.insert(word.clone());
                all_num.insert(num);
                words.push(word);
                nums.push(num);
                break;
            }
        }
    }

    // Write general .csv
    write_to_csv(&file_volume_path, &main_file_name, words, nums, count_file).unwrap();
}

// Create all .csv files
fn create_file_csv(file_volume_path: &String, count_file: u32, main_file_name: &String) -> Result<(), Error> {
    let mut file = File::create(format!("{}/{}", file_volume_path, main_file_name))?;
    for i in 1..=count_file {
        let mut file = File::create(format!("{}/{}.csv", file_volume_path, i.to_string()))?;
    }
    Ok(())
}

// Checking if all .csv files can be created
fn check_divisibility_data_file(count_data: &u32, count_file: &u32) -> Result<(), &'static str> {
    if count_data % count_file == 0 {
        Ok(())
    } else {
        Err("Count data must divisibility on count_file")
    }
}

fn delete_extra_csv(file_volume_path: &String) -> Result<(), std::io::Error>{
    for entry in fs::read_dir(format!("{}/", file_volume_path))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "csv") {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

fn write_to_csv(file_volume_path: &String, main_file_name: &String, all_word: Vec<String>, all_num: Vec<i32>, count_file: u32) -> Result<(), csv::Error> {
    let mut wtr_general = Writer::from_path(format!("{}/{}", &file_volume_path, &main_file_name))?;

    let total_files = count_file as usize;
    let total_items = all_word.len();
    let items_per_file = (total_items + total_files - 1) / total_files;

    let mut file_num: usize = 1;
    let mut wtr_peace = Writer::from_path(format!("{}/{}.csv", file_volume_path, file_num))?;

    for i in 0..total_items {
        if i / items_per_file + 1 == file_num {
            wtr_peace.write_record(&[&all_word[i], &all_num[i].to_string()])?;
        } else {
            wtr_peace.flush()?;
            file_num += 1;
            wtr_peace = Writer::from_path(format!("{}/{}.csv", file_volume_path, file_num))?;
            wtr_peace.write_record(&[&all_word[i], &all_num[i].to_string()])?;
        }
        if i == total_items - 1 {
            wtr_peace.flush()?;
        }

        wtr_general.write_record(&[&all_word[i], &all_num[i].to_string()])?;
    }

    wtr_general.flush()?;
    Ok(())
}

fn create_data() -> (String, i32) {
    let word = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let mut rng = thread_rng();
    let number = rng.gen();

    (word, number)
}