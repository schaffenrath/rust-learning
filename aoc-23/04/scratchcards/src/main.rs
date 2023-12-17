use std::collections::HashSet;
use std::collections::HashMap;


fn file_to_str(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::io::BufReader;
    let file = File::open(filename).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();
    content
}

fn calc_points(input: &str) -> i32 {
    let mut total_score = 0;
    for line in input.lines() {
        let prefix_end = line.find(':').unwrap_or(0);
        let nums: Vec<&str> = line[(prefix_end+1)..].split('|').collect();
        let solutions: HashSet<&str> = nums[0].split(' ').collect();
        let mut sol_count = 0;
        for val in nums[1].split(' ').into_iter() {
            // Dirty filtering, for sure could be done easier
            if val.is_empty() || val == " " { continue; }
            if solutions.contains(val) {
                if sol_count == 0 { sol_count = 1; } else { sol_count *= 2; }
            }
        }
        total_score += sol_count;
    }
    total_score
}

fn calc_cards(input: &str) -> i32 {
    let mut card_count = HashMap::new();
    for line in input.lines() {
        let prefix_end = line.find(':').unwrap_or(0);
        let card_num: i32 = line[5..prefix_end].trim_start().parse().unwrap();
        // Add original card
        card_count.entry(card_num).and_modify(|c| *c+=1).or_insert(1);
        let nums: Vec<&str> = line[(prefix_end+1)..].split('|').collect();
        let solutions: HashSet<&str> = nums[0].split(' ').collect();
        let mut sol_count = 0;
        for val in nums[1].split(' ').into_iter() {
            // Dirty filtering, for sure could be done easier
            if val.is_empty() || val == " " { continue; }
            if solutions.contains(val) {
                sol_count += 1;
            }
        }
        if sol_count > 0 {
            for i in 1..sol_count+1 {
                let card_instances = *card_count.get(&card_num).unwrap();
                // Add copy of card
                card_count.entry(card_num+i).and_modify(|c| *c+=card_instances).or_insert(card_instances);
            }
        }
    }
    card_count.values().sum()
}

fn main() {
    let input = file_to_str("./input.txt");
    let sol = calc_cards(&input);
    println!("Solution: {}", sol);
}
