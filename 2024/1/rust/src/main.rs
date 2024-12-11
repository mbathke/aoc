use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn find_factor(num: i32, numbers: &Vec<i32>) -> i32 {
    numbers.iter().filter(|&n| *n == num).collect::<Vec<_>>().len() as i32
}   

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <path/to/file>", args[0]);
        return;
    }

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    // first read the numbers from left and right
    if let Ok(lines) = read_lines(args[1].to_string()) {
        for line in lines.flatten() {
            println!("{}", line);
            let split: Vec<&str> = line.split("   ").collect();
            let left_element = split.first().unwrap().parse::<i32>().unwrap();
            let right_element = split.last().unwrap().parse::<i32>().unwrap();

            left_numbers.push(left_element);
            right_numbers.push(right_element);
        }
    }

    // sort the numbers
    left_numbers.sort();
    right_numbers.sort();
    let mut total_distance = 0;
    let mut total_sim_score = 0;

    for (pos, number) in left_numbers.iter().enumerate() {
        let distance = i32::abs(left_numbers[pos] - right_numbers[pos]);
        let factor = find_factor(*number, &right_numbers);

        let sim_score = left_numbers[pos] * factor;

        println!("Factor for {} is {} and its similarity score is {}", number, factor, sim_score);

        total_sim_score += sim_score;

        println!(
            "Distance of {} and {} is {}",
            left_numbers[pos],
            right_numbers[pos],
            distance,
        );

        total_distance += distance;
    }

    println!("The total distance is {}", total_distance);
    println!("The total similarity score is {}", total_sim_score);
}
