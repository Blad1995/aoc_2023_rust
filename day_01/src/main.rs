use std::fs;

fn main() {
    let input_name = "input/input_1.txt";
    let file_data = fs::read_to_string(input_name).unwrap();
    let lines = file_data.lines();
    let mut total_sum: u32 = 0;
    for line in lines {
        let mut first_num: Option<char> = Option::None;
        let mut last_num: Option<char> = Option::None;
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                match first_num {
                    Some(_) => last_num = Some(letter),
                    None => {
                        first_num = Some(letter);
                        last_num = first_num;
                    }
                }
            }
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
