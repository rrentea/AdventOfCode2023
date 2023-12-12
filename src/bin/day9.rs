use aoc2023::utils;

fn predict(values: &[i64]) -> i64 {
    if values.iter().all(|value| *value == 0) {
        0
    } else {
        let mut new_values = vec![];
        for i in 1..values.len() {
            new_values.push(values[i] - values[i - 1]);
        }
        values.last().unwrap() + predict(&new_values)
    }
}

fn predict_past(values: &[i64]) -> i64 {
    if values.iter().all(|value| *value == 0) {
        0
    } else {
        let mut new_values = vec![];
        for i in 1..values.len() {
            new_values.push(values[i] - values[i - 1]);
        }
        values.first().unwrap() - predict_past(&new_values)
    }
}

fn part1(input: &str) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        let mut history: Vec<i64> = vec![];
        for number in line.split(' ') {
            history.push(number.parse::<i64>().unwrap())
        }

        histories.push(history)
    }

    histories.iter().map(|history| predict(history)).sum()
}

fn part2(input: &str) -> i64 {
    let mut histories: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        let mut history: Vec<i64> = vec![];
        for number in line.split(' ') {
            history.push(number.parse::<i64>().unwrap())
        }

        histories.push(history)
    }

    histories.iter().map(|history| predict_past(history)).sum()
}

fn main() {
    match utils::read_file("src/data/day9/input") {
        Ok(input) => {
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
