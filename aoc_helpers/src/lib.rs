use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn read_list_of_numbers(file_path: &str) -> Vec<Vec<i32>> {
    let reader = BufReader::new(File::open(file_path).expect("Unable to open file"));

    let mut lists = Vec::new();
    let mut current_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.is_empty() {
            if !current_list.is_empty() {
                lists.push(current_list);
                current_list = Vec::new();
            }
        } else {
            let integer = line.parse::<i32>().expect("Invalid integer");
            current_list.push(integer);
        }
    }

    if !current_list.is_empty() {
        lists.push(current_list);
    }

    lists
}

pub fn read_list_of_any<T>(file_path: &str) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + 'static,
{
    let reader = BufReader::new(File::open(file_path)?);

    let mut lists = Vec::new();
    let mut current_list = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            if !current_list.is_empty() {
                lists.push(current_list);
                current_list = Vec::new();
            }
        } else {
            let value = line.parse::<T>()?;
            current_list.push(value);
        }
    }

    if !current_list.is_empty() {
        lists.push(current_list);
    }

    Ok(lists)
}

