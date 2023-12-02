use std::fs;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut result = 0;
    // each line is 1 game
    for line in puzzle.split("\n") {
        if line.trim() == "" { return };

        let mut valid = true;

        let id :i32 = get_game_id(line);
        let sets = get_game_sets(line);

        for set in sets {
            for cubes in set {
                if !validate_cubes(cubes) {
                    valid = false;
                    break;
                }
            }

            if !valid {
                break;
            }
        }

        if valid {
            println!("Game {} is valid!", id);
            result += id;
            println!("Result: {}", result);
        }
    }

}

fn validate_cubes(cubes: (i32, String)) -> bool {
    let bag : [(&i32, &str); 3] = [
        (&12, "red"),
        (&13, "green"),
        (&14, "blue")
    ];

    if let Some(cubes_in_bag) = bag.iter()
        .find(|&&(_, color)| color == cubes.1) {
        if cubes_in_bag.0 >= &cubes.0 {
            return true 
        }
    }
    return false; 
}

fn get_game_sets(line: &str) -> Vec<Vec<(i32, String)>> {
    let mut result = Vec::new();
    let set_list :Vec<&str> = line.split(":").collect::<Vec<_>>()[1]
        .split(";").collect();

    for set in set_list {
        let mut new_set = Vec::new();
        let cube_list :Vec<&str> = set.split(",").collect();

        for cube in cube_list {
            let cube_split :Vec<&str> = cube.trim().split(" ").collect();
            let cube_amount: i32 = cube_split[0].parse().unwrap();
            let cube_color: String = cube_split[1].parse().unwrap();

            new_set.push((
                cube_amount, 
                cube_color
            ));
        }

        result.push(new_set);
    }

    return result;
}

fn get_game_id(line: &str) -> i32 {
    let _id = line.split(":").collect::<Vec<_>>()[0]
        .split(" ").collect::<Vec<_>>()[1];

    return _id.parse().unwrap();
}
