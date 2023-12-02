use std::fs;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut result :i32 = 0; 

    // let lines: Vec<&str> = puzzle.split("\n").collect(); 
    for line in puzzle.split("\n") {
        let mut last_val: String = String::new(); // last integer found in line
        let mut cal_val: String = String::new(); // current calibration value

        for c in line.chars() {
            if c.is_numeric() {
                last_val = c.to_string(); 
                if cal_val == "" {
                    cal_val = last_val.clone(); 
                }
            }
        }

        cal_val.push_str(&last_val);
        if !cal_val.is_empty() {
            let parsed :i32 = cal_val.parse().unwrap();
            result += parsed;
        }

    }

    println!("{}", result);
}

