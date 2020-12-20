use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, Error};

pub e
pub fn day3b() -> Result<i32, String> {
    let lines = read_lines("src/day3_input.txt").unwrap();
    let width_ref = &(lines.first().unwrap())[..].chars().count();
    let width = width_ref.clone();
    let height = lines.len();
    println!("width: {:?}", width);
    let lines_ref : Vec<Vec<char>> = lines.iter().map(|s| &**s).map(|x| x.chars().collect()).collect();
    let
    let  = helper(width, height, lines_ref, 3, 1)?;
    return Ok(res);
}

pub fn day3a() -> Result<i32, String> {
    let lines = read_lines("src/day3_input.txt").unwrap();
    let width_ref = &(lines.first().unwrap())[..].chars().count();
    let width = width_ref.clone();
    let height = lines.len();
    println!("width: {:?}", width);
    let lines_ref : Vec<Vec<char>> = lines.iter().map(|s| &**s).map(|x| x.chars().collect()).collect();
    let res = helper(width, height, lines_ref, 3, 1)?;
    return Ok(res);
}

pub fn helper(width: usize, height: usize, lines_ref: Vec<Vec<char>>, mut x_pos: usize, mut y_pos: usize) -> Result<i32, String> {
    let mut answer = if lines_ref[y_pos][x_pos as usize] =='#' { 1 } else { 0 };
    let mut i = 0;
    while y_pos + 1  < height {
        x_pos += 3;
        y_pos += 1;
        x_pos = ((x_pos % width) + width) % width;
        let val = lines_ref[y_pos][x_pos as usize];
        println!("i: {:?}, x {:?}, y{:?}, val {:?}", i, x_pos, y_pos, val);
        println!("thing {:?}", lines_ref[y_pos]);
        if val == '#' {
            answer += 1;
        }
        i+=1;
    }

    Ok(answer)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Vec<String>, Error>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<_>>())
}
