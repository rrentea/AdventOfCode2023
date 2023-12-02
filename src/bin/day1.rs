use aoc2023::utils;

fn get_first_digit(s: String, digits: &[(&str, u32)]) -> Option<u32> {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap().is_ascii_digit() {
            return Some(s.chars().nth(i).unwrap().to_digit(10).unwrap());
        }
        for (word, digit) in digits.clone() {
            let pos = s[i..].find(word);
            if pos.is_some() && pos.unwrap() == 0 {
                return Some(*digit);
            }
        }
    }

    None
}

fn get_last_digit(s: String, digits: &[(&str, u32)]) -> Option<u32> {
    for i in (0..s.len()).rev() {
        if s.chars().nth(i).unwrap().is_ascii_digit() {
            return Some(s.chars().nth(i).unwrap().to_digit(10).unwrap());
        }
        for (word, digit) in digits.clone() {
            let pos = s[i..].find(word);
            if pos.is_some() && pos.unwrap() == 0 {
                return Some(*digit);
            }
        }
    }

    None
}

fn get_calibration_values(s: &str) -> u32 {
    let digits = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let first_digit = get_first_digit(s.to_string(), &digits).unwrap();
    let last_digit = get_last_digit(s.to_string(), &digits).unwrap();

    first_digit * 10 + last_digit
}

fn main() {
    match utils::read_file("src/data/day1/input.txt") {
        Ok(content) => {
            let sum_of_values: u32 = content.lines().map(get_calibration_values).sum();
            println!("{}", sum_of_values);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
