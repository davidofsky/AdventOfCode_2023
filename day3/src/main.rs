use std::fs;

fn main() {
    let puzzle: String = fs::read_to_string("./puzzle")
        .expect("The file named 'puzzle' should be in the current working directory!");

    let mut numbers :Vec<Number> = Vec::new();
    let mut symbols :Vec<Symbol> = Vec::new();

    let lines :Vec<&str> = puzzle.split("\n").collect();
    for y in 0..lines.len() {
        if lines[y].trim() == "" { continue };

        let chars :Vec<char> = lines[y].chars().collect();

        let mut current_number = Number {
            value: 0,
            coordinates: Vec::new()
        };

        for x in 0..chars.len() {
            if chars[x].is_numeric() {
                current_number.value = format!("{}{}", current_number.value, chars[x].to_digit(10).unwrap()).parse().unwrap();
                current_number.coordinates.push(Coordinate {x,y});
            } else {
                if current_number.coordinates.len() > 0 {
                    add_number_to_vec(&mut current_number, &mut numbers);
                }

                if chars[x] != '.' {
                    symbols.push(Symbol {
                        value: chars[x], 
                        adjecent_numbers: 0,
                        gear_ratio: 1,
                        coordinate: Coordinate {x,y} 
                    });
                }
            }
        }
        if current_number.coordinates.len() > 0 {
            add_number_to_vec(&mut current_number, &mut numbers);
        }
    }

    let mut result = 0;
    let mut part_number_count = 0;
    for number in &numbers {
        println!("");
        println!("Number {}", number.value);
        println!("Coordinates: ");

        let mut adjecent_symbol :char = '.';

        for coordinate in &number.coordinates {
            println!("  x: {}, y: {}", coordinate.x, coordinate.y);
            for symbol in symbols.iter_mut() {
                if euclidian_distance(&symbol.coordinate, &coordinate) < 2.0 {
                    adjecent_symbol = symbol.value;
                    symbol.adjecent_numbers += 1;
                    symbol.gear_ratio = symbol.gear_ratio*number.value;
                    break;
                }
            }
            if adjecent_symbol != '.' {
                result += number.value;
                part_number_count+=1;
                println!("Has adjecent symbol: {}", adjecent_symbol);
                adjecent_symbol = '.';
                break;
            }
        }
    }

    println!("There are {} symbols", symbols.len());
    println!("There are {} numbers", numbers.len());
    println!("There are {} part-numbers", part_number_count);


    println!("The sum of all the part-numbers is: {}", result);
    println!("");

    // Part 2

    let gears :Vec<&Symbol> = symbols.iter().filter(|&s|s.adjecent_numbers==2).collect();
    println!("There are {} gears", gears.len());

    let mut gear_ratio_sum = 0;
    for gear in gears {
        gear_ratio_sum += gear.gear_ratio;
    }

    println!("The sum of all the gear ratios is: {}", gear_ratio_sum);
}

fn euclidian_distance(coord1: &Coordinate, coord2: &Coordinate) -> f64 {
    let x_squared = (coord2.x as f64 - coord1.x as f64).powi(2);
    let y_squared = (coord2.y as f64 - coord1.y as f64).powi(2);
    (x_squared + y_squared).sqrt()
}


fn add_number_to_vec(new_number: &mut Number, number_vec: &mut Vec<Number>) {
    number_vec.push(new_number.clone());
    new_number.value = 0;
    new_number.coordinates = Vec::new();
}

// A number has a value and a set of coordinates
// These coordinates can be used to easily check if it is near a symbol
#[derive(Clone)]
struct Number {
    value: u32,
    coordinates: Vec<Coordinate>
}

struct Symbol {
    value: char,
    adjecent_numbers: u32,
    gear_ratio: u32,
    coordinate: Coordinate
}

#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize
}
