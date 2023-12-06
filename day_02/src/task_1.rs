use crate::parsing::parse_line;
use crate::structs::{Cubes, Game};
use std::fs;

pub fn task_1() {
    let lines = fs::read_to_string("data/test_input_1.txt").unwrap();
    let mut games: Vec<Game> = Vec::new();
    let max_game = Cubes {
        blue: 14,
        red: 12,
        green: 13,
    };
    for line in lines.lines() {
        let parsed_game = parse_line(line);
        let mut valid_game = true;
        for draw in parsed_game.draws.iter() {
            if draw.green > max_game.green || draw.blue > max_game.blue || draw.red > max_game.red {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            games.push(parsed_game);
        }
    }
    // for game in games.iter() {
    //     println!("{:?}", game)
    // }
    println!("{}", games.iter().map(|x| x.id).sum::<u32>())
}
