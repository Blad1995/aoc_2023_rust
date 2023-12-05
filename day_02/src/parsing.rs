use crate::structs::{Cubes, Game};
use regex::{Captures, Regex};

fn get_value_or_zero(re_match: Option<Captures>) -> u32 {
    match re_match {
        Some(x) => x.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        None => 0,
    }
}

pub fn parse_line(line: &str) -> Game {
    let game_id_regex = Regex::new("Game ([0-9]+):").unwrap();
    let game_id = game_id_regex
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    let blue_re = Regex::new("([0-9]+) blue").unwrap();
    let green_re = Regex::new("([0-9]+) green").unwrap();
    let red_re = Regex::new("([0-9]+) red").unwrap();

    let mut cubes: Vec<Cubes> = Vec::new();
    for draw in line.split(";") {
        let blue = get_value_or_zero(blue_re.captures(draw));
        let green = get_value_or_zero(green_re.captures(draw));
        let red = get_value_or_zero(red_re.captures(draw));
        cubes.push(Cubes { red, blue, green })
    }
    return Game {
        id: game_id,
        draws: cubes,
    };
}
