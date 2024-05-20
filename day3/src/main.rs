use aoc_helpers::read_list_of_any;
use std::env;

fn split_string_n(s: &str, n: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let chunk_size = (chars.len() as f64 / n as f64).ceil() as usize;
    let mut result = Vec::new();
    for i in 0..n {
        let start = i * chunk_size;
        let end = ((i + 1) * chunk_size).min(chars.len());
        let chunk: String = chars[start..end].iter().collect();
        result.push(chunk);
    }
    result
}

fn char_to_number(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some((c as u32) - ('a' as u32) + 1),
        'A'..='Z' => Some((c as u32) - ('A' as u32) + 27),
        _ => None,
    }
}

// Part 1
fn find_common_chars_within_string_chunks(inner_vector: Vec<String>) -> Vec<Option<char>> {
    let mut list_of_common_chars: Vec<Option<char>> = Vec::new();
    for i in inner_vector {
        let t = split_string_n(&i, 2);
        let (first, second): (String, String);
        if t[0].len() > t[1].len() {
            (first, second) = (t[0].clone(), t[1].clone());
        } else {
            (first, second) = (t[1].clone(), t[0].clone());
        }
        let mut temp_storage_for_chars: Vec<char> = Vec::new();
        for character in first.chars() {
            if second.find(character).is_some() {
                if !temp_storage_for_chars.contains(&character) {
                    temp_storage_for_chars.push(character);
                }
            }
        }
        if !temp_storage_for_chars.is_empty() {
            list_of_common_chars.push(Some(temp_storage_for_chars[0]));
        }
    }
    list_of_common_chars
}

// Part 1
fn find_common_chars_between_strings(inner_vector: Vec<String>) -> Vec<char> {
    let mut common_chars: Vec<char> = Vec::new();
    for i in inner_vector.chunks(3) {
        let mut strings = vec![&i[0], &i[1], &i[2]];
        strings.sort_by(|a, b| b.len().cmp(&a.len()));
        let (first, second, third) = (&strings[0], &strings[1], &strings[2]);
        let mut temp_storage_for_chars: Vec<char> = Vec::new();
        for character in first.chars() {
            if second.find(character).is_some() && third.find(character).is_some() {
                if !temp_storage_for_chars.contains(&character) {
                    temp_storage_for_chars.push(character);
                }
            }
        }
        if !temp_storage_for_chars.is_empty() {
            common_chars.push(temp_storage_for_chars[0]);
        }
    }
    common_chars
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let output: Result<Vec<Vec<String>>, _> = read_list_of_any(file_path);
    let value = output?;
    let inner_vector = &value[0];

    // println!("{:?}", inner_vector);

    let list_of_common_chars = find_common_chars_within_string_chunks(inner_vector.clone());
    // println!("{:?}", list_of_common_chars);

    let mut sum_of_digits: u32 = 0;
    for i in list_of_common_chars.clone() {
        let c = match i {
            None => continue,
            _ => i.unwrap(),
        };
        let d = char_to_number(c).unwrap();
        sum_of_digits += d;
    }
    println!("Part 1: {}", sum_of_digits);

    let list_of_common_chars_2 = find_common_chars_between_strings(inner_vector.clone());
    // println!("{:?}", list_of_common_chars_2);

    let mut sum_of_digits_2: u32 = 0;
    for i in list_of_common_chars_2.clone() {
        let d = char_to_number(i).unwrap();
        sum_of_digits_2 += d;
    }
    println!("Part 2: {}", sum_of_digits_2);

    Ok(())
}
