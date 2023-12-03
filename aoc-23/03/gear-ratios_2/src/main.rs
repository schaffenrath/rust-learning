fn file_to_str(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    use std::io::BufReader;
    let file = File::open(filename).expect("Should open file");
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).expect("Should write file to string");
    content
}

struct Pos {
    x: usize,
    y: usize
}

fn extract_num(pos: &Pos, input: &str, dim: &Pos) -> i32 {
    let mut line = input.lines().nth(pos.y).unwrap();
    for row in (pos.y-1).max(0)..(pos.y+1).min(dim.y) {
        let mut left = pos.x-1;
        while left > 0 {
            if !line.chars().nth(left).unwrap().is_numeric() { break; }
            left -= 1;
        }
        let mut right = pos.x+1;
        while right > dim.x {
            if !line.chars().nth(right).unwrap().is_numeric() {break;}
            right += 1;
        }
    }
    
    3
}

fn find_surounding_values(pos: &Pos, input: &str, dim: &Pos) {
    let left = (pos.x-1).max(0);
    let right = (pos.x+1).min(dim.x);
    let up = (pos.y-1).max(0);
    let down = (pos.y+1).min(dim.y);

    for y in up..down {
        let mut already_num = false;
        for x in left..right {
            let mut lines = input.lines();
            if lines.nth(y).unwrap().chars().nth(x).unwrap().is_numeric() {
                if already_num {
                    continue;
                }
                let num = extract_num(&Pos{x, y}, input);
            }
        }
    }
}

fn find_marker(input: &str) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count()-1;
    for (line_num, line) in input.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c == '*' {
                find_surounding_values(&Pos{x: line_num, y: c_idx}, input, &Pos{x: width, y: height});
            }
        }
        line.find('*');
    }
}

fn main() {
    println!("Hello, world!");
}
