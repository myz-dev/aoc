//! Cube Conundrum

use parser::parse_input;

use crate::parser::{Game, Set};

mod parser;

fn main() {
    //     let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let input = std::fs::read_to_string("./task02/input.txt").unwrap();
    let games = parse_input(&input);

    // ----------------- TASK 01

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let id_of_possible_game = |g: &Game| -> Option<u32> {
        let not_enough_reds = g.sets.iter().any(|s| s.red > max_red);
        let not_enough_blues = g.sets.iter().any(|s| s.blue > max_blue);
        let not_enough_green = g.sets.iter().any(|s| s.green > max_green);
        if not_enough_blues || not_enough_green || not_enough_reds {
            None
        } else {
            Some(g.id)
        }
    };
    let id_sum_task_1: u32 = games.iter().filter_map(id_of_possible_game).sum();

    println!("Answer task 1: {id_sum_task_1}");

    // ----------------- TASK 02
    // let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    // let games = parse_input(input);

    let max_triple_in_game = |sets: &[Set]| -> Set {
        let max_values = sets
            .iter()
            .fold(Set::new_empty(), |cur_max, next| cur_max.eat_max(next));
        max_values
    };

    let power_sum: u32 = games
        .iter()
        .map(|g| max_triple_in_game(&g.sets))
        .map(|s| s.red * s.green * s.blue)
        .sum();
    println!("Answer task 2: {power_sum}");
}
