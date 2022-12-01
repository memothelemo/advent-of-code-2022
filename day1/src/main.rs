use std::fs;

fn parse_input(input: &str) -> Vec<usize> {
    let mut temp = 0usize;
    let mut vector: Vec<_> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                Some(std::mem::take(&mut temp))
            } else {
                temp += line.parse::<usize>().unwrap();
                None
            }
        })
        .collect();

    if temp > 0 {
        vector.push(temp);
    }

    vector
}

fn puzzle_one(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort();
    *input.last().unwrap()
}

fn puzzle_two(input: &[usize]) -> usize {
    let mut input = input.to_vec();
    input.sort_by(|a, b| b.cmp(a));
    assert!(input.len() >= 3);
    input[0] + input[1] + input[2]
}

fn main() {
    // We're expecting people to use the root directory
    // to read the input file right here...
    let input = fs::read_to_string("day1/input.txt").unwrap();
    let calories_per_elf = parse_input(&input);

    let puzzle_one = puzzle_one(&calories_per_elf);
    let puzzle_two = puzzle_two(&calories_per_elf);

    println!("Puzzle #1: {puzzle_one}");
    println!("Puzzle #2: {puzzle_two}");
}
