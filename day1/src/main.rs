use aoc_helpers::read_list_of_numbers;
use std::env;

fn calculate_sum_of_subvectors_and_return_with_index(list: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut sum = 0;
    let mut index_sum_tuple = Vec::<(i32, i32)>::new();
    for i in 0..list.len() {
        for j in 0..list[i].len() {
            sum += list[i][j];
        }
        index_sum_tuple.push((i as i32 + 1, sum));
        sum = 0;
    }
    index_sum_tuple
}

fn sort_tuples_by_sum(list: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    list.sort_by(|a, b| a.1.cmp(&b.1)); // sort by sum
    list.to_vec()
}

fn get_top_n_tuples_by_sum(list: &mut Vec<(i32, i32)>, top_n: i32) -> Vec<(i32, i32)> {
    let top_n = top_n as usize;
    let len = list.len();
    list[len - top_n..].to_vec() // get the last n elements by slicing the list from len - n to len
}

fn calculate_total_sum_from_tuples(list: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    for i in 0..list.len() {
        sum += list[i].1;
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let lists = read_list_of_numbers(&file_path);
    let index_sum_list = calculate_sum_of_subvectors_and_return_with_index(&lists);
    let sorted_list = sort_tuples_by_sum(&mut index_sum_list.clone());
    let top_n = 3;
    let top_n_tuples = get_top_n_tuples_by_sum(&mut sorted_list.clone(), top_n);
    println!("Top {}: {:?}", top_n, top_n_tuples);
    let sum_of_calories = calculate_total_sum_from_tuples(&top_n_tuples);
    println!("Sum of calories: {}", sum_of_calories);
}
