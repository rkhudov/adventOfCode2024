use multimap::MultiMap;
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

fn grid_rotate(grid: &mut Vec<Vec<char>>) {
    let n = grid.len();
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let temp = grid[i][j];
            grid[i][j] = grid[j][n - 1 - i];
            grid[j][n - 1 - i] = grid[n - 1 - i][n - 1 - j];
            grid[n - 1 - i][n - 1 - j] = grid[n - 1 - j][i];
            grid[n - 1 - j][i] = temp;
        }
    }
}

fn count_horizonal_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                let mut a = 'a';
                let mut b = 'b';
                let mut c = 'c';
                match grid.get(row).and_then(|inner| inner.get(col + 1)) {
                    Some(value) => a = *value,
                    None => break,
                }
                match grid.get(row).and_then(|inner| inner.get(col + 2)) {
                    Some(value) => b = *value,
                    None => break,
                }
                match grid.get(row).and_then(|inner| inner.get(col + 3)) {
                    Some(value) => c = *value,
                    None => break,
                }
                if a == 'M' && b == 'A' && c == 'S' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_diagonal_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                let mut a = 'a';
                let mut b = 'b';
                let mut c = 'c';
                match grid.get(row + 1).and_then(|inner| inner.get(col + 1)) {
                    Some(value) => a = *value,
                    None => break,
                }
                match grid.get(row + 2).and_then(|inner| inner.get(col + 2)) {
                    Some(value) => b = *value,
                    None => break,
                }
                match grid.get(row + 3).and_then(|inner| inner.get(col + 3)) {
                    Some(value) => c = *value,
                    None => break,
                }
                if a == 'M' && b == 'A' && c == 'S' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_x_max(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'M' {
                let mut a = 'a';
                let mut b = 'b';
                let mut c = 'c';
                let mut d = 'd';
                match grid.get(row).and_then(|inner| inner.get(col + 2)) {
                    Some(value) => a = *value,
                    None => break,
                }
                match grid.get(row + 1).and_then(|inner| inner.get(col + 1)) {
                    Some(value) => b = *value,
                    None => break,
                }
                match grid.get(row + 2).and_then(|inner| inner.get(col)) {
                    Some(value) => c = *value,
                    None => break,
                }
                match grid.get(row + 2).and_then(|inner| inner.get(col + 2)) {
                    Some(value) => d = *value,
                    None => break,
                }
                if a == 'M' && b == 'A' && c == 'S' && d == 'S' {
                    count += 1;
                }
            }
        }
    }
    count
}

fn day_4(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in content.split("\n") {
        grid.push(line.chars().collect());
    }

    for _ in 0..4 {
        result_1 += count_horizonal_xmas(&grid);
        result_1 += count_diagonal_xmas(&grid);
        result_2 += count_x_max(&grid);
        grid_rotate(&mut grid);
    }
    println!("Day 4");
    println!("Result part 1: {:?}", result_1);
    println!("Result part 2: {:?}", result_2);
}

fn bubble_sort_with_rules(mut vec: Vec<i32>, multi_map: &MultiMap<i32, i32>) -> Vec<i32> {
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..vec.len() - 1 {
            let current = vec[i];
            let next = vec[i + 1];

            // Check if `next` can follow `current` based on the multi_map
            if let Some(followers) = multi_map.get_vec(&current) {
                if !followers.contains(&next) {
                    // If `next` cannot follow `current`, swap them
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            } else {
                // If current element has no followers, no restriction, swap if needed
                if current > next {
                    vec.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }

    vec
}

fn day_5(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut left_to_right_rules = MultiMap::new();
    let mut right_to_left_rules = MultiMap::new();
    let mut not_valid_instructions: Vec<Vec<i32>> = Vec::new();
    let mut switch = false;
    for line in content.split("\n") {
        if line == "" {
            switch = true;
            continue;
        }
        if !switch {
            let parts: Vec<&str> = line.split("|").collect();
            let left_instruction = parts[0].parse::<i32>().unwrap();
            let right_instruction = parts[1].parse::<i32>().unwrap();
            left_to_right_rules.insert(left_instruction, right_instruction);
            right_to_left_rules.insert(right_instruction, left_instruction);
        } else {
            let mut instructions: Vec<i32> = Vec::new();
            for instruction in line.split(",") {
                instructions.push(instruction.parse::<i32>().unwrap());
            }
            let mut take = true;
            println!("Instructions {:?}", instructions);
            for i in 0..instructions.len() {
                let instruction = instructions[i];
                if i == 0 {
                    let allowed_instructions = left_to_right_rules.get_vec(&instruction).unwrap();
                    let rest_instructions = &instructions[1..];
                    let first = rest_instructions
                        .iter()
                        .all(|x| allowed_instructions.contains(x));

                    if !first {
                        println!("Not valid first {:?}", instruction);
                        take = false;
                        not_valid_instructions.push(instructions.clone());
                        let new_instructions =
                            bubble_sort_with_rules(instructions.clone(), &left_to_right_rules);
                        println!("New instructions {:?}", new_instructions);
                        result_2 += new_instructions[new_instructions.len() / 2];
                        break;
                    }
                } else if i == instructions.len() - 1 {
                    let allowed_instructions = right_to_left_rules.get_vec(&instruction).unwrap();
                    let rest_instructions = &instructions[..&instructions.len() - 1];
                    let last = rest_instructions
                        .iter()
                        .all(|x| allowed_instructions.contains(x));

                    if !last {
                        println!("Not valid last {:?}", instruction);
                        take = false;
                        not_valid_instructions.push(instructions.clone());
                        let new_instructions =
                            bubble_sort_with_rules(instructions.clone(), &right_to_left_rules);
                        println!("New instructions {:?}", new_instructions);
                        result_2 += new_instructions[new_instructions.len() / 2];
                        break;
                    }
                } else {
                    // let allowed_instructions = left_to_right_rules.get_vec(&instruction).unwrap();
                    let right_allowed_instructions = left_to_right_rules.get_vec(&instruction);
                    let allowed_instructions;
                    match right_allowed_instructions {
                        Some(val) => {
                            allowed_instructions = val;
                        }
                        None => {
                            println!("Not valid");
                            let mut new_instructions =
                                bubble_sort_with_rules(instructions.clone(), &left_to_right_rules);
                            println!("New instructions {:?}", new_instructions);
                            new_instructions.reverse();
                            let mut new_new_instructions = bubble_sort_with_rules(
                                new_instructions.clone(),
                                &right_to_left_rules,
                            );
                            new_new_instructions.reverse();
                            println!("New new instructions {:?}", new_new_instructions);
                            result_2 += new_new_instructions[new_new_instructions.len() / 2];
                            take = false;
                            break;
                        }
                    }
                    let right_rest_instructions = &instructions[i + 1..];
                    let check = right_rest_instructions
                        .iter()
                        .all(|x| allowed_instructions.contains(x));

                    if !check {
                        println!("Instructions {:?}", instructions);
                        println!("Not valid middle {:?}", instruction);
                        take = false;
                        not_valid_instructions.push(instructions.clone());
                        let new_instructions =
                            bubble_sort_with_rules(instructions.clone(), &left_to_right_rules);
                        println!("New instructions {:?}", new_instructions);
                        result_2 += new_instructions[new_instructions.len() / 2];
                        break;
                    }
                }
            }
            if take {
                println!("Valid");
                result_1 += instructions[instructions.len() / 2];
            } else {
                continue;
            }
        }
    }
    println!("Day 4");
    println!("Result part 1: {:?}", result_1);
    println!("Result part 2: {:?}", result_2);
}

fn main() -> io::Result<()> {
    // let content = read_file("data/day_2_input.txt");
    // day_1(&content);
    // day_2(&content);
    // day_3("data/day_3_input.txt");
    // day_4("data/day_4_input.txt");
    day_5("data/day_5_input.txt");
    Ok(())
}
