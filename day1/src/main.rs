use std::fs;

mod helpers;
use helpers::find_spelled_number;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut result :i32 = 0; 

    // let lines: Vec<&str> = puzzle.split("\n").collect(); 
    for line in puzzle.split("\n") {
        let mut first_val: Option<(&str, usize)> = find_spelled_number(line, false);
        let mut last_val: Option<(&str, usize)> = find_spelled_number(line, true);

        let find_first_until = if first_val.is_none() {line.len()} else {first_val.unwrap().1};
        for i in 0..find_first_until {
            let c : char = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                first_val = Some((
                    &line[i..=i], i
                ));
                break;
            }
        }
        let find_last_until = if last_val.is_none() {0} else {last_val.unwrap().1};
        for i in (find_last_until..line.len()).rev() {
            let c : char = line.chars().nth(i).unwrap();
            if c.is_numeric() {
                last_val = Some((
                    &line[i..=i], i
                ));
                break;
            }
        }
        
        if first_val.is_none() || last_val.is_none() {
            continue;
        }

        let cal_val : String = format!("{}{}", first_val.unwrap().0, last_val.unwrap().0);
        let parsed :i32 = cal_val.parse().unwrap();
        result += parsed;
    }

    println!("{}", result);
}

