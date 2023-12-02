use std::fs;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut result = 0;
    // each line is 1 game
    for line in puzzle.split("\n") {
        if line.trim() == "" { return };

        let id :i32 = get_game_id(line);
        let sets = get_game_sets(line);

        let mut min_cubes_array : [(i32, &str); 3] = [
            (0, "red"),
            (0, "green"),
            (0, "blue")
        ];
        for set in sets {
            for cubes in set {
                let index = min_cubes_array.iter()
                    .position(|&t| t.1 == cubes.1).unwrap();
                if min_cubes_array[index].0 < cubes.0 {
                    min_cubes_array[index].0 = cubes.0;
                }
            }
        }
        let mut power_of_set :i32 = 1;
        for &(min_cubes, _) in min_cubes_array.iter() {
            power_of_set = power_of_set * min_cubes;
        }
        result += power_of_set;


        println!("result {}", result);
    }

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
