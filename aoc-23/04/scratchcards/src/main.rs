use std::collections::HashSet;


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
        let mut solutions = HashSet::new();
        solutions = nums[0].split(' ').collect();
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

fn main() {
    let input = file_to_str("./input.txt");
    let sol = calc_points(&input);
    println!("Solution: {}", sol);
}
