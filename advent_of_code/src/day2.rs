use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn day2a() -> usize {
    return shared_logic(|start_val, end_val, target_char, password| {
        let mut target_char_couter = 0;
        for ch in password.chars() {
            if ch == target_char {
                target_char_couter += 1;
            }
        }
        if target_char_couter < start_val || target_char_couter > end_val {
            return false;
        };
        return true;
    });
}

pub fn day2b() -> usize {
    return shared_logic(|start_val, end_val, target_char, password| {
        // println!("start, end, pass : {:?}, {:?},{:?}", start_val, end_val, password);
        let first_char = password.chars().nth((start_val - 1) as usize);
        let second_char = password.chars().nth((end_val - 1) as usize);
        let is_first_eqal = if first_char.unwrap() == target_char { 1 } else { 0 };
        let is_second_equal = if second_char.unwrap() == target_char { 1 } else { 0 };
        return is_first_eqal + is_second_equal == 1;
    });
}

fn shared_logic(is_valid: fn(i32, i32, char, &str) -> bool) -> usize {
    // File hosts must exist in current path before this produces output
    let lines = read_lines("src/day2_input.txt").unwrap();
    // Consumes the iterator, returns an (Optional) String
    return lines.map(|l| {
        let res = l.unwrap();
        // println!("res: {:?}", res);
        let dash_split = res.split("-").collect::<Vec<&str>>();
        let start_val = dash_split[0].parse::<i32>().unwrap();
        // println!("dash_split: {:?}", dash_split);
        let space_split_struct = dash_split[1].split(" ");
        let mut space_split = space_split_struct.into_iter();
        let end_val = space_split.next().unwrap().parse::<i32>().unwrap();
        let target_char = space_split.next().unwrap().chars().nth(0).unwrap();
        let password = space_split.next().unwrap();
        return is_valid(start_val, end_val, target_char, password);
    }).filter(|x| *x == true).count();
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
