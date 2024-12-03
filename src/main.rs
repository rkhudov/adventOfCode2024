use regex::Regex;
use std::fs::{self, File};
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

fn is_safe(cur_number: usize, number: usize, flag: &str) -> bool {
    let difference = cur_number.abs_diff(number);
    // println!("Compare {:?} and {:?}. Difference, {:?}", cur_number, number, difference);
    if (1..=3).contains(&difference)
        && ((cur_number > number && flag == "increase")
            || (cur_number < number && flag == "decrease"))
    {
        return true;
    }
    false
}

fn day_2(content: &Vec<String>) {
    let mut result = 0;
    for line in content {
        let mut safe_count = 0;
        let mut flag = "";
        let mut dumpener = 0;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut number = parts[0].parse::<usize>().unwrap();

        for i in 1..parts.len() {
            let cur_number = parts[i].parse::<usize>().unwrap();
            if i == 1 && cur_number > number {
                flag = "increase";
            } else if i == 1 && cur_number < number {
                flag = "decrease";
            }

            if is_safe(cur_number, number, flag) {
                safe_count += 1;
            } else if parts.get(i + 1).is_none() {
                safe_count += 1;
            } else if dumpener < 1 && is_safe(parts[i + 1].parse::<usize>().unwrap(), number, flag)
            {
                safe_count += 1;
                dumpener += 1;
            }
            number = cur_number;
        }

        if safe_count == (parts.len() - 1) {
            result += 1;
        }
    }

    println!("Day 2");
    println!("First/second part result: {result}");
}

fn day_3(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result_1 = 0;
    let mut result_2 = 0;

    let pattern = r"(mul|do(?:n\'t)?)\((?:(\d+),(\d+))?\)";
    let re = Regex::new(pattern).unwrap();
    let mut active: bool = true;

    for cap in re.captures_iter(&content) {
        let instruction = &cap[1];

        if instruction == "mul" {
            let num_1: usize = cap[2].parse().unwrap();
            let num_2: usize = cap[3].parse().unwrap();
            result_1 += num_1 * num_2;

            if active {
                result_2 += num_1 * num_2;
            } else {
                result_2 += 0;
            }
        }
        if instruction == "do" {
            active = true;
        }
        if instruction == "don't" {
            active = false;
        }
    }

    println!("Day 3");
    println!("Result part 1: {:?}", result_1);
    println!("Result part 2: {:?}", result_2);
}

fn main() -> io::Result<()> {
    // let content = read_file("data/day_2_input.txt");
    // day_1(&content);
    // day_2(&content);
    day_3("data/day_3_input.txt");
    Ok(())
}
