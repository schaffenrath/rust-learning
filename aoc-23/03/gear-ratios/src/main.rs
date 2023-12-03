use std::collections::btree_map::Range;


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

struct PartNum {
    value: i32,
    start_pos: i32,
    end_pos: i32    
}

struct Schematic {
    part_matrix: Vec<Vec<PartNum>>,
    symbol_matrix: Vec<Vec<i32>>,
    part_sum: i32
}

impl Schematic {
    fn create_matrix_from_str(&mut self, input: &str) {
        let max_lines = input.lines().count();
        let line_width = input.lines().last().unwrap().len();
        for (line_num, line) in input.lines().enumerate() {
            let mut num_start: i32 = -1;
            let mut num_str = String::new();
            for (char_idx, entry) in line.chars().enumerate() {
                if entry.is_numeric() {
                    if num_start == -1 { num_start = i32::try_from(char_idx).unwrap(); }
                    num_str.push(entry);
                } else {
                    if num_start != -1 {
                        let end_pos: i32 = i32::try_from(char_idx).unwrap() - 1;
                        let part = PartNum { value: num_str.parse().unwrap(), start_pos: num_start, end_pos: end_pos};
                        if self.find_symbol(&input, &part, line_num, max_lines, line_width) {
                            self.part_sum += part.value;
                        }
                        num_start = -1;
                        num_str.clear();
                    }
                    
                }
                if entry.is_numeric() && char_idx == line_width-1 {
                        let end_pos: i32 = i32::try_from(char_idx).unwrap();
                        let part = PartNum { value: num_str.parse().unwrap(), start_pos: num_start, end_pos: end_pos};
                        if self.find_symbol(&input, &part, line_num, max_lines, line_width) {
                            self.part_sum += part.value;
                        }

                }
            }
        }
    }

    fn find_symbol_in_line(&self, line: &str, part: &PartNum, line_width: usize) -> bool {
        for i in (part.start_pos-1).max(0)..(part.end_pos+2).min(i32::try_from(line_width-1).unwrap()) {
            let mut characters = line.chars();
            let c = characters.nth(usize::try_from(i).unwrap()).unwrap();
            if !c.is_numeric() && c != '.'  {
                return true;
            }
        }
        false
    }

    fn find_symbol(&self, input: &str, part: &PartNum, line_num: usize, max_lines: usize, max_width: usize) -> bool {
        if line_num != 0 {
            if self.find_symbol_in_line(input.lines().nth(line_num-1).unwrap(), part, max_width) {return true;}
        }
        // Check current line
        let left = usize::try_from((part.start_pos-1).max(0)).unwrap();
        let right = usize::try_from((part.end_pos+1).min(i32::try_from(max_width-1).unwrap())).unwrap();
        let curr_line = input.lines().nth(line_num).unwrap();
        for i in [left,right] {
            let c = curr_line.chars().nth(i).unwrap();
            if !c.is_numeric() && c != '.' {
                return true;
            }

        }
        if line_num != max_lines-1 {
            if self.find_symbol_in_line(input.lines().nth(line_num+1).unwrap(), part, max_width) {return true;}
        }

        false
    }

    fn get_sum(&self) -> i32 {
        self.part_sum
    }
}

fn main() {
    let content = file_to_str("./gear.txt");
    let mut schem = Schematic {part_matrix: Vec::new(), symbol_matrix: Vec::new(), part_sum: 0};
    schem.create_matrix_from_str(&content);
    println!("Sum: {}", schem.get_sum());
}
