use crate::parsing::parse_line;
use crate::structs::{Cubes, Game, MinGame};
use std::cmp::max;
use std::fs;

fn get_min_cubes(game: &Game) -> Cubes {
    let mut min_draw = Cubes {
        red: 0,
        blue: 0,
        green: 0,
    };
    for draw in game.draws.iter() {
        min_draw.red = max(draw.red, min_draw.red);
        min_draw.green = max(draw.green, min_draw.green);
        min_draw.blue = max(draw.blue, min_draw.blue);
    }
    min_draw
}

pub fn task_2() {
    let lines = fs::read_to_string("data/input_1.txt").unwrap();
    let mut games: Vec<Game> = Vec::new();
    for line in lines.lines() {
        games.push(parse_line(line))
    }
    let min_games = games
        .iter()
        .map(|x| MinGame {
            id: x.id,
            min_draw: get_min_cubes(x),
        })
        .collect::<Vec<MinGame>>();
    let result = min_games
        .iter()
        .map(|g| g.min_draw.red * g.min_draw.blue * g.min_draw.green)
        .sum::<u32>();
    println!("{result}")
}
