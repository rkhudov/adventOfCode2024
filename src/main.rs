use multimap::MultiMap;
use regex::Regex;
use std::collections::{HashSet, HashMap};
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
    println!("Day 5");
    println!("Result part 1: {:?}", result_1);
    println!("Result part 2: {:?}", result_2);
}

fn get_next_position(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> char {
    let next_position = '?';
    if let Some(row) = grid.get(row_index) {
        if let Some(value) = row.get(col_index) {
            return *value;
        } else {
            return next_position;
        }
    }
    next_position
}

fn day_6(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut guard_row_index = 0;
    let mut guard_column_index = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for (row, line) in content.split("\n").enumerate() {
        let mut grid_row: Vec<char> = Vec::new();

        for (column, ch) in line.char_indices() {
            if ch == '^' {
                grid_row.push('.');
                guard_row_index = row;
                guard_column_index = column;
            } else {
                grid_row.push(ch);
            }
        }
        grid.push(grid_row);
    }
    println!("{:?}", grid);
    let initial_guard_row_index = guard_row_index;
    let initial_guard_column_index = guard_column_index;
    let original_grid = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' || grid[i][j] == '#' {
                continue;
            }
            grid[i][j] = '#';
            println!("Placing # element in position {:?} {:?}", i, j);
            // println!("grid looks like: {:?}", grid);

            let mut flag = true;
            let mut direction = '^';
            let mut recorded_positions: HashSet<Vec<usize>> = HashSet::new();

            guard_row_index = initial_guard_row_index;
            guard_column_index = initial_guard_column_index;
            let mut hack = 0;

            while flag {
                if hack > 10000 {
                    println!("Loop found!");
                    result_2 += 1;
                    flag = false;
                }
                // println!("Guard in position {:?} {:?}", guard_row_index, guard_column_index);
                // println!("Direction {direction}");
                match direction {
                    '^' => {
                        if guard_row_index == 0 {
                            flag = false;
                            break;
                        }
                        guard_row_index -= 1;
                        let position =
                            get_next_position(&grid, guard_row_index, guard_column_index);
                        match position {
                            '?' => flag = false,
                            '.' => {
                                let initial_len = recorded_positions.len();
                                recorded_positions
                                    .insert(vec![guard_row_index, guard_column_index]);
                                let modified_len = recorded_positions.len();
                                if initial_len == modified_len {
                                    hack += 1;
                                }
                            }
                            '#' => {
                                guard_row_index += 1;
                                direction = '>';
                            }
                            _ => println!("Not the case"),
                        }
                    }
                    '>' => {
                        guard_column_index += 1;
                        let position =
                            get_next_position(&grid, guard_row_index, guard_column_index);
                        match position {
                            '?' => flag = false,
                            '.' => {
                                let initial_len = recorded_positions.len();
                                recorded_positions
                                    .insert(vec![guard_row_index, guard_column_index]);
                                let modified_len = recorded_positions.len();
                                if initial_len == modified_len {
                                    hack += 1;
                                }
                            }
                            '#' => {
                                guard_column_index -= 1;
                                direction = 'v';
                            }
                            _ => println!("Not the case"),
                        }
                    }
                    'v' => {
                        guard_row_index += 1;
                        let position =
                            get_next_position(&grid, guard_row_index, guard_column_index);
                        match position {
                            '?' => flag = false,
                            '.' => {
                                let initial_len = recorded_positions.len();
                                recorded_positions
                                    .insert(vec![guard_row_index, guard_column_index]);
                                let modified_len = recorded_positions.len();
                                if initial_len == modified_len {
                                    hack += 1;
                                }
                            }
                            '#' => {
                                guard_row_index -= 1;
                                direction = '<';
                            }
                            _ => println!("Not the case"),
                        }
                    }
                    '<' => {
                        if guard_column_index == 0 {
                            flag = false;
                            break;
                        }
                        guard_column_index -= 1;
                        let position =
                            get_next_position(&grid, guard_row_index, guard_column_index);
                        match position {
                            '?' => flag = false,
                            '.' => {
                                let initial_len = recorded_positions.len();
                                recorded_positions
                                    .insert(vec![guard_row_index, guard_column_index]);
                                let modified_len = recorded_positions.len();
                                if initial_len == modified_len {
                                    hack += 1;
                                }
                            }
                            '#' => {
                                guard_column_index += 1;
                                direction = '^';
                            }
                            _ => println!("Not the case"),
                        }
                    }
                    _ => println!("Not the case"),
                }
            }

            grid = original_grid.clone();

            result_1 = recorded_positions.len();
            println!("Result part 1: {:?}", result_1);
        }
    }
    println!("Day 6");
    println!("Result part 2: {:?}", result_2);
}

fn can_combine(numbers: &Vec<usize>, target: usize, cur_value: usize) -> bool {
    if cur_value == target {
        return true;
    } else if cur_value > target {
        return false;
    }

    for (i, &number) in numbers.iter().enumerate() {
        let mut remaining = numbers.to_vec();
        remaining.remove(i);

        if cur_value != 0 && can_combine(&remaining, target, cur_value * number) {
            return true;
        }

        if can_combine(&remaining, target, cur_value + number) {
            return true;
        }
    }
    false
}

fn possible_targets(targets: Vec<i64>, number: i64) -> Vec<i64> {
    let mut new_targets: Vec<i64> = Vec::new();
    if targets.is_empty() {
        new_targets.push(number);
        return new_targets;
    }

    for target in targets {
        new_targets.push(target + number);
        new_targets.push(target * number);
        new_targets.push((target.to_string() + &number.to_string()).parse().unwrap());
    }

    new_targets
}

fn day_7(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result = 0;

    for line in content.split('\n') {
        // println!("Line {line}");
        let parts: Vec<&str> = line.split(':').collect();
        let target = parts[0].parse::<i64>().unwrap();
        let mut numbers: Vec<i64> = Vec::new();
        for number in parts[1].split_whitespace() {
            numbers.push(number.parse::<i64>().unwrap());
        }

        // println!("Target: {target}; numbers {:?}", numbers);
        let possible_targets = numbers
            .iter()
            .fold(Vec::new(), |acc, &num| possible_targets(acc, num));
        if possible_targets.contains(&target) {
            result += target;
        }
    }

    println!("Day 7");
    println!("Result part 1/2: {result}");
}

fn day_8(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut antennas_to_coordinates_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (row_index , line) in content.lines().enumerate() {
        let row_index_i = row_index as i32;
        let mut grid_row = Vec::new();
        for (col_index, ch) in line.chars().enumerate() {
            let col_index_j = col_index as i32;
            grid_row.push(ch);
            if ch != '.' {
                antennas_to_coordinates_map.entry(ch).or_insert(vec![]).push((row_index_i, col_index_j));
            }
        }
        grid.push(grid_row);
    }
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut antinodes_coordinates: HashSet<(i32, i32)> = HashSet::new();
    for (_, antenna_coordinates) in antennas_to_coordinates_map {
        for i in 0..antenna_coordinates.len() {
            for j in (i+1)..antenna_coordinates.len() {
                let row_diff = antenna_coordinates[j].0 - antenna_coordinates[i].0;
                let col_diff = antenna_coordinates[j].1 - antenna_coordinates[i].1;

                for (x, y) in [
                    (antenna_coordinates[i], (-row_diff, -col_diff)),
                    (antenna_coordinates[j], (row_diff, col_diff)),
                ] {
                    for a in 0..rows { // set a to 1, to get result for part 1
                        let antinode_coordinates = (x.0 + y.0 * a, x.1 + y.1 * a);
                        if antinode_coordinates.0 >= 0 && antinode_coordinates.0 < rows && antinode_coordinates.1 >= 0 && antinode_coordinates.1 < cols {
                            antinodes_coordinates.insert(antinode_coordinates);
                        }
                    }
                }
            }
        }
    }
    result = antinodes_coordinates.len();

    println!("Day 8");
    println!("Result part 1/2: {result}");
}

fn day_9(filename: &str) {
    let content = fs::read_to_string(filename).expect("Unable to read file");
    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut disk_map: Vec<String> = Vec::new();
    let mut file_id = 0;
    let mut file_blocks_index: Vec<usize> = Vec::new();
    let mut file_blocks: Vec<u32> = Vec::new();

    for (index, ch) in content.chars().enumerate() {
        let number = ch.to_digit(10).unwrap();
        if index % 2 == 0 {
            for _ in 0..number {
                file_blocks_index.push(disk_map.len());
                disk_map.push(file_id.to_string());
                file_blocks.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..number {
                disk_map.push('.'.to_string());
            }
        }
    }
    file_blocks_index.reverse();
    file_blocks.reverse();
    let mut disk_map_clone = disk_map.clone();
    let mut index = 0;

    for (i, element) in disk_map.into_iter().enumerate() {
        if element == "." {
            if i < file_blocks_index[index] {
                disk_map_clone.swap(i, file_blocks_index[index]);
            } else {
                break;
            }
            index += 1;
        }
    }

    for (i, element) in disk_map_clone.into_iter().enumerate() {
        if element != "." {
            let number = element.parse::<usize>().unwrap();
            result_1 += number * i;
        }
    }

    println!("Day 9");
    println!("Result part 1: {result_1}");
    println!("Result part 2: {result_2}");
}

fn main() -> io::Result<()> {
    // let content = read_file("data/day_2_input.txt");
    // day_1(&content);
    // day_2(&content);
    // day_3("data/day_3_input.txt");
    // day_4("data/day_4_input.txt");
    // day_5("data/day_5_input.txt");
    // day_6("data/day_6_input.txt");
    // day_7("data/day_7_input.txt");
    // day_8("data/day_8_input.txt");
    day_9("data/day_9_input.txt");
    Ok(())
}
