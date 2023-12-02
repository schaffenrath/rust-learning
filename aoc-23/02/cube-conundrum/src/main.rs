/*
You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

Your puzzle answer was 2207.
--- Part Two ---

The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
    Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
    Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
    Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?

Your puzzle answer was 62241.
 */


use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Eq, Hash, PartialEq)]
enum Color {
    Blue,
    Red,
    Green
}

#[derive(Eq, Hash, PartialEq)]
struct Game {
    color: Color,
    count: i32
}

fn get_file_as_str(filename: &str) -> String {
    use std::fs::File;
    use std::io::BufReader;
    let file = File::open(filename)
                    .expect("Should open file");
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Should read file to string");
    content
}

fn get_color_from_str(input: &str) -> Option<Color> {
    if input == "blue" {
        return Some(Color::Blue);
    } else if input  == "red" {
        return Some(Color::Red);
    } else if input == "green" {
        return Some(Color::Green);
    } else {
        println!("Try to convert invalid string: {}", input);
        return None;
    }
}

fn count_valid_games(content: &str, total_cubes: &HashMap<Color,i32>) -> Option<i32> {
    let mut game_sum: i32 = 0;
    'input_line: for line in content.lines() {
        let mut min_cubes = HashMap::new();
        min_cubes.insert(Color::Blue, 0);
        min_cubes.insert(Color::Green, 0);
        min_cubes.insert(Color::Red, 0);
        let games: Vec<&str> = line.split(':').collect();
        if games.len() != 2 {
            println!("Invalid form: missing separation through ':'");
            return None;
        }
        let rounds: Vec<&str> = games[1].split(';').collect();
        for round in rounds {
            let round_confs: Vec<&str> = round.split(&[' ', ',']).collect();
            let mut cube_count = 0;
            for round_conf in round_confs {
                if round_conf.is_empty() {
                    continue;
                }
                if cube_count == 0 {
                    cube_count = round_conf.parse().unwrap();
                } else {
                    let cube_color = get_color_from_str(round_conf).unwrap();
                    // Second part
                    if min_cubes[&cube_color] < cube_count {
                        *min_cubes.get_mut(&cube_color).unwrap() = cube_count;
                    }
                    // First part
                    // if total_cubes[&cube_color.unwrap()] < cube_count {
                    //     continue 'input_line; 
                    // }
                    cube_count = 0;
                }
            }
        }
        // Second part
        let game_prod = min_cubes[&Color::Red] * min_cubes[&Color::Blue] * min_cubes[&Color::Green];
        game_sum += game_prod;
        // First part
        // let game_num_alpha: &str = games[0].split(' ').last().unwrap();
        // let game_num: i32 = game_num_alpha.parse().unwrap();
        // game_sum += game_num;
    }
    Some(game_sum)
}


fn main() {
    let content = get_file_as_str("./input.txt");
    let mut total_cubes = HashMap::new();
    total_cubes.insert(Color::Red, 12);
    total_cubes.insert(Color::Green, 13);
    total_cubes.insert(Color::Blue, 14);
    let sum = count_valid_games(&content, &total_cubes);
    println!("Sum: {}", sum.unwrap());
}
