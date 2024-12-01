use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut content: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        content.push(line);
    }
    content
}

fn day_1(content: &Vec<String>) {
    let mut left_column: Vec<usize> = Vec::new();
    let mut right_column: Vec<usize> = Vec::new();
    for line in content {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left_column.push(parts[0].parse::<usize>().unwrap());
        right_column.push(parts[1].parse::<usize>().unwrap());
    }

    left_column.sort();
    right_column.sort();

    let mut result_1 = 0;
    let mut result_2 = 0;
    for (i, element) in left_column.iter().enumerate() {
        result_1 += element.abs_diff(right_column[i]);
        let count = right_column.iter().filter(|&x| x == element).count();
        result_2 += element * count;
    }

    println!("Day 1");
    println!("First part result: {:?}", result_1);
    println!("Second part result: {:?}", result_2);
}

fn main() -> io::Result<()> {
    let content = read_file("input.txt");
    day_1(&content);
    Ok(())
}
