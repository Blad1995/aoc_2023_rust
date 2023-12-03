use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;

fn main() {
    let mapping: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let input_name = "input/input_2.txt";
    let file_data = fs::read_to_string(input_name).unwrap();
    let lines = file_data.lines();

    let mut total_sum: u32 = 0;
    for line in lines {
        let mut first_num: Option<char> = Option::None;
        let mut last_num: Option<char> = Option::None;
        let mut i = 0;
        while i < line.len() {
            if line.chars().nth(i).unwrap().is_ascii_digit() {
                match first_num {
                    Some(_) => last_num = Some(line.chars().nth(i).unwrap()),
                    None => {
                        first_num = Some(line.chars().nth(i).unwrap());
                        last_num = first_num;
                    }
                }
            } else {
                for (key, value) in mapping.iter() {
                    if line[i..].starts_with(key) {
                        match first_num {
                            Some(_) => last_num = Some(value.chars().next().unwrap()),
                            None => {
                                first_num = Some(value.chars().next().unwrap());
                                last_num = first_num;
                            }
                        }
                        break;
                    }
                }
            }
            i += 1;
        }
        let line_number = format!(
            "{}{}",
            first_num.expect("Cannot be None"),
            last_num.expect("Cannot be None")
        )
        .to_string();
        total_sum += line_number.parse::<u32>().unwrap();
    }
    println!("{total_sum}");
}
