use aoc_helpers::read_list_of_any;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let output: Result<Vec<Vec<String>>, _> = read_list_of_any(file_path);
    let value = output?;
    let inner_vector = &value[0];
    // println!("{:?}", inner_vector);

    // Calculation for Part 1
    let mut score = 0;
    for i in inner_vector {
        let i: Vec<char> = i.chars().collect();
        // Rock vs Rock (Draw)
        if i[0] == 'A' && i[2] == 'X' {
            score += 3 + 1;
        }
        // Rock vs Paper (Win)
        else if i[0] == 'A' && i[2] == 'Y' {
            score += 6 + 2;
        }
        // Rock vs Scissors (Lose)
        else if i[0] == 'A' && i[2] == 'Z' {
            score += 0 + 3;
        }
        // Paper vs Rock (Lose)
        else if i[0] == 'B' && i[2] == 'X' {
            score += 0 + 1;
        }
        // Paper vs Paper (Draw)
        else if i[0] == 'B' && i[2] == 'Y' {
            score += 3 + 2;
        }
        // Paper vs Scissors (Win)
        else if i[0] == 'B' && i[2] == 'Z' {
            score += 6 + 3;
        }
        // Scissors vs Rock (Win)
        else if i[0] == 'C' && i[2] == 'X' {
            score += 6 + 1;
        }
        // Scissors vs Paper (Lose)
        else if i[0] == 'C' && i[2] == 'Y' {
            score += 0 + 2;
        }
        // Scissors vs Scissors (Draw)
        else if i[0] == 'C' && i[2] == 'Z' {
            score += 3 + 3;
        }
        // println!("{:?}", i);
    }
    println!("Part 1 Score: {}", score);

    // Calculation for Part 2
    let mut score = 0;
    for i in inner_vector {
        let i: Vec<char> = i.chars().collect();
        // (Lose) Rock vs Scissors
        if i[0] == 'A' && i[2] == 'X' {
            score += 0 + 3;
        }
        // (Draw) Rock vs Rock
        else if i[0] == 'A' && i[2] == 'Y' {
            score += 3 + 1;
        }
        // (Win) Rock vs Paper
        else if i[0] == 'A' && i[2] == 'Z' {
            score += 6 + 2;
        }
        // (Lose) Paper vs Rock
        else if i[0] == 'B' && i[2] == 'X' {
            score += 0 + 1;
        }
        // (Draw) Paper vs Paper
        else if i[0] == 'B' && i[2] == 'Y' {
            score += 3 + 2;
        }
        // (Win) Paper vs Scissors
        else if i[0] == 'B' && i[2] == 'Z' {
            score += 6 + 3;
        }
        // (Lose) Scissors vs Paper
        else if i[0] == 'C' && i[2] == 'X' {
            score += 0 + 2;
        }
        // (Draw) Scissors vs Scissors
        else if i[0] == 'C' && i[2] == 'Y' {
            score += 3 + 3;
        }
        // (Win) Scissors vs Rock
        else if i[0] == 'C' && i[2] == 'Z' {
            score += 6 + 1;
        }
        // println!("{:?}", i);
    }
    println!("Part 2 Score: {}", score);

    Ok(())
}
