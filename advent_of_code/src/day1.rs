use std::collections::HashSet;
use std::fs;

pub fn day1a() -> Result<i32, String> {
    let values = get_input();
    for val in values.iter(){
        let num_to_seek = 2020i32 - val;
        if values.contains(&num_to_seek){
            return Ok(val * num_to_seek);
        }
    }

    Err("ahh terrible".to_string())
}

pub fn day1b() -> Result<i32, String> {
    let values = get_input();
    for val1 in values.iter(){
        for val2 in values.iter().filter(|x| *x != val1){
            for val3 in values.iter().filter(|x| *x != val2 && *x != val1) {
                if val1 + val2 + val3 == 2020 {
                    return Ok(val1 * val2 * val3);
                }
            }
        }
    }

    Err("ahh terrible".to_string())
}


pub fn get_input() -> HashSet<i32> {

    let contents = fs::read_to_string("src/aoc_1_input.txt")
        .expect("Something went wrong reading the file");

    let result_strings  = contents.split_whitespace();
    let result_ints = result_strings.map(|x| x.parse::<i32>().unwrap()).collect::<HashSet<i32>>();
    // println!("reuslt_ints : {:?}", result_ints);
    return result_ints;
}
