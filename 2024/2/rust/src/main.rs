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

fn get_safe_reports_for_first_puzzle(lines: &mut Lines<BufReader<File>>) -> i32 {
    let mut count_safe = 0;

    for line in lines.flatten() {
        println!("{}", line);

        let levels = line.split(' ').map(|l| l.parse::<i32>().unwrap());

        let mut increase = false;
        let mut decrease = false;
        let mut first_run = true;
        let safe = levels.is_sorted_by(|a, b| {
            let increased = increase;
            let decreased = decrease;
            let mut result = false;

            if a < b && b - a < 4 {
                increase = true;
                decrease = false;
                result = if first_run {
                    increase
                } else {
                    increase == increased
                };
            } else if b < a && a - b < 4 {
                increase = false;
                decrease = true;
                result = if first_run {
                    decrease
                } else {
                    decrease == decreased
                };
            } else {
                result = false;
            }

            first_run = false;
            return result;
        });

        count_safe += if safe { 1 } else { 0 };
        println!("Report {} is safe: {}", line, safe);
    }

    count_safe
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <path/to/file>", args[0]);
        return;
    }

    if let Ok(mut lines) = read_lines(args[1].to_string()) {
        let count_safe = get_safe_reports_for_first_puzzle(&mut lines);
        println!("Puzzle One: {} reports are safe!", count_safe);
    }
}
